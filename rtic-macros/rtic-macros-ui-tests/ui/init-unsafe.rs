#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[init]
    unsafe fn init(_: init::Context) -> (Shared, Local) {}
}
