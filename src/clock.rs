use crate::lowlevel::clock::{
    clock_adjtime, clock_gettime, clock_nanosleep, clock_settime, clockid_t, TimeSpec, Timeval,
    TimexRaw, CLOCK_BOOTTIME, CLOCK_BOOTTIME_ALARM, CLOCK_MONOTONIC, CLOCK_MONOTONIC_COARSE,
    CLOCK_MONOTONIC_RAW, CLOCK_PROCESS_CPUTIME_ID, CLOCK_REALTIME, CLOCK_REALTIME_ALARM,
    CLOCK_REALTIME_COARSE, CLOCK_TAI, CLOCK_THREAD_CPUTIME_ID, TIMER_ABSTIME,
};
use syscalls::Errno;

/// The [ClockId] is the identifier of the particular clock on
/// which to act. A clock may be system-wide and hence visible for
/// all processes, or per-process if it measures time only within a
/// single process.
#[derive(Debug, Clone, Copy)]
pub enum ClockId {
    /// A settable system-wide clock that measures real (i.e., wall-
    /// clock) time.  Setting this clock requires appropriate privi‐
    /// leges.  This clock is affected by discontinuous jumps in the
    /// system time (e.g., if the system administrator manually
    /// changes the clock), and by the incremental adjustments per‐
    /// formed by adjtime(3) and NTP.
    ClockRealtime,

    /// (since Linux 3.0; Linux-specific)
    /// Like CLOCK_REALTIME, but not settable.  See timer_create(2)
    /// for further details.
    ClockRealtimeAlarm,

    /// (since Linux 2.6.32; Linux-specific)
    /// A faster but less precise version of CLOCK_REALTIME.  This
    /// clock is not settable.  Use when you need very fast, but not
    /// fine-grained timestamps.  Requires per-architecture support,
    /// and probably also architecture support for this flag in the
    /// vdso(7).
    ClockRealtimeCoarse,

    /// (since Linux 3.10; Linux-specific)
    /// A nonsettable system-wide clock derived from wall-clock time
    /// but ignoring leap seconds.  This clock does not experience
    /// discontinuities and backwards jumps caused by NTP inserting
    /// leap seconds as CLOCK_REALTIME does.
    ///
    /// The acronym TAI refers to International Atomic Time.
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

    ///  (since Linux 2.6.32; Linux-specific)
    ///  A faster but less precise version of CLOCK_MONOTONIC.  Use
    /// when you need very fast, but not fine-grained timestamps.
    /// Requires per-architecture support, and probably also architec‐
    /// ture support for this flag in the vdso(7).
    ClockMonotonicCoarse,

    ///  (since Linux 2.6.28; Linux-specific)
    /// Similar to CLOCK_MONOTONIC, but provides access to a raw hard‐
    /// ware-based time that is not subject to NTP adjustments or the
    /// incremental adjustments performed by adjtime(3).  This clock
    /// does not count time that the system is suspended.
    ClockMonotonicRaw,

    /// (since Linux 2.6.39; Linux-specific)
    /// A nonsettable system-wide clock that is identical to
    /// CLOCK_MONOTONIC, except that it also includes any time that
    /// the system is suspended.  This allows applications to get a
    /// suspend-aware monotonic clock without having to deal with the
    /// complications of CLOCK_REALTIME, which may have discontinu‐
    /// ities if the time is changed using settimeofday(2) or similar.
    ClockBoottime,
    /// (since Linux 3.0; Linux-specific)
    /// Like CLOCK_BOOTTIME.  See timer_create(2) for further details.
    ClockBoottimeAlarm,
    /// (since Linux 2.6.12)
    /// This is a clock that measures CPU time consumed by this
    /// process (i.e., CPU time consumed by all threads in the
    /// process).  On Linux, this clock is not settable.
    ClockProcessCputimeId,

    ///  (since Linux 2.6.12)
    /// This is a clock that measures CPU time consumed by this
    /// thread.  On Linux, this clock is not settable.
    ClockThreadCputimeId,
}
impl ClockId {
    /// Get the raw `clockid_t`.
    pub const fn as_raw(&self) -> clockid_t {
        match self {
            ClockId::ClockRealtime => CLOCK_REALTIME,
            ClockId::ClockRealtimeAlarm => CLOCK_REALTIME_ALARM,
            ClockId::ClockRealtimeCoarse => CLOCK_REALTIME_COARSE,
            ClockId::ClockTai => CLOCK_TAI,
            ClockId::ClockMonotonic => CLOCK_MONOTONIC,
            ClockId::ClockMonotonicCoarse => CLOCK_MONOTONIC_COARSE,
            ClockId::ClockMonotonicRaw => CLOCK_MONOTONIC_RAW,
            ClockId::ClockBoottime => CLOCK_BOOTTIME,
            ClockId::ClockBoottimeAlarm => CLOCK_BOOTTIME_ALARM,
            ClockId::ClockProcessCputimeId => CLOCK_PROCESS_CPUTIME_ID,
            ClockId::ClockThreadCputimeId => CLOCK_THREAD_CPUTIME_ID,
        }
    }
    /// Creates [ClockId] from raw `clockid_t`.
    pub const fn from_raw(clockid: clockid_t) -> Option<Self> {
        match clockid {
            CLOCK_REALTIME => Some(ClockId::ClockRealtime),
            CLOCK_REALTIME_ALARM => Some(ClockId::ClockRealtimeAlarm),
            CLOCK_REALTIME_COARSE => Some(ClockId::ClockRealtimeCoarse),
            CLOCK_TAI => Some(ClockId::ClockTai),
            CLOCK_MONOTONIC => Some(ClockId::ClockMonotonic),
            CLOCK_MONOTONIC_COARSE => Some(ClockId::ClockMonotonicCoarse),
            CLOCK_MONOTONIC_RAW => Some(ClockId::ClockMonotonicRaw),
            CLOCK_BOOTTIME => Some(ClockId::ClockBoottime),
            CLOCK_BOOTTIME_ALARM => Some(ClockId::ClockBoottimeAlarm),
            CLOCK_PROCESS_CPUTIME_ID => Some(ClockId::ClockProcessCputimeId),
            CLOCK_THREAD_CPUTIME_ID => Some(ClockId::ClockThreadCputimeId),
            _ => None,
        }
    }
}

/// Retrieve the time of the specified clock [ClockId].
pub fn get_time(clockid: ClockId) -> Result<TimeSpec, Errno> {
    let mut tp = TimeSpec::zeroed();
    unsafe { clock_gettime(clockid.as_raw(), &mut tp).and(Ok(tp)) }
}

/// Set the time `tp` of the specified clock [ClockId].
pub fn set_time(clockid: ClockId, tp: TimeSpec) -> Result<(), Errno> {
    unsafe { clock_settime(clockid.as_raw(), &tp).and(Ok(())) }
}

/// Takes a `Timex` structure, updates kernel parameters from (selected) field
/// values, and updates the same structure with the current
/// kernel values.
pub fn adjust_time(clockid: ClockId, timex: &mut Timex) -> Result<(), Errno> {
    let mut timex_raw = TimexRaw::from_timex(timex);
    unsafe { clock_adjtime(clockid.as_raw(), &raw mut timex_raw).and(Ok(())) }?;
    *timex = timex_raw.into_timex();
    Ok(())
}

/// The [nanosleep_relative] function shall cause the current thread to be
/// suspended from execution until either the time interval specified
/// by the `ts` argument has elapsed, or a signal is delivered to the
/// calling thread and its action is to invoke a signal-catching
/// function, or the process is terminated. The clock used to measure
/// the time shall be the clock specified by [ClockId].
pub fn nanosleep_relative(clockid: ClockId, ts: TimeSpec) -> Result<(), Errno> {
    unsafe { clock_nanosleep(clockid.as_raw(), 0, &ts, core::ptr::null_mut()).and(Ok(())) }
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
pub fn nanosleep_absolute(clockid: ClockId, ts: TimeSpec) -> Result<(), Errno> {
    unsafe {
        clock_nanosleep(clockid.as_raw(), TIMER_ABSTIME, &ts, core::ptr::null_mut()).and(Ok(()))
    }
}

/// Like [nanosleep_relative] but returns the amount of time remaining in the
/// interval (the requested time minus the time actually slept)
pub fn nanosleep_relative_with_remain(clockid: ClockId, ts: TimeSpec) -> Result<TimeSpec, Errno> {
    let mut remaining = TimeSpec::new();
    unsafe { clock_nanosleep(clockid.as_raw(), 0, &ts, &raw mut remaining).and(Ok(remaining)) }
}
/// Like [nanosleep_absolute] but returns the amount of time remaining in the
/// interval (the requested time minus the time actually slept)
pub fn nanosleep_absolute_with_remain(clockid: ClockId, ts: TimeSpec) -> Result<TimeSpec, Errno> {
    let mut remaining = TimeSpec::new();
    unsafe {
        clock_nanosleep(clockid.as_raw(), TIMER_ABSTIME, &ts, &raw mut remaining).and(Ok(remaining))
    }
}

/// The modes field determines which parameters, if any, to set.  It is a bit mask
/// containing a bitwise OR combination of zero or more of the
/// following bits:
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TimexMode(std::ffi::c_uint);
impl TimexMode {
    /// time offset
    pub const ADJ_OFFSET: std::ffi::c_uint = 0x0001;
    /// frequency offset
    pub const ADJ_FREQUENCY: std::ffi::c_uint = 0x0002;
    /// maximum time error
    pub const ADJ_MAXERROR: std::ffi::c_uint = 0x0004;
    /// estimated time error
    pub const ADJ_ESTERROR: std::ffi::c_uint = 0x0008;
    /// clock status
    pub const ADJ_STATUS: std::ffi::c_uint = 0x0010;
    /// pll time constant
    pub const ADJ_TIMECONST: std::ffi::c_uint = 0x0020;
    /// set TAI offset
    pub const ADJ_TAI: std::ffi::c_uint = 0x0080;
    /// add 'time' to current time
    pub const ADJ_SETOFFSET: std::ffi::c_uint = 0x0100;
    /// select microsecond resolution
    pub const ADJ_MICRO: std::ffi::c_uint = 0x1000;
    /// select nanosecond resolution
    pub const ADJ_NANO: std::ffi::c_uint = 0x2000;
    /// tick value
    pub const ADJ_TICK: std::ffi::c_uint = 0x4000;
    /// Check if `mode` is set.
    pub fn is_set(&self, mode: std::ffi::c_uint) -> bool {
        self.0 & mode == mode
    }
    /// Set a `mode`
    pub fn set(&mut self, mode: std::ffi::c_uint) {
        self.0 |= mode;
    }
    pub(crate) fn as_raw(&self) -> std::ffi::c_uint {
        self.0
    }

    pub(crate) fn from_raw(raw: std::ffi::c_uint) -> Self {
        Self(raw)
    }
}

/// The buf.status field is a bit mask that is used to set and/or
/// retrieve status bits associated with the NTP implementation.  Some
/// bits in the mask are both readable and settable, while others are
/// read-only.
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StatusCodes(std::ffi::c_int);
impl StatusCodes {
    /// enable PLL updates (rw)
    pub const STA_PLL: std::ffi::c_int = 0x0001;
    /// enable PPS freq discipline (rw)
    pub const STA_PPSFREQ: std::ffi::c_int = 0x0002;
    /// enable PPS time discipline (rw)
    pub const STA_PPSTIME: std::ffi::c_int = 0x0004;
    /// select frequency-lock mode (rw)
    pub const STA_FLL: std::ffi::c_int = 0x0008;
    /// insert leap (rw)
    pub const STA_INS: std::ffi::c_int = 0x0010;
    /// delete leap (rw)
    pub const STA_DEL: std::ffi::c_int = 0x0020;
    /// clock unsynchronized (rw)
    pub const STA_UNSYNC: std::ffi::c_int = 0x0040;
    /// hold frequency (rw)
    pub const STA_FREQHOLD: std::ffi::c_int = 0x0080;
    /// PPS signal present (ro)
    pub const STA_PPSSIGNAL: std::ffi::c_int = 0x0100;
    /// PPS signal jitter exceeded (ro)
    pub const STA_PPSJITTER: std::ffi::c_int = 0x0200;
    /// PPS signal wander exceeded (ro)
    pub const STA_PPSWANDER: std::ffi::c_int = 0x0400;
    /// PPS signal calibration error (ro)
    pub const STA_PPSERROR: std::ffi::c_int = 0x0800;
    /// clock hardware fault (ro)
    pub const STA_CLOCKERR: std::ffi::c_int = 0x1000;
    /// resolution (0 = us, 1 = ns) (ro)
    pub const STA_NANO: std::ffi::c_int = 0x2000;
    /// mode (0 = PLL, 1 = FLL) (ro)
    pub const STA_MODE: std::ffi::c_int = 0x4000;
    /// clock source (0 = A, 1 = B) (ro)
    pub const STA_CLK: std::ffi::c_int = 0x8000;
    /// Check if `status_code` is set.
    pub fn is_set(&self, status_code: std::ffi::c_int) -> bool {
        self.0 & status_code == status_code
    }
    /// Set a `status_code`
    pub fn set(&mut self, status_code: std::ffi::c_int) {
        self.0 |= status_code;
    }
    pub(crate) fn as_raw(&self) -> std::ffi::c_int {
        self.0
    }

    pub(crate) fn from_raw(raw: std::ffi::c_int) -> Self {
        Self(raw)
    }
}

/// Required fields to update a clock
#[derive(Debug, Clone, Default)]
pub struct Timex {
    /// Mode selector for `adjtimex`.
    pub modes: TimexMode,
    /// Time offset; nanoseconds if `STA_NANO` is set, otherwise microseconds.
    pub offset: std::ffi::c_longlong,
    /// Frequency offset (see `man 2 adjtimex` for units).
    pub freq: std::ffi::c_longlong,
    /// Maximum error in microseconds.
    pub maxerror: std::ffi::c_longlong,
    /// Estimated error in microseconds.
    pub esterror: std::ffi::c_longlong,
    /// Clock command/status flags.
    pub status: StatusCodes,
    /// PLL (phase-locked loop) time constant.
    pub constant: std::ffi::c_longlong,
    /// Clock precision (microseconds, read-only).
    pub precision: std::ffi::c_longlong,
    /// Clock frequency tolerance (read-only).
    pub tolerance: std::ffi::c_longlong,
    /// Current time (read-only except for `ADJ_SETOFFSET`).
    ///
    /// If `STA_NANO` is set, `time.tv_usec` contains nanoseconds;
    /// otherwise microseconds.
    pub time: Timeval,
    /// Microseconds between clock ticks.
    pub tick: std::ffi::c_longlong,
    /// PPS (pulse-per-second) frequency (read-only).
    pub ppsfreq: std::ffi::c_longlong,
    /// PPS jitter (nanoseconds if `STA_NANO` is set, otherwise microseconds).
    pub jitter: std::ffi::c_longlong,
    /// PPS interval duration (seconds, read-only).
    pub shift: std::ffi::c_longlong,
    /// PPS stability (read-only).
    pub stabil: std::ffi::c_longlong,
    /// Count of PPS jitter limit exceeded events (read-only).
    pub jitcnt: std::ffi::c_longlong,
    /// Count of PPS calibration intervals (read-only).
    pub calcnt: std::ffi::c_longlong,
    /// Count of PPS calibration errors (read-only).
    pub errcnt: std::ffi::c_longlong,
    /// Count of PPS stability limit exceeded events (read-only).
    pub stbcnt: std::ffi::c_longlong,
    /// TAI offset, as set by previous `ADJ_TAI` operation
    /// (seconds, read-only, since Linux 2.6.26).
    pub tai: std::ffi::c_int,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_time() {
        let time = get_time(ClockId::ClockBoottime).unwrap();
        assert!(time.tv_sec > 0);
    }

    #[test]
    fn test_adjust_time() {
        let mut tx = Timex::default();
        adjust_time(ClockId::ClockRealtime, &mut tx).unwrap();
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
}
