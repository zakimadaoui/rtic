#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[init(shared = [A], shared = [B])]
    fn init(_: init::Context) {}
}
