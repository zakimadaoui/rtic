#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[idle(shared = [A], shared = [B])]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }
}
