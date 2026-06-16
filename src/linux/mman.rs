use crate::mman::MmanFlags;
#[cfg(target_os = "linux")]
use std::ffi::c_uint;
use std::ffi::c_void;

pub unsafe fn mlock(addr: *const c_void, len: usize) -> Result<(), std::io::Error> {
    let ret = libc::mlock(addr, len);
    if ret == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(target_os = "linux")]
pub unsafe fn mlock2(
    addr: *const c_void,
    len: usize,
    flags: MmanFlags,
) -> Result<(), std::io::Error> {
    let ret = libc::mlock2(addr, len, flags.bits() as c_uint);
    if ret == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(target_os = "macos")]
pub unsafe fn mlock2(
    addr: *const c_void,
    len: usize,
    _flags: MmanFlags,
) -> Result<(), std::io::Error> {
    let ret = libc::mlock(addr, len);
    if ret == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub unsafe fn munlock(addr: *const c_void, len: usize) -> Result<(), std::io::Error> {
    let ret = libc::munlock(addr, len);
    if ret == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn mlockall(flags: MmanFlags) -> Result<(), std::io::Error> {
    let ret = unsafe { libc::mlockall(flags.bits()) };
    if ret == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn munlockall() -> Result<(), std::io::Error> {
    let ret = unsafe { libc::munlockall() };
    if ret == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}
