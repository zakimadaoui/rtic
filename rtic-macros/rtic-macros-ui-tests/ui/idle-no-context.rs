#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[idle]
    fn idle() -> ! {
        loop {}
    }
}
