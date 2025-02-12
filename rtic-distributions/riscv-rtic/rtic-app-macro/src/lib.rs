use backend::{CortexCorePass, CortexSwPass};
use proc_macro::TokenStream;
use rtic_macros::BackendBindings;
mod backend;

#[proc_macro_attribute]
pub fn app(args: TokenStream, input: TokenStream) -> TokenStream {
    rtic_macros::run_macro(
        args.into(),
        input.into(),
        BackendBindings::new(CortexCorePass, CortexSwPass),
    )
    .into()
}
