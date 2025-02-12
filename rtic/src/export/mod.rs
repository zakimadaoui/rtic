pub use bare_metal::CriticalSection;
pub use portable_atomic as atomic;

// Cortex-M target (any)
#[cfg(feature = "cortex-m")]
pub use cortex_m_rtic::exports::*;

#[cfg(feature = "riscv")]
pub use riscv_rtic::exports::*;

pub mod executor;

#[inline(always)]
pub fn assert_send<T: Send>() {}

#[inline(always)]
pub fn assert_sync<T: Sync>() {}
