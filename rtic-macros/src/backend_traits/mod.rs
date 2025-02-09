//! Definitions of backend traits

use analyze::Analysis;
use proc_macro2::TokenStream as TokenStream2;
use syntax::analyze::Analysis as SyntaxAnalysis;
use syntax::ast::App;

use super::*;

pub trait RticBackendBase {
    #[allow(unused)]
    fn enable_interrupts(&self, app: &App) -> TokenStream2;
    #[allow(unused)]
    fn disable_interrupts(&self, app: &App) -> TokenStream2;
    #[allow(unused)]
    fn pend_interrupt(&self, app: &App, handler: syn::Ident) -> TokenStream2;
    #[allow(unused)]
    fn unpend_interrupt(&self, app: &App, handler: syn::Ident) -> TokenStream2;

    /// # Implementation specific pre-computed values and global definitions
    /// When the Implementation requires pre-computed constants, additional global `use' statements or additional function definitions that must to be accessible from the global application scope, the above trait method could be implemented to return the TokenStream representing those global definitions.
    fn generate_global_definitions(&self, app: &App, analysis: &Analysis) -> Option<TokenStream2>;

    /// # Additional user code validation
    /// Implement this method to validate/analyze the resulting parsed and analyzed user application before the code generation phase starts.
    ///
    /// ## Use case
    /// In certain cases, some checks/validation related to implementation/hardware specific details need to be made before allowing the user code to be expanded.
    /// An example, could be that the user has attempted to use an Exception line as for a dispatcher, but the distribution needs to forbid that.
    /// Implementing this trait method, gives the ability to enforcing such checks.
    fn pre_codgen_validation(&self, app: &App, analysis: &SyntaxAnalysis) -> syn::Result<()>; // TODO: replaces `architecture_specific_analysis`

    /// Modify app based on backend before continuing
    fn pre_codegen_processing(&self, app: &mut App, analysis: &SyntaxAnalysis) -> syn::Result<()>; // TODO: replaces `pre_init_preprocessing`, also need to find a better name for this...

    /// Define additional compile-time assertions related to this pass in case the platform needs it.
    fn extra_assertions(&self, app: &App, analysis: &SyntaxAnalysis) -> Vec<TokenStream2>;

    /// Path to the interrupt type
    fn interrupt_path(&self, app: &App) -> syn::Path;

    /// # Wrapping task execution
    /// In certain cases, some code needs to be executed before and after the [exec] method of a task is called within an interrupt handler. To allow this "wrapping" of the task execution, this trait method can be implemented. The statement from calling the task's [exec] method has been provided as input to this trait method (`dispatch_task_call`), If you return a Some(tokenstream), the returned TokenStream must include/wrap the `dispatch_task_call`.
    ///
    /// ## Example use case
    /// An example could be an implementation for cortex M MCUs that support a BASEPRI register. Every time an interrupt handler id called, the current value of BASEPRI needs to be saved, then task [exec] method is called, then the saved BASEPRI value is restored.
    ///
    /// ## Arguments
    /// - `dispatch_task_call`: call to the task [exec] method.
    ///
    /// ## Contract
    /// The `dispatch_task_call` token stream must be placed in between your custom logic. This tokenstream must not be mutated.
    // fn wrap_task_execution(
    //     &self,
    //     task_prio: u16, // TODO: more information needs to be provided here to cover more complex cases
    //     dispatch_task_call: TokenStream2,
    // ) -> Option<TokenStream2>;
    fn interrupt_entry_statements(
        &self,
        app: &App,
        analysis: &Analysis,
        handler: Option<syn::Ident>,
    ) -> Option<TokenStream2>;
    fn interrupt_exit_statements(
        &self,
        app: &App,
        analysis: &Analysis,
        handler: Option<syn::Ident>,
    ) -> Option<TokenStream2>;

    /// Return attributes to be inserted on top of interrupt handler definitions
    fn interrupt_handler_config(
        &self,
        app: &App,
        analysis: &Analysis,
        dispatcher_name: syn::Ident,
    ) -> Vec<syn::Attribute>;
}

//TODO/FIXME: most of the comments here come from the MMRTIC project, they need to be updated to reflect the current RTIC 2.x implementation

/// Interface for providing the low-level hardware bindings specific for a target (A.k.a The Backend) to be used during the preprocessing and code generation phases
/// of the **Core Compilation Pass*.
pub trait CorePassBackend: RticBackendBase {
    /// # Setting up the system, Part I
    /// Implementation must return the TokenStream to be inserted **BEFORE** the call to Global `#[init]` and tasks init() functions.
    ///
    /// Note that the generated code resulting from the returned TokenStream will be wrapped in a critical section (interrupts
    /// are disabled at start and re-enabled at end)
    ///
    /// ## Use case
    /// This trait method is meant to cover the following use cases:
    /// - enabling interrupt lines used by the application
    /// - setting priority of interrupts, and similar initializations depending on specific hardware details
    /// - multicore systems where a master core needs to wake-up and initialize other cores (see rp2040 distribution as an example)
    /// ## Note
    /// This function will be called several times in case of a multicore system, each time with different `app_info` and `app_analysis`.
    fn pre_init(&self, app: &App, analysis: &Analysis) -> Option<TokenStream2>;

    /// # Setting up the system, Part II
    /// Implementation must return the TokenStream to be inserted **AFTER** the call to Global `#[init]` and tasks init() functions,
    /// and **BEFORE** starting the idle task.
    ///
    /// Note that the generated code resulting from the returned TokenStream will be wrapped in a critical section (interrupts
    /// are disabled at start and re-enabled at end)
    ///
    /// ## Use case
    /// This trait method is meant to cover the following use cases:
    /// - enabling interrupt lines used by the application
    /// - setting priority of interrupts, and similar initializations depending on specific hardware details
    /// - multicore systems where a master core needs to wake-up and initialize other cores (see rp2040 distribution as an example)
    /// ## Note
    /// This function will be called several times in case of a multicore system, each time with different `app_info` and `app_analysis`.
    fn post_init(&self, app: &App, analysis: &Analysis) -> Option<TokenStream2>;

    /// # SRP-based Resource locking implementation
    ///
    /// The provided method argument `incomplete_lock_fn` holds the TokenStream representation of an incomplete function named `lock` responsible for locking a distinct resource in the system. The distribution must generate the missing target-specific logic for implementing the locking of that resource.
    ///
    /// To illustrate this further with an example, let's assume the user defined the following shared resources:
    ///
    /// ```rust
    /// // Before code expansion
    /// #[shared]
    /// struct Shared {
    ///     pub resource1: R1Type
    /// }
    /// ```
    ///
    /// Every field of the shared resources struct has a corresponding autogenerated **resource proxy** struct that implements the `RticMutex` internal trait. as follows:
    ///
    /// ```rust
    /// struct __resource1_mutex {
    ///     #[doc(hidden)]
    ///     task_priority: u16,
    /// }
    /// impl RticMutex for __resource1_mutex {
    ///     type ResourceType = R1Type;
    ///     // this is what the trait method argument `incomplete_lock_fn` expands to
    ///     fn lock(&mut self, f: impl FnOnce(&mut Self::ResourceType)) {
    ///         const CEILING: u16 = 3u16; // resource ceiling
    ///         let task_priority = self.task_priority; // current task priority
    ///         let resource_ref = unsafe { &mut SHARED.assume_init_mut().resource1 };
    ///         /* TODO: THE HARDWARE-SPECIFIC CODE COMES HERE */
    ///     }
    /// }
    /// ```
    ///
    /// Each time RTIC needs to generate an implementation of the `RticMutex` trait for a **resource proxy**, it calls the method [CorePassBackend::generate_resource_proxy_lock_impl] to ask the backend to populate the missing details of the [lock] function for that particular resource.
    ///
    /// ## Contract
    ///
    ///* The returned value representing the populated function must have the same signature as `incomplete_lock_fn'.
    ///     
    ///* The implementation must be according to SRP rules such that:
    ///    * System interrupt priority ceiling is raised to the value of `CEILING`.
    ///    * The closure `f` is called and `resource_ref` is passed to it as a parameter. (to execute the resource critical section).
    ///    * System interrupt priority ceiling should be restored back to `task_priority` value.     
    ///* If global definitions need to be generated for use in the locking implementation, the trait method which will be described next should be used to cover such need.
    ///
    /// ## Note
    /// This trait method is called for every shared resource in every sub-application.
    ///
    /// ## Debugging Tip
    /// Use ```eprintln("{}", incomplete_lock_fn.to_tokenstream().to_string())``` to see the `incomplete_lock_fn` signature and already provided logic inside it.
    // fn generate_resource_proxy_lock_impl(
    //     &self,
    //     app: &App,
    //     incomplete_lock_fn: syn::ImplItemFn,
    // ) -> syn::ImplItemFn;

    /// TODO: Tentative backend trait function for Generateing a `Mutex` implementation
    /// In next commits/iterations some efforts are needed to replace this with `generate_resource_proxy_lock_impl` method like experimented with in the MMRTIC project
    fn impl_mutex(
        &self,
        app: &App,
        analysis: &Analysis,
        cfgs: &[syn::Attribute],
        resources_prefix: bool,
        name: &syn::Ident,
        ty: &TokenStream2,
        ceiling: u8,
        ptr: &TokenStream2,
    ) -> TokenStream2;
}

/// Interface for providing the hardware specific details (i.e backend) needed by the software pass
pub trait SwPassBackend: RticBackendBase {
    /// Returns a Macro that defines the maximum priority level for async tasks.
    fn async_prio_limit(&self, app: &App, analysis: &Analysis) -> Vec<TokenStream2>;

    /// Generate code that will be run before system initialization (init()) which checks for stack overflow
    fn check_stack_overflow_before_init(
        &self,
        app: &App,
        analysis: &SyntaxAnalysis,
    ) -> TokenStream2;
}
