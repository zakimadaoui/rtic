#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[idle]
    fn idle(_: idle::Context) -> u32 {
        0
    }
}
