#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[task(local = [a: u32])]
    async fn foo(_: foo::Context) {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {}
}
