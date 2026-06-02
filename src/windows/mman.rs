use crate::mman::MmanFlags;
use std::ffi::c_void;

pub unsafe fn mlock(addr: *const c_void, len: usize) -> Result<(), std::io::Error> {
    windows::Win32::System::Memory::VirtualLock(addr, len)?;
    Ok(())
}

pub unsafe fn mlock2(
    addr: *const c_void,
    len: usize,
    _flags: MmanFlags,
) -> Result<(), std::io::Error> {
    windows::Win32::System::Memory::VirtualLock(addr, len)?;
    Ok(())
}

pub unsafe fn munlock(addr: *const c_void, len: usize) -> Result<(), std::io::Error> {
    windows::Win32::System::Memory::VirtualUnlock(addr, len)?;
    Ok(())
}

pub fn mlockall(_flags: MmanFlags) -> Result<(), std::io::Error> {
    Ok(())
}

pub fn munlockall() -> Result<(), std::io::Error> {
    Ok(())
}
