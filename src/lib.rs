#[warn(missing_docs)]
/// Time functions
pub mod clock;
#[cfg(target_family = "unix")]
mod linux;
/// Memory functions
pub mod mman;
/// Scheduling functions
pub mod sched;
#[cfg(target_os = "windows")]
mod windows;
