use std::ffi::c_int;

pub fn ret_to_result<T>(ret: c_int, ok: T) -> Result<T, std::io::Error> {
    if ret == 0 {
        Ok(ok)
    } else {
        Err(std::io::Error::last_os_error())
    }
}

pub fn ret_into_result(ret: c_int) -> Result<c_int, std::io::Error> {
    if ret == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(ret)
    }
}
