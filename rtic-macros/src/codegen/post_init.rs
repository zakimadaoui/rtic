use crate::{analyze::Analysis, codegen::util, syntax::ast::App, BackendBindings};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Generates code that runs after `#[init]` returns
pub fn codegen(app: &App, analysis: &Analysis, bindings: &BackendBindings) -> Vec<TokenStream2> {
    let mut stmts = vec![];

    // Initialize shared resources
    for (name, res) in &app.shared_resources {
        let mangled_name = util::static_shared_resource_ident(name);
        // If it's live
        let cfgs = res.cfgs.clone();
        if analysis.shared_resources.get(name).is_some() {
            stmts.push(quote!(
                // We include the cfgs
                #(#cfgs)*
                // Resource is a RacyCell<MaybeUninit<T>>
                // - `get_mut` to obtain a raw pointer to `MaybeUninit<T>`
                // - `write` the defined value for the late resource T
                #mangled_name.get_mut().write(core::mem::MaybeUninit::new(shared_resources.#name));
            ));
        }
    }

    // Initialize local resources
    for (name, res) in &app.local_resources {
        let mangled_name = util::static_local_resource_ident(name);
        // If it's live
        let cfgs = res.cfgs.clone();
        if analysis.local_resources.get(name).is_some() {
            stmts.push(quote!(
                // We include the cfgs
                #(#cfgs)*
                // Resource is a RacyCell<MaybeUninit<T>>
                // - `get_mut` to obtain a raw pointer to `MaybeUninit<T>`
                // - `write` the defined value for the late resource T
                #mangled_name.get_mut().write(core::mem::MaybeUninit::new(local_resources.#name));
            ));
        }
    }

    stmts.extend(bindings.core.post_init(app, analysis));

    // TODO: replace this with a binding for enabling interrupts
    // Enable the interrupts -- this completes the `init`-ialization phase
    // or ask this to be part of `post_init` binding
    stmts.push(quote!(rtic::export::interrupt::enable();));

    stmts
}
