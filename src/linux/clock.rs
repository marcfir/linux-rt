use crate::{
    clock::{ClockId, TimeSpec},
    linux::error::ret_to_result,
};

pub fn get_time(clockid: ClockId) -> Result<TimeSpec, std::io::Error> {
    let mut tp = TimeSpec::zeroed();
    let ret = unsafe { libc::clock_gettime(clockid.as_raw(), &raw mut tp as *mut libc::timespec) };
    ret_to_result(ret, tp)
}

pub fn set_time(clockid: ClockId, ts: TimeSpec) -> Result<(), std::io::Error> {
    let ret =
        unsafe { libc::clock_settime(clockid.as_raw(), &raw const ts as *const libc::timespec) };
    ret_to_result(ret, ())
}

pub fn nanosleep_relative(_clockid: ClockId, ts: TimeSpec) -> Result<(), std::io::Error> {
    #[cfg(target_os = "linux")]
    let ret = unsafe {
        libc::clock_nanosleep(
            _clockid.as_raw(),
            0,
            &raw const ts as *const libc::timespec,
            core::ptr::null_mut(),
        )
    };
    #[cfg(target_os = "macos")]
    let ret = unsafe {
        libc::nanosleep(
            &raw const ts as *const libc::timespec,
            core::ptr::null_mut(),
        )
    };
    ret_to_result(ret, ())
}
pub fn nanosleep_absolute(clockid: ClockId, ts: TimeSpec) -> Result<(), std::io::Error> {
    #[cfg(target_os = "linux")]
    let ret = unsafe {
        libc::clock_nanosleep(
            clockid.as_raw(),
            libc::TIMER_ABSTIME,
            &raw const ts as *const libc::timespec,
            core::ptr::null_mut(),
        )
    };
    #[cfg(target_os = "macos")]
    let ret = {
        let now = get_time(clockid)?;
        let relative_sleep = (ts - now).as_nanoseconds().max(0) as u64;
        unsafe {
            libc::nanosleep(
                &raw const relative_sleep as *const libc::timespec,
                core::ptr::null_mut(),
            )
        }
    };
    ret_to_result(ret, ())
}
pub fn nanosleep_relative_with_remain(
    _clockid: ClockId,
    ts: TimeSpec,
) -> Result<TimeSpec, std::io::Error> {
    let mut remaining = TimeSpec::zeroed();
    #[cfg(target_os = "linux")]
    {
        let ret = unsafe {
            libc::clock_nanosleep(
                _clockid.as_raw(),
                0,
                &raw const ts as *const libc::timespec,
                &raw mut remaining as *mut libc::timespec,
            )
        };
    }

    #[cfg(target_os = "macos")]
    let ret = unsafe {
        libc::nanosleep(
            &raw const ts as *const libc::timespec,
            &raw mut remaining as *mut libc::timespec,
        )
    };
    ret_to_result(ret, remaining)
}
pub fn nanosleep_absolute_with_remain(
    clockid: ClockId,
    ts: TimeSpec,
) -> Result<TimeSpec, std::io::Error> {
    let mut remaining = TimeSpec::zeroed();
    #[cfg(target_os = "linux")]
    let ret = unsafe {
        libc::clock_nanosleep(
            clockid.as_raw(),
            libc::TIMER_ABSTIME,
            &raw const ts as *const libc::timespec,
            &raw mut remaining as *mut libc::timespec,
        )
    };
    #[cfg(target_os = "macos")]
    let ret = {
        let now = get_time(clockid)?;
        let relative_sleep = (ts - now).as_nanoseconds().max(0) as u64;
        unsafe {
            libc::nanosleep(
                &raw const relative_sleep as *const libc::timespec,
                &raw mut remaining as *mut libc::timespec,
            )
        }
    };
    ret_to_result(ret, remaining)
}
