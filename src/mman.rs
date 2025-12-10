use std::ffi::c_void;

use bitflags::bitflags;
use syscalls::Errno;

use crate::lowlevel;

bitflags! {
    /// These flags control the scheduling behavior
    #[derive(Debug, Clone, PartialEq)]
    pub struct MmanFlags: std::ffi::c_int {
        /// Lock all pages which are currently mapped into the address
        /// space of the process.
        const MCL_CURRENT = lowlevel::mman::flags::MCL_CURRENT;
        /// Lock all pages which will become mapped into the address
        /// space of the process in the future.  These could be, for
        /// instance, new pages required by a growing heap and stack as
        /// well as new memory-mapped files or shared memory regions.
        const MCL_FUTURE = lowlevel::mman::flags::MCL_FUTURE;
        /// Used together with MCL_CURRENT, MCL_FUTURE, or both.  Mark
        /// all current (with MCL_CURRENT) or future (with MCL_FUTURE)
        /// mappings to lock pages when they are faulted in.  When used
        /// with MCL_CURRENT, all present pages are locked, but
        /// mlockall() will not fault in non-present pages.  When used
        /// with MCL_FUTURE, all future mappings will be marked to lock
        /// pages when they are faulted in, but they will not be
        /// populated by the lock when the mapping is created.
        /// MCL_ONFAULT must be used with either MCL_CURRENT or
        /// MCL_FUTURE or both.
        const MCL_ONFAULT = lowlevel::mman::flags::MCL_ONFAULT;
    }
}

/// locks pages in the address range starting at addr and
/// continuing for size bytes.  All pages that contain a part of the
/// specified address range are guaranteed to be resident in RAM when
/// the call returns successfully; the pages are guaranteed to stay in
/// RAM until later unlocked.
/// # Safety
/// See [here](https://man7.org/linux/man-pages/man2/munlock.2.html)
pub unsafe fn mlock(addr: *const c_void, len: usize) -> Result<(), Errno> {
    lowlevel::mman::mlock(addr, len).map(|_| ())
}
/// also locks pages in the specified range starting at addr
/// and continuing for size bytes.  However, the state of the pages
/// contained in that range after the call returns successfully will
/// depend on the value in the flags argument.
/// The flags argument can be either 0 or the following constant: MLOCK_ONFAULT
/// # Safety
/// See [here](https://man7.org/linux/man-pages/man2/munlock.2.html)
pub unsafe fn mlock2(addr: *const c_void, len: usize, flags: MmanFlags) -> Result<(), Errno> {
    lowlevel::mman::mlock2(addr, len, flags.bits()).map(|_| ())
}
/// unlocks pages in the address range starting at addr and
/// continuing for size bytes.  After this call, all pages that
/// contain a part of the specified memory range can be moved to
/// external swap space again by the kernel.
/// # Safety
/// See [here](https://man7.org/linux/man-pages/man2/munlock.2.html)
pub unsafe fn munlock(addr: *const c_void, len: usize) -> Result<(), Errno> {
    lowlevel::mman::munlock(addr, len).map(|_| ())
}
/// locks all pages mapped into the address space of the
/// calling process.  This includes the pages of the code, data, and
/// stack segment, as well as shared libraries, user space kernel
/// data, shared memory, and memory-mapped files.  All mapped pages
/// are guaranteed to be resident in RAM when the call returns
/// successfully; the pages are guaranteed to stay in RAM until later
/// unlocked.
/// The flags argument is constructed as the bitwise OR of one or more
/// of the following constants: MCL_CURRENT | MCL_FUTURE | MCL_ONFAULT
/// # Safety
/// See [here](https://man7.org/linux/man-pages/man2/munlock.2.html)
pub fn mlockall(flags: MmanFlags) -> Result<(), Errno> {
    unsafe { lowlevel::mman::mlockall(flags.bits()).map(|_| ()) }
}

/// unlocks all pages mapped into the address space of the calling process.
/// # Safety
/// See [here](https://man7.org/linux/man-pages/man2/munlock.2.html)
pub fn munlockall() -> Result<(), Errno> {
    unsafe { lowlevel::mman::munlockall().map(|_| ()) }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mlockall() {
        mlockall(MmanFlags::MCL_FUTURE).unwrap();
    }
}
