#![no_std]
pub mod exports; // NOTE: This module must exist in all distributions, must export all the necessary dependencies specified by the compilation passes and the backend implementations
pub use rtic_app_macro::app;
