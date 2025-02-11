#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[task(binds = UART0)]
    fn foo(_: foo::Context) {}

    #[task(binds = UART0)]
    fn bar(_: bar::Context) {}
}
