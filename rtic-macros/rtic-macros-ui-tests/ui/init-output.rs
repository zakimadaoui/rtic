#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[init]
    fn init(_: init::Context) -> u32 {
        0
    }
}
