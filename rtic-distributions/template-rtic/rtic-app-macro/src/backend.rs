use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use rtic_macros::{ast::App, codegen_utils as util, Analysis as CodegenAnalysis, SyntaxAnalysis};
use std::collections::HashSet;
use syn::{parse, parse_quote, Ident};

pub struct CortexCorePass;
pub struct CortexSwPass;

impl rtic_macros::RticBackendBase for CortexCorePass {
    fn interrupt_path(&self, device: syn::Path) -> syn::Path {
        let interrupt = interrupt_ident();
        parse_quote!(#device::#interrupt)
    }
}
impl rtic_macros::RticBackendBase for CortexSwPass {
    fn interrupt_path(&self, device: syn::Path) -> syn::Path {
        let interrupt = interrupt_ident();
        parse_quote!(#device::#interrupt)
    }
}

impl rtic_macros::HwPassBackend for CortexCorePass {
    fn pre_init(&self, app: &App, analysis: &CodegenAnalysis) -> Vec<TokenStream2> {
        vec![]
    }

    fn post_init(&self, _app: &App, _analysis: &CodegenAnalysis) -> Vec<TokenStream2> {
        vec![]
    }

    fn impl_mutex(
        &self,
        _app: &App,
        _analysis: &CodegenAnalysis,
        _cfgs: &[syn::Attribute],
        _resources_prefix: bool,
        _name: &syn::Ident,
        _ty: &TokenStream2,
        _ceiling: u8,
        _ptr: &TokenStream2,
    ) -> TokenStream2 {
        quote!()
    }

    fn interrupt_entry_statements(
        &self,
        _app: &App,
        _analysis: &CodegenAnalysis,
        _handler: Option<syn::Ident>,
    ) -> Option<TokenStream2> {
        None
    }

    fn interrupt_exit_statements(
        &self,
        _app: &App,
        _analysis: &CodegenAnalysis,
        _handler: Option<syn::Ident>,
    ) -> Option<TokenStream2> {
        None
    }

    fn interrupt_handler_config(
        &self,
        _app: &App,
        _analysis: &CodegenAnalysis,
        _dispatcher_name: syn::Ident,
    ) -> Vec<syn::Attribute> {
        vec![]
    }

    fn pre_codgen_validation(&self, app: &App, analysis: &SyntaxAnalysis) -> syn::Result<()> {
        Ok(())
    }

    fn pre_codegen_processing(
        &self,
        _app: &mut App,
        _analysis: &SyntaxAnalysis,
    ) -> syn::Result<()> {
        Ok(())
    }

    fn generate_global_definitions(
        &self,
        _app: &App,
        _analysis: &CodegenAnalysis,
    ) -> Option<TokenStream2> {
        None
    }

    fn extra_assertions(&self, _app: &App, _analysis: &SyntaxAnalysis) -> Vec<TokenStream2> {
        vec![]
    }
}

impl rtic_macros::SwPassBackend for CortexSwPass {
    fn async_prio_limit(&self, app: &App, analysis: &CodegenAnalysis) -> Vec<TokenStream2> {
        vec![]
    }

    fn check_stack_overflow_before_init(
        &self,
        app: &App,
        analysis: &CodegenAnalysis,
    ) -> Vec<TokenStream2> {
        vec![]
    }

    fn generate_global_definitions(
        &self,
        _app: &App,
        _analysis: &CodegenAnalysis,
    ) -> Option<TokenStream2> {
        None
    }

    fn pre_codgen_validation(&self, _app: &App, _analysis: &SyntaxAnalysis) -> syn::Result<()> {
        Ok(())
    }

    fn pre_codegen_processing(
        &self,
        _app: &mut App,
        _analysis: &SyntaxAnalysis,
    ) -> syn::Result<()> {
        Ok(())
    }

    fn extra_assertions(&self, _app: &App, _analysis: &SyntaxAnalysis) -> Vec<TokenStream2> {
        vec![]
    }
}

pub fn interrupt_ident() -> Ident {
    let span = Span::call_site();
    Ident::new("interrupt", span)
}
