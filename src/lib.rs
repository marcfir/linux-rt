#[warn(missing_docs)]
/// Time functions
pub mod clock;
/// Memory functions
pub mod mman;
/// Scheduling functions
pub mod sched;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;
