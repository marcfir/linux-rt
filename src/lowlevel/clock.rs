use std::ffi::c_int;

use syscalls::{syscall, Errno, Sysno};

#[allow(non_camel_case_types)]
pub type clockid_t = std::ffi::c_int;

pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;
pub const CLOCK_MONOTONIC_RAW: clockid_t = 4;
pub const CLOCK_REALTIME_COARSE: clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE: clockid_t = 6;
pub const CLOCK_BOOTTIME: clockid_t = 7;
pub const CLOCK_REALTIME_ALARM: clockid_t = 8;
pub const CLOCK_BOOTTIME_ALARM: clockid_t = 9;
/// The driver implementing this got removed. The clock ID is kept as a
/// place holder. Do not reuse!
#[deprecated]
#[allow(dead_code)]
pub const CLOCK_SGI_CYCLE: clockid_t = 10;
pub const CLOCK_TAI: clockid_t = 11;

pub const TIMER_ABSTIME: c_int = 0x01;

/// Time in seconds and nanoseconds.
/// The time is normalized when [TimeSpec::tv_nsec] is in the range of [0, 999'999'999].
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TimeSpec {
    /// Seconds
    pub tv_sec: std::ffi::c_long,
    // linux x32 compatibility
    // See https://sourceware.org/bugzilla/show_bug.cgi?id=16437
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    /// Nanoseconds [0, 999'999'999]
    pub tv_nsec: std::ffi::i64,
    #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
    /// Nanoseconds [0, 999'999'999]
    pub tv_nsec: std::ffi::c_long,
}

impl TimeSpec {
    #[inline]
    pub const fn new() -> Self {
        Self::zeroed()
    }
    pub const fn zeroed() -> Self {
        Self {
            tv_sec: 0,
            tv_nsec: 0,
        }
    }

    #[inline]
    pub const fn seconds(seconds: i64) -> TimeSpec {
        TimeSpec {
            tv_sec: seconds,
            tv_nsec: 0,
        }
    }
    #[inline]
    pub const fn milliseconds(microseconds: i64) -> Self {
        let (sec, nsec) = (microseconds / 1_000, microseconds % 1_000);
        Self {
            tv_sec: sec,
            tv_nsec: nsec,
        }
    }
    #[inline]
    pub const fn microseconds(microseconds: i64) -> Self {
        let (sec, nsec) = (microseconds / 1_000_000, microseconds % 1_000_000);
        Self {
            tv_sec: sec,
            tv_nsec: nsec,
        }
    }
    #[inline]
    pub const fn nanoseconds(nanoseconds: i64) -> Self {
        let (sec, nsec) = (nanoseconds / 1_000_000_000, nanoseconds % 1_000_000_000);
        Self {
            tv_sec: sec,
            tv_nsec: nsec,
        }
    }
    #[inline]
    pub const fn as_nanoseconds(&self) -> i64 {
        self.tv_sec * 1_000_000_000 + self.tv_nsec
    }
    #[inline]
    pub const fn as_nanoseconds_i128(&self) -> i128 {
        self.tv_sec as i128 * 1_000_000_000 + self.tv_nsec as i128
    }
    #[inline]
    pub const fn as_microseconds(&self) -> i64 {
        self.tv_sec * 1_000_000 + self.tv_nsec / 1_000
    }
    #[inline]
    pub const fn as_milliseconds(&self) -> i64 {
        self.tv_sec * 1_000 + self.tv_nsec / 1_000_000
    }
}

impl Default for TimeSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl core::ops::Neg for TimeSpec {
    type Output = TimeSpec;

    fn neg(self) -> TimeSpec {
        TimeSpec::nanoseconds(-self.as_nanoseconds())
    }
}

impl core::ops::Add for TimeSpec {
    type Output = TimeSpec;

    fn add(self, rhs: TimeSpec) -> TimeSpec {
        TimeSpec::nanoseconds(self.as_nanoseconds() + rhs.as_nanoseconds())
    }
}

impl core::ops::Sub for TimeSpec {
    type Output = TimeSpec;

    fn sub(self, rhs: TimeSpec) -> TimeSpec {
        TimeSpec::nanoseconds(self.as_nanoseconds() - rhs.as_nanoseconds())
    }
}

impl core::ops::Mul<i32> for TimeSpec {
    type Output = TimeSpec;

    fn mul(self, rhs: i32) -> TimeSpec {
        let nsec = self
            .as_nanoseconds()
            .checked_mul(i64::from(rhs))
            .expect("TimeSpec multiply out of bounds");

        TimeSpec::nanoseconds(nsec)
    }
}

impl core::ops::Div<i32> for TimeSpec {
    type Output = TimeSpec;

    fn div(self, rhs: i32) -> TimeSpec {
        let nsec = self.as_nanoseconds() / i64::from(rhs);
        TimeSpec::nanoseconds(nsec)
    }
}

/// Retrieve the time of the specified clock [clockid_t].
#[allow(clippy::missing_safety_doc)]
pub unsafe fn clock_gettime(clockid: clockid_t, tp: *mut TimeSpec) -> Result<usize, Errno> {
    syscall!(Sysno::clock_gettime, clockid, tp)
}

/// Set the time of the specified clock [clockid_t].
#[allow(clippy::missing_safety_doc)]
pub unsafe fn clock_settime(clockid: clockid_t, tp: *const TimeSpec) -> Result<usize, Errno> {
    syscall!(Sysno::clock_settime, clockid, tp)
}

#[allow(clippy::missing_safety_doc)]
/// # Parameter
///  * `remain` nullable
pub unsafe fn clock_nanosleep(
    clockid: clockid_t,
    flags: c_int,
    tp: *const TimeSpec,
    remain: *mut TimeSpec,
) -> Result<usize, Errno> {
    syscall!(Sysno::clock_nanosleep, clockid, flags, tp, remain)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nanos() {
        assert_eq!(
            TimeSpec::nanoseconds(0),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 0
            }
        );
        assert_eq!(
            TimeSpec::nanoseconds(-0),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 0
            }
        );

        assert_eq!(
            TimeSpec::nanoseconds(1_000_000_000),
            TimeSpec {
                tv_sec: 1,
                tv_nsec: 0
            }
        );
        assert_eq!(
            TimeSpec::nanoseconds(999_999_999),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 999_999_999
            }
        );
        assert_eq!(
            TimeSpec::nanoseconds(1_999_999_999),
            TimeSpec {
                tv_sec: 1,
                tv_nsec: 999_999_999
            }
        );

        assert_eq!(
            TimeSpec::nanoseconds(-1_999_999_999),
            TimeSpec {
                tv_sec: -1,
                tv_nsec: -999_999_999
            }
        );
    }

    #[test]
    fn test_ops() {
        assert_eq!(
            TimeSpec::nanoseconds(1_999_999_999) + TimeSpec::nanoseconds(1),
            TimeSpec::seconds(2)
        );
        assert_eq!(
            TimeSpec::nanoseconds(1_999_999_999) - TimeSpec::nanoseconds(1),
            TimeSpec::nanoseconds(1_999_999_998)
        );
        assert_eq!(
            TimeSpec::seconds(1) - TimeSpec::nanoseconds(1_999_999_999),
            TimeSpec::nanoseconds(-999_999_999)
        );
        assert_eq!(
            TimeSpec::nanoseconds(1_999_999_999) * 2,
            TimeSpec::nanoseconds(3_999_999_998)
        );
        assert_eq!(
            TimeSpec::seconds(3) / 2,
            TimeSpec::nanoseconds(1_500_000_000)
        );
    }
}
