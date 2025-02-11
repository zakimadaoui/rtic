#![no_main]

#[rtic_macros_ui_tests::mock_app(device = mock)]
mod app {
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    pub fn init(_: init::Context) -> (Shared, Local) {}
}
