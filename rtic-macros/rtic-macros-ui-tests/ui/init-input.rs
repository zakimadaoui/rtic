#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context, _undef: u32) -> (Shared, Local) {}
}
