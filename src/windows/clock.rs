use std::time::Duration;

use crate::clock::{ClockId, TimeSpec};

pub fn get_time(clockid: ClockId) -> Result<TimeSpec, std::io::Error> {
    match clockid {
        ClockId::ClockRealtime => clock_gettime_realtime(),
        ClockId::ClockMonotonic => clock_gettime_monotonic(),
    }
}

pub fn nanosleep_relative(_clockid: ClockId, ts: TimeSpec) -> Result<(), std::io::Error> {
    std::thread::sleep(Duration::from_nanos(ts.as_nanoseconds() as u64));
    Ok(())
}
pub fn nanosleep_absolute(clockid: ClockId, ts: TimeSpec) -> Result<(), std::io::Error> {
    match clockid {
        ClockId::ClockRealtime => sleep_absolute_realtime(ts),
        ClockId::ClockMonotonic => sleep_absolute_monotonic(ts),
    }
}
pub fn nanosleep_relative_with_remain(
    clockid: ClockId,
    ts: TimeSpec,
) -> Result<TimeSpec, std::io::Error> {
    nanosleep_relative(clockid, ts).and(Ok(TimeSpec::zeroed()))
}
pub fn nanosleep_absolute_with_remain(
    clockid: ClockId,
    ts: TimeSpec,
) -> Result<TimeSpec, std::io::Error> {
    nanosleep_absolute(clockid, ts).and(Ok(TimeSpec::zeroed()))
}

#[inline]
pub fn clock_gettime_monotonic() -> Result<TimeSpec, std::io::Error> {
    let mut freq = 0;
    (unsafe { windows::Win32::System::Performance::QueryPerformanceFrequency(&raw mut freq) })?;

    let mut now = 0;
    (unsafe { windows::Win32::System::Performance::QueryPerformanceCounter(&raw mut now) })?;

    // seconds
    let secs = now / freq;

    // remainder in ticks
    let rem = now % freq;

    // convert remainder to nanoseconds
    let nanos = (rem * 1_000_000_000) / freq;

    Ok(TimeSpec {
        tv_sec: secs,
        tv_nsec: nanos,
    })
}

#[inline]
pub fn clock_gettime_realtime() -> Result<TimeSpec, std::io::Error> {
    let ft = unsafe { windows::Win32::System::SystemInformation::GetSystemTimePreciseAsFileTime() };

    // FILETIME = 100-nanosecond intervals since Jan 1, 1601 (UTC)
    let ticks = ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64);

    // Convert to Unix epoch (Jan 1, 1970)
    const EPOCH_DIFF: u64 = 11644473600; // seconds
    let secs = ticks / 10_000_000 - EPOCH_DIFF;

    let nanos = (ticks % 10_000_000) * 100;

    Ok(TimeSpec {
        tv_sec: secs as i64,
        tv_nsec: nanos as i64,
    })
}

#[inline]
pub fn sleep_absolute_realtime(ts: TimeSpec) -> Result<(), std::io::Error> {
    let now = clock_gettime_realtime()?;
    let relative_sleep = (ts - now).as_nanoseconds().max(0) as u64;
    std::thread::sleep(Duration::from_nanos(relative_sleep));
    Ok(())
}

#[inline]
pub fn sleep_absolute_monotonic(ts: TimeSpec) -> Result<(), std::io::Error> {
    let now = clock_gettime_monotonic()?;
    let relative_sleep = (ts - now).as_nanoseconds().max(0) as u64;
    std::thread::sleep(Duration::from_nanos(relative_sleep));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_get_time() {
        let start = clock_gettime_monotonic().unwrap();
        std::thread::sleep(Duration::from_millis(100));
        let end = clock_gettime_monotonic().unwrap();
        let elapsed = (end - start).as_microseconds();
        assert!(elapsed >= 100_000, "{elapsed}");

        let start = clock_gettime_realtime().unwrap();
        std::thread::sleep(Duration::from_millis(100));
        let end = clock_gettime_realtime().unwrap();
        let elapsed = (end - start).as_microseconds();
        assert!(elapsed >= 100_000, "{elapsed}");

        println!("{:?}", clock_gettime_realtime());
    }

    #[test]
    fn test_sleep_absolute() {
        let start = clock_gettime_monotonic().unwrap();
        let target = start + TimeSpec::milliseconds(20);

        sleep_absolute_monotonic(start + TimeSpec::milliseconds(20)).unwrap();
        let end = clock_gettime_monotonic().unwrap();
        let elapsed = (end - start).as_microseconds();
        println!(
            "20ms={:?} {start:?} {target:?} {elapsed}",
            TimeSpec::milliseconds(20)
        );
        let to_sleep = start + TimeSpec::milliseconds(20);
        println!(
            "{start:?} -> {to_sleep:?}  {}",
            to_sleep.tv_nsec - start.tv_nsec
        );
        assert!(elapsed >= 20_000, "{elapsed}");
        println!("{elapsed}");

        // let start = clock_gettime_realtime().unwrap();
        // std::thread::sleep(Duration::from_millis(100));
        // let end = clock_gettime_realtime().unwrap();
        // let elapsed = (end - start).as_microseconds();
        // assert!(elapsed >= 100_000, "{elapsed}");

        // println!("{:?}", clock_gettime_realtime());
    }
}
