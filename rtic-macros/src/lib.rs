#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rtic-rs/rtic/master/book/en/src/RTIC.svg",
    html_favicon_url = "https://raw.githubusercontent.com/rtic-rs/rtic/master/book/en/src/RTIC.svg"
)]

mod analyze;
mod backend_traits;
mod codegen;
mod syntax;

use backend_traits::{CorePassBackend, SwPassBackend};
use proc_macro2::TokenStream;
use std::{env, fs, path::Path};

// TODO LIST
// change all instances of rtic::export too !

pub struct BackendBindings {
    core: Box<dyn CorePassBackend>,
    sw: Box<dyn SwPassBackend>,
}

impl BackendBindings {
    pub fn new(core: impl CorePassBackend + 'static, sw: impl SwPassBackend + 'static) -> Self {
        Self {
            core: Box::new(core),
            sw: Box::new(sw),
        }
    }
}

/// Used for mocking the API in testing
pub fn run_mock_app(args: TokenStream, input: TokenStream) -> TokenStream {
    if let Err(e) = syntax::parse2(args, input) {
        e.to_compile_error()
    } else {
        "fn main() {}".parse().unwrap()
    }
}

/// Attribute used to declare a RTIC application
///
/// For user documentation see the [RTIC book](https://rtic.rs)
///
/// # Panics
///
/// Should never panic, cargo feeds a path which is later converted to a string
pub fn run_macro(args: TokenStream, input: TokenStream, bindings: BackendBindings) -> TokenStream {
    let (mut app, analysis) = match syntax::parse2(args, input) {
        Err(e) => return e.to_compile_error(),
        Ok(x) => x,
    };

    // Modify app based on backend before continuing
    let res = bindings
        .core
        .pre_codegen_processing(&mut app, &analysis)
        .and_then(|_| bindings.sw.pre_codegen_processing(&mut app, &analysis));
    if let Err(e) = res {
        return e.to_compile_error();
    }
    let app = app;
    // App is not mutable after this point

    // Pre-codegen validation
    let res = bindings
        .core
        .pre_codgen_validation(&app, &analysis)
        .and_then(|_| bindings.sw.pre_codgen_validation(&app, &analysis));
    if let Err(e) = res {
        return e.to_compile_error();
    }

    let analysis = analyze::app(analysis, &app);

    let ts = codegen::app(&app, &analysis, &bindings);

    // Default output path: <project_dir>/target/
    let mut out_dir = Path::new("target");

    // Get output directory from Cargo environment
    // TODO don't want to break builds if OUT_DIR is not set, is this ever the case?
    let out_str = env::var("OUT_DIR").unwrap_or_else(|_| "".to_string());

    if !out_dir.exists() {
        // Set out_dir to OUT_DIR
        out_dir = Path::new(&out_str);

        // Default build path, annotated below:
        // $(pwd)/target/thumbv7em-none-eabihf/debug/build/rtic-<HASH>/out/
        // <project_dir>/<target-dir>/<TARGET>/debug/build/rtic-<HASH>/out/
        //
        // traverse up to first occurrence of TARGET, approximated with starts_with("thumbv")
        // and use the parent() of this path
        //
        // If no "target" directory is found, <project_dir>/<out_dir_root> is used
        for path in out_dir.ancestors() {
            if let Some(dir) = path.components().last() {
                let dir = dir.as_os_str().to_str().unwrap();

                if dir.starts_with("thumbv") || dir.starts_with("riscv") {
                    if let Some(out) = path.parent() {
                        out_dir = out;
                        break;
                    }
                    // If no parent, just use it
                    out_dir = path;
                    break;
                }
            }
        }
    }

    // Try to write the expanded code to disk
    if let Some(out_str) = out_dir.to_str() {
        fs::write(format!("{out_str}/rtic-expansion.rs"), ts.to_string()).ok();
    }

    ts
}
