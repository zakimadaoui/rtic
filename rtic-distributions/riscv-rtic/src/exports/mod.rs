pub mod cortex_common;
pub use cortex_common::*;

// Cortex-M target with basepri support
#[cfg(feature = "cortex-m-basepri")]
mod cortex_basepri;
#[cfg(feature = "cortex-m-basepri")]
pub use cortex_basepri::*;

// Cortex-M target with source mask support
#[cfg(feature = "cortex-m-source-masking")]
mod cortex_source_mask;
#[cfg(feature = "cortex-m-source-masking")]
pub use cortex_source_mask::*;
