/// The [ClockId] is the identifier of the particular clock on
/// which to act. A clock may be system-wide and hence visible for
/// all processes, or per-process if it measures time only within a
/// single process.
#[derive(Debug, Clone, Copy)]
pub enum ClockId {
    /// # Unix
    /// A settable system-wide clock that measures real (i.e., wall-
    /// clock) time.  Setting this clock requires appropriate privi‐
    /// leges.  This clock is affected by discontinuous jumps in the
    /// system time (e.g., if the system administrator manually
    /// changes the clock), and by the incremental adjustments per‐
    /// formed by adjtime(3) and NTP.
    /// # Windows
    /// The GetSystemTimePreciseAsFileTime function retrieves the current
    /// system date and time with the highest possible level of precision (<1us).
    /// The retrieved information is in Coordinated Universal Time (UTC) format.
    ClockRealtime,

    /// (since Linux 3.10; Linux-specific)
    /// A nonsettable system-wide clock derived from wall-clock time
    /// but ignoring leap seconds.  This clock does not experience
    /// discontinuities and backwards jumps caused by NTP inserting
    /// leap seconds as CLOCK_REALTIME does.
    ///
    /// The acronym TAI refers to International Atomic Time.
    #[cfg(target_os = "linux")]
    ClockTai,
    /// A nonsettable system-wide clock that represents monotonic time
    /// since—as described by POSIX—"some unspecified point in the
    /// past".  On Linux, that point corresponds to the number of sec‐
    /// onds that the system has been running since it was booted.
    ///
    /// The CLOCK_MONOTONIC clock is not affected by discontinuous
    /// jumps in the system time (e.g., if the system administrator
    /// manually changes the clock), but is affected by the incremen‐
    /// tal adjustments performed by adjtime(3) and NTP.  This clock
    /// does not count time that the system is suspended.  All
    /// CLOCK_MONOTONIC variants guarantee that the time returned by
    /// consecutive calls will not go backwards, but successive calls
    /// may—depending on the architecture—return identical (not-
    /// increased) time values.
    ClockMonotonic,
}
#[cfg(target_os = "linux")]
impl ClockId {
    /// Get the raw `clockid_t`.
    pub const fn as_raw(&self) -> libc::clockid_t {
        match self {
            ClockId::ClockRealtime => libc::CLOCK_REALTIME,

            ClockId::ClockTai => libc::CLOCK_TAI,
            ClockId::ClockMonotonic => libc::CLOCK_MONOTONIC,
        }
    }
    /// Creates [ClockId] from raw `clockid_t`.
    pub const fn from_raw(clockid: libc::clockid_t) -> Option<Self> {
        match clockid {
            libc::CLOCK_REALTIME => Some(ClockId::ClockRealtime),
            #[cfg(target_os = "linux")]
            libc::CLOCK_TAI => Some(ClockId::ClockTai),
            libc::CLOCK_MONOTONIC => Some(ClockId::ClockMonotonic),
            _ => None,
        }
    }
}

#[cfg(target_family = "unix")]
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

#[cfg(target_family = "windows")]
/// Time in seconds and nanoseconds.
/// The time is normalized when [TimeSpec::tv_nsec] is in the range of [0, 999'999'999].
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TimeSpec {
    /// Seconds
    pub tv_sec: i64,
    /// Nanoseconds [0, 999'999'999]
    pub tv_nsec: i64,
}

impl TimeSpec {
    /// Returns a zero-initialized [TimeSpec].
    #[inline]
    pub const fn zeroed() -> Self {
        Self {
            tv_sec: 0,
            tv_nsec: 0,
        }
    }
    /// Creates a new [TimeSpec] from the specified number of whole seconds.
    #[inline]
    pub const fn seconds(seconds: i64) -> TimeSpec {
        TimeSpec {
            tv_sec: seconds,
            tv_nsec: 0,
        }
    }

    /// Creates a new [TimeSpec] from the specified number of milliseconds.
    #[inline]
    pub const fn milliseconds(milliseconds: i64) -> Self {
        let sec = milliseconds.div_euclid(1_000);
        let nsec = milliseconds.rem_euclid(1_000) * 1_000_000;

        Self {
            tv_sec: sec,
            tv_nsec: nsec,
        }
    }

    /// Creates a new [TimeSpec] from the specified number of microseconds.
    #[inline]
    pub const fn microseconds(microseconds: i64) -> Self {
        let sec = microseconds.div_euclid(1_000_000);
        let nsec = microseconds.rem_euclid(1_000_000) * 1_000;

        Self {
            tv_sec: sec,
            tv_nsec: nsec,
        }
    }

    /// Creates a new [TimeSpec] from the specified number of nanoseconds.
    #[inline]
    pub const fn nanoseconds(nanoseconds: i64) -> Self {
        let sec = nanoseconds.div_euclid(1_000_000_000);
        let nsec = nanoseconds.rem_euclid(1_000_000_000);

        Self {
            tv_sec: sec,
            tv_nsec: nsec,
        }
    }

    /// Returns the total number of nanoseconds contained by this [TimeSpec].
    #[inline]
    pub const fn as_nanoseconds(&self) -> i64 {
        self.tv_sec * 1_000_000_000 + self.tv_nsec
    }

    /// Returns the total number of nanoseconds contained by this [TimeSpec].
    #[inline]
    pub const fn as_nanoseconds_i128(&self) -> i128 {
        self.tv_sec as i128 * 1_000_000_000 + self.tv_nsec as i128
    }

    /// Returns the total number of whole microseconds contained by this [TimeSpec].
    #[inline]
    pub const fn as_microseconds(&self) -> i64 {
        self.tv_sec * 1_000_000 + self.tv_nsec / 1_000
    }

    /// Returns the total number of whole milliseconds contained by this [TimeSpec].
    #[inline]
    pub const fn as_milliseconds(&self) -> i64 {
        self.tv_sec * 1_000 + self.tv_nsec / 1_000_000
    }
}

impl Default for TimeSpec {
    fn default() -> Self {
        Self::zeroed()
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

impl Ord for TimeSpec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.tv_sec == other.tv_sec {
            self.tv_nsec.cmp(&other.tv_nsec)
        } else {
            self.tv_sec.cmp(&other.tv_sec)
        }
    }
}
impl Eq for TimeSpec {}
impl PartialOrd for TimeSpec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Retrieve the time of the specified clock [ClockId].
pub fn get_time(clockid: ClockId) -> Result<TimeSpec, std::io::Error> {
    #[cfg(target_os = "linux")]
    {
        crate::linux::clock::get_time(clockid)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::clock::get_time(clockid)
    }
}

/// Set the time `tp` of the specified clock [ClockId].
pub fn set_time(clockid: ClockId, ts: TimeSpec) -> Result<(), std::io::Error> {
    #[cfg(target_os = "linux")]
    {
        crate::linux::clock::set_time(clockid, ts)
    }
    #[cfg(target_os = "windows")]
    {
        todo!(
            "set_time not implemented on windows! {:?},{:?}",
            clockid,
            ts
        )
    }
}

/// The [nanosleep_relative] function shall cause the current thread to be
/// suspended from execution until either the time interval specified
/// by the `ts` argument has elapsed, or a signal is delivered to the
/// calling thread and its action is to invoke a signal-catching
/// function, or the process is terminated. The clock used to measure
/// the time shall be the clock specified by [ClockId].
pub fn nanosleep_relative(clockid: ClockId, ts: TimeSpec) -> Result<(), std::io::Error> {
    #[cfg(target_os = "linux")]
    {
        crate::linux::clock::nanosleep_relative(clockid, ts)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::clock::nanosleep_relative(clockid, ts)
    }
}
/// The [nanosleep_absolute] function shall cause the current thread to be
/// suspended from execution until either the time value of the clock
/// specified by [ClockId] reaches the absolute time specified by the
/// `ts` argument, or a signal is delivered to the calling thread and
/// its action is to invoke a signal-catching function, or the process
/// is terminated.  If, at the time of the call, the time value
/// specified by `ts` is less than or equal to the time value of the
/// specified clock, then [nanosleep_absolute] shall return immediately
/// and the calling process shall not be suspended.
pub fn nanosleep_absolute(clockid: ClockId, ts: TimeSpec) -> Result<(), std::io::Error> {
    #[cfg(target_os = "linux")]
    {
        crate::linux::clock::nanosleep_absolute(clockid, ts)
    }

    #[cfg(target_os = "windows")]
    {
        crate::windows::clock::nanosleep_absolute(clockid, ts)
    }
}

/// Like [nanosleep_relative] but returns the amount of time remaining in the
/// interval (the requested time minus the time actually slept)
pub fn nanosleep_relative_with_remain(
    clockid: ClockId,
    ts: TimeSpec,
) -> Result<TimeSpec, std::io::Error> {
    #[cfg(target_os = "linux")]
    {
        crate::linux::clock::nanosleep_relative_with_remain(clockid, ts)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::clock::nanosleep_relative_with_remain(clockid, ts)
    }
}
/// Like [nanosleep_absolute] but returns the amount of time remaining in the
/// interval (the requested time minus the time actually slept)
pub fn nanosleep_absolute_with_remain(
    clockid: ClockId,
    ts: TimeSpec,
) -> Result<TimeSpec, std::io::Error> {
    #[cfg(target_os = "linux")]
    {
        crate::linux::clock::nanosleep_absolute_with_remain(clockid, ts)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::clock::nanosleep_absolute_with_remain(clockid, ts)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_time() {
        let time = get_time(ClockId::ClockMonotonic).unwrap();
        assert!(time.tv_sec > 0);
    }

    #[test]
    fn test_sleep() {
        nanosleep_relative(
            ClockId::ClockMonotonic,
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 1_000_000,
            },
        )
        .unwrap();
        nanosleep_relative_with_remain(
            ClockId::ClockMonotonic,
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 1_000_000,
            },
        )
        .unwrap();
        // assert!(time.tv_sec > 0);
    }

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
                tv_sec: -2,
                tv_nsec: 1
            }
        );
    }

    #[test]
    fn test_timespec_construct() {
        // nanoseconds

        assert_eq!(
            TimeSpec::nanoseconds(0),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 0
            }
        );
        assert_eq!(
            TimeSpec::nanoseconds(22),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 22
            }
        );
        assert_eq!(
            TimeSpec::nanoseconds(1),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 1
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
            TimeSpec::nanoseconds(1_000_000_000),
            TimeSpec {
                tv_sec: 1,
                tv_nsec: 0
            }
        );

        assert_eq!(
            TimeSpec::nanoseconds(1_500_000_000),
            TimeSpec {
                tv_sec: 1,
                tv_nsec: 500_000_000
            }
        );

        assert_eq!(
            TimeSpec::nanoseconds(-1),
            TimeSpec {
                tv_sec: -1,
                tv_nsec: 999_999_999
            }
        );

        assert_eq!(
            TimeSpec::nanoseconds(-1_500_000_000),
            TimeSpec {
                tv_sec: -2,
                tv_nsec: 500_000_000
            }
        );

        // microseconds

        assert_eq!(
            TimeSpec::microseconds(22),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 22_000
            }
        );

        assert_eq!(
            TimeSpec::microseconds(0),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 0
            }
        );

        assert_eq!(
            TimeSpec::microseconds(1),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 1_000
            }
        );

        assert_eq!(
            TimeSpec::microseconds(999_999),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 999_999_000
            }
        );

        assert_eq!(
            TimeSpec::microseconds(1_000_000),
            TimeSpec {
                tv_sec: 1,
                tv_nsec: 0
            }
        );

        assert_eq!(
            TimeSpec::microseconds(1_500_000),
            TimeSpec {
                tv_sec: 1,
                tv_nsec: 500_000_000
            }
        );

        assert_eq!(
            TimeSpec::microseconds(-1),
            TimeSpec {
                tv_sec: -1,
                tv_nsec: 999_999_000
            }
        );

        // milliseconds

        assert_eq!(
            TimeSpec::milliseconds(22),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 22_000_000
            }
        );

        assert_eq!(
            TimeSpec::milliseconds(-500),
            TimeSpec {
                tv_sec: -1,
                tv_nsec: 500_000_000
            }
        );

        assert_eq!(
            TimeSpec::milliseconds(0),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 0
            }
        );

        assert_eq!(
            TimeSpec::milliseconds(1),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 1_000_000
            }
        );

        assert_eq!(
            TimeSpec::milliseconds(999),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 999_000_000
            }
        );

        assert_eq!(
            TimeSpec::milliseconds(1_000),
            TimeSpec {
                tv_sec: 1,
                tv_nsec: 0
            }
        );

        assert_eq!(
            TimeSpec::milliseconds(1_500),
            TimeSpec {
                tv_sec: 1,
                tv_nsec: 500_000_000
            }
        );

        assert_eq!(
            TimeSpec::milliseconds(-1),
            TimeSpec {
                tv_sec: -1,
                tv_nsec: 999_000_000
            }
        );

        assert_eq!(
            TimeSpec::milliseconds(-1_500),
            TimeSpec {
                tv_sec: -2,
                tv_nsec: 500_000_000
            }
        );

        // seconds

        assert_eq!(
            TimeSpec::seconds(0),
            TimeSpec {
                tv_sec: 0,
                tv_nsec: 0
            }
        );

        assert_eq!(
            TimeSpec::seconds(1),
            TimeSpec {
                tv_sec: 1,
                tv_nsec: 0
            }
        );

        assert_eq!(
            TimeSpec::seconds(123),
            TimeSpec {
                tv_sec: 123,
                tv_nsec: 0
            }
        );

        assert_eq!(
            TimeSpec::seconds(-1),
            TimeSpec {
                tv_sec: -1,
                tv_nsec: 0
            }
        );

        assert_eq!(
            TimeSpec::seconds(-123),
            TimeSpec {
                tv_sec: -123,
                tv_nsec: 0
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
        assert!(TimeSpec::nanoseconds(1_500_000_000) == TimeSpec::nanoseconds(1_500_000_000));
        assert!(TimeSpec::nanoseconds(1_500_000_000) != TimeSpec::nanoseconds(1_500_000_001));
        assert!(TimeSpec::nanoseconds(1_500_000_000) < TimeSpec::nanoseconds(1_500_000_001));
        assert!(TimeSpec::nanoseconds(1_500_100_000) > TimeSpec::nanoseconds(1_500_000_001));
        assert!(TimeSpec::nanoseconds(-1_500_000_000) == TimeSpec::nanoseconds(-1_500_000_000));
        assert!(TimeSpec::nanoseconds(-1_500_000_000) != TimeSpec::nanoseconds(-1_500_000_001));
        assert!(TimeSpec::nanoseconds(-1_500_000_000) > TimeSpec::nanoseconds(-1_500_000_001));
        assert!(TimeSpec::nanoseconds(-1_500_100_000) < TimeSpec::nanoseconds(-1_500_000_001));
    }
}
