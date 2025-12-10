use std::{ffi::c_int, os::raw::c_void};
use syscalls::{syscall, Errno, Sysno};

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "x86",
    target_arch = "s390x",
    target_arch = "mips64",
    target_arch = "riscv64",
    target_arch = "riscv32",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "aarch64",
))]
pub mod flags {
    use std::ffi::c_int;

    pub const MCL_CURRENT: c_int = 0x01;
    pub const MCL_FUTURE: c_int = 0x02;
    pub const MCL_ONFAULT: c_int = 0x04;
}

#[cfg(any(
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "sparc64",
    target_arch = "sparc"
))]
pub mod flags {
    use std::ffi::c_int;
    pub const MCL_CURRENT: c_int = 0x2000;
    pub const MCL_FUTURE: c_int = 0x4000;
    pub const MCL_ONFAULT: c_int = 0x8000;
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn mlock(addr: *const c_void, len: usize) -> Result<usize, Errno> {
    syscall!(Sysno::mlock, addr, len)
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn mlock2(addr: *const c_void, len: usize, flags: c_int) -> Result<usize, Errno> {
    syscall!(Sysno::mlock2, addr, len, flags)
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn munlock(addr: *const c_void, len: usize) -> Result<usize, Errno> {
    syscall!(Sysno::munlock, addr, len)
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn mlockall(flags: c_int) -> Result<usize, Errno> {
    syscall!(Sysno::mlockall, flags)
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn munlockall() -> Result<usize, Errno> {
    syscall!(Sysno::munlockall)
}

#[cfg(test)]
mod test {
    use super::*;
    use libc::malloc;

    #[test]
    fn test_mlock() {
        let mem = unsafe { malloc(20) };
        assert_eq!(unsafe { mlock(mem, 20) }, Ok(0));
        assert_eq!(unsafe { munlock(mem, 20) }, Ok(0));
        assert_eq!(unsafe { munlock(mem, 20) }, Ok(0));
    }
}
