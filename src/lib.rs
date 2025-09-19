#[warn(missing_docs)]
/// Time functions
pub mod clock;
mod lowlevel;
/// Scheduling functions
pub mod sched;
pub use lowlevel::clock::TimeSpec;
pub use lowlevel::sched::CpuSet;
