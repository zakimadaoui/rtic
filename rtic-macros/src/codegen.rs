use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::analyze::Analysis;
use crate::syntax::ast::App;
use crate::BackendBindings;

mod assertions;
mod async_dispatchers;
mod hardware_tasks;
mod idle;
mod init;
mod local_resources;
mod local_resources_struct;
mod module;
mod post_init;
mod pre_init;
mod shared_resources;
mod shared_resources_struct;
mod software_tasks;
pub mod util;

mod main;

// TODO: organize codegen to actual parts of code
// so `main::codegen` generates ALL the code for `fn main`,
// `software_tasks::codegen` generates ALL the code for software tasks etc...

#[allow(clippy::too_many_lines)]
pub fn app(app: &App, analysis: &Analysis, bindings: &BackendBindings) -> TokenStream2 {
    // Generate the `main` function
    let main = main::codegen(app, analysis, bindings);
    let init_codegen = init::codegen(app, analysis, bindings);
    let idle_codegen = idle::codegen(app, analysis, bindings);
    let shared_resources_codegen = shared_resources::codegen(app, analysis, bindings);
    let local_resources_codegen = local_resources::codegen(app, analysis);
    let hardware_tasks_codegen = hardware_tasks::codegen(app, analysis, bindings);
    let software_tasks_codegen = software_tasks::codegen(app, analysis, bindings);
    let async_dispatchers_codegen = async_dispatchers::codegen(app, analysis, bindings);

    let user_imports = &app.user_imports;
    let user_code = &app.user_code;
    let name = &app.name;
    let device = &app.args.device;

    let rt_err = util::rt_err_ident();
    let async_limit = bindings.sw.async_prio_limit(app, analysis);

    quote!(
        /// The RTIC application module
        pub mod #name {
            /// Always include the device crate which contains the vector table
            use #device as #rt_err;

            #(#async_limit)*

            #(#user_imports)*

            #(#user_code)*
            /// User code end

            #init_codegen

            #idle_codegen

            #hardware_tasks_codegen

            #software_tasks_codegen

            #shared_resources_codegen

            #local_resources_codegen

            #async_dispatchers_codegen

            #main
        }
    )
}
