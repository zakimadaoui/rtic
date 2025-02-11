#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[task]
    async unsafe fn foo(_: foo::Context) {}
}
