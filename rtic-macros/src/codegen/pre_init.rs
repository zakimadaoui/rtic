use crate::analyze::Analysis;
use crate::syntax::ast::App;
use crate::BackendBindings;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Generates code that runs before `#[init]`
pub fn codegen(app: &App, analysis: &Analysis, bindings: &BackendBindings) -> Vec<TokenStream2> {
    let mut stmts = vec![];

    // Disable interrupts -- `init` must run with interrupts disabled
    // TODO: replace this with a backend method as we don't want dependency on hardware even on rtic::export
    stmts.push(quote!(rtic::export::interrupt::disable();));

    if app.args.core {
        stmts.push(quote!(
            // To set the variable in cortex_m so the peripherals cannot be taken multiple times
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal().into();
        ));
    }

    stmts.extend(bindings.core.pre_init(app, analysis));

    stmts
}
