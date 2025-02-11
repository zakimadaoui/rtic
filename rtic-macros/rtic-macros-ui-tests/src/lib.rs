use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn mock_app(args: TokenStream, input: TokenStream) -> TokenStream {
    rtic_macros::run_mock_app(args.into(), input.into()).into()
}
