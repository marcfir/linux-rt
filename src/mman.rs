use std::ffi::c_void;

use bitflags::bitflags;

#[cfg(target_family = "unix")]
bitflags! {
    /// These flags control the scheduling behavior
    #[derive(Debug, Clone, PartialEq)]
    pub struct MmanFlags: std::ffi::c_int {
        /// Lock all pages which are currently mapped into the address
        /// space of the process.
        const MCL_CURRENT = libc::MCL_CURRENT;
        /// Lock all pages which will become mapped into the address
        /// space of the process in the future.  These could be, for
        /// instance, new pages required by a growing heap and stack as
        /// well as new memory-mapped files or shared memory regions.
        const MCL_FUTURE = libc::MCL_FUTURE;
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
        const MCL_ONFAULT = libc::MCL_ONFAULT;
    }
}

#[cfg(target_family = "windows")]
bitflags! {
    /// These flags control the scheduling behavior
    #[derive(Debug, Clone, PartialEq)]
    pub struct MmanFlags: std::ffi::c_int {
        /// Lock all pages which are currently mapped into the address
        /// space of the process.
        const MCL_CURRENT = 1;
        /// Lock all pages which will become mapped into the address
        /// space of the process in the future.  These could be, for
        /// instance, new pages required by a growing heap and stack as
        /// well as new memory-mapped files or shared memory regions.
        const MCL_FUTURE = 2;
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
        const MCL_ONFAULT = 4;
    }
}

/// # Linux
/// Locks pages in the address range starting at addr and
/// continuing for size bytes. All pages that contain a part of the
/// specified address range are guaranteed to be resident in RAM when
/// the call returns successfully; the pages are guaranteed to stay in
/// RAM until later unlocked.
/// ## Safety
/// See [here](https://man7.org/linux/man-pages/man2/munlock.2.html)
/// # Windows
/// Uses the VirtualLock function (memoryapi.h) see [here](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtuallock)
pub unsafe fn mlock(addr: *const c_void, len: usize) -> Result<(), std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::mman::mlock(addr, len)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::mman::mlock(addr, len)
    }
}
/// # Linux
/// Also locks pages in the specified range starting at addr
/// and continuing for size bytes.  However, the state of the pages
/// contained in that range after the call returns successfully will
/// depend on the value in the flags argument.
/// The flags argument can be either 0 or the following constant: MLOCK_ONFAULT
/// ## Safety
/// See [here](https://man7.org/linux/man-pages/man2/munlock.2.html)
/// # Windows
/// Works like mlock. The `flags` are ignored.
pub unsafe fn mlock2(
    addr: *const c_void,
    len: usize,
    flags: MmanFlags,
) -> Result<(), std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::mman::mlock2(addr, len, flags)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::mman::mlock2(addr, len, flags)
    }
}
/// # Linux
/// Unlocks pages in the address range starting at addr and
/// continuing for size bytes.  After this call, all pages that
/// contain a part of the specified memory range can be moved to
/// external swap space again by the kernel.
/// ## Safety
/// See [here](https://man7.org/linux/man-pages/man2/munlock.2.html)
/// # Windows
/// Uses the VirtualUnlock function (memoryapi.h) see [here](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualunlock)
pub unsafe fn munlock(addr: *const c_void, len: usize) -> Result<(), std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::mman::munlock(addr, len)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::mman::munlock(addr, len)
    }
}
/// # Linux
/// Locks all pages mapped into the address space of the
/// calling process.  This includes the pages of the code, data, and
/// stack segment, as well as shared libraries, user space kernel
/// data, shared memory, and memory-mapped files.  All mapped pages
/// are guaranteed to be resident in RAM when the call returns
/// successfully; the pages are guaranteed to stay in RAM until later
/// unlocked.
/// The flags argument is constructed as the bitwise OR of one or more
/// of the following constants: MCL_CURRENT | MCL_FUTURE | MCL_ONFAULT
/// ## Safety
/// See [here](https://man7.org/linux/man-pages/man2/munlock.2.html)
/// # Windows
/// Not supported. Call ignored.
pub fn mlockall(flags: MmanFlags) -> Result<(), std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::mman::mlockall(flags)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::mman::mlockall(flags)
    }
}

/// # Linux
/// Unlocks all pages mapped into the address space of the calling process.
/// ## Safety
/// See [here](https://man7.org/linux/man-pages/man2/munlock.2.html)
/// # Windows
/// Not supported. Call ignored.
pub fn munlockall() -> Result<(), std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::mman::munlockall()
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::mman::munlockall()
    }
}

#[cfg(test)]
mod test {

    use std::alloc::Layout;

    use super::*;
    #[test]
    fn test_mlockall() {
        mlockall(MmanFlags::MCL_FUTURE).unwrap();
    }

    #[test]
    fn test_mlock() {
        let mem = unsafe { std::alloc::alloc(Layout::new::<[u8; 20]>()) } as *const c_void;
        unsafe { mlock(mem, 20) }.unwrap();
        unsafe { munlock(mem, 20) }.unwrap();
    }
}
