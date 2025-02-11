#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[task(priority = 256)]
    async fn foo(_: foo::Context) {}
}
