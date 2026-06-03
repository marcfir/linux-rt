use windows::Win32::System::{
    Threading::{
        GetPriorityClass, GetThreadPriority, SetPriorityClass, SetThreadAffinityMask,
        SetThreadPriority, BELOW_NORMAL_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS,
        REALTIME_PRIORITY_CLASS, THREAD_PRIORITY, THREAD_PRIORITY_ABOVE_NORMAL,
        THREAD_PRIORITY_BELOW_NORMAL, THREAD_PRIORITY_HIGHEST, THREAD_PRIORITY_IDLE,
        THREAD_PRIORITY_LOWEST, THREAD_PRIORITY_NORMAL, THREAD_PRIORITY_TIME_CRITICAL,
    },
    WindowsProgramming::THREAD_PRIORITY_ERROR_RETURN,
};

use crate::sched::{Attributes, CpuSet, Pid, Policy, SchedFlags};

pub fn get_attr(pid: Pid) -> Result<Attributes, std::io::Error> {
    let policy_win = unsafe { GetPriorityClass(pid.process_handle) };
    if policy_win == 0 {
        return Err(std::io::Error::last_os_error());
    }
    let policy = match policy_win {
        0x00000100 => Policy::Fifo,
        0x00000040 => Policy::Idle,
        0x00004000 => Policy::Batch,
        _ => Policy::Normal,
    };
    let win_priority = unsafe { GetThreadPriority(pid.thread_handle) };
    if win_priority as u32 == THREAD_PRIORITY_ERROR_RETURN {
        return Err(std::io::Error::last_os_error());
    }
    let (priority, nice) = match policy {
        Policy::Fifo | Policy::RoundRobin | Policy::Deadline => {
            (win_to_rt_priority(THREAD_PRIORITY(win_priority)), 0)
        }
        _ => (0, win_to_nice(THREAD_PRIORITY(win_priority))),
    };

    Ok(Attributes {
        policy,
        flags: SchedFlags::empty(),
        nice,
        priority,
        runtime_ns: 0,
        deadline_ns: 0,
        period_ns: 0,
        sched_util_min: 0,
        sched_util_max: 0,
    })
}

pub fn set_attr(pid: Pid, attr: Attributes) -> Result<(), std::io::Error> {
    match attr.policy {
        Policy::Ext | Policy::Normal => {
            unsafe { SetPriorityClass(pid.process_handle, NORMAL_PRIORITY_CLASS)? };
        }
        Policy::Batch => {
            unsafe { SetPriorityClass(pid.process_handle, BELOW_NORMAL_PRIORITY_CLASS)? };
        }
        Policy::Idle => {
            unsafe { SetPriorityClass(pid.process_handle, IDLE_PRIORITY_CLASS)? };
        }
        Policy::Fifo | Policy::RoundRobin | Policy::Deadline => {
            unsafe { SetPriorityClass(pid.process_handle, REALTIME_PRIORITY_CLASS)? };
        }
    }
    match attr.policy {
        Policy::Fifo | Policy::RoundRobin | Policy::Deadline => {
            unsafe { SetThreadPriority(pid.thread_handle, rt_priority_to_win(attr.priority))? };
        }
        _ => {
            unsafe { SetThreadPriority(pid.thread_handle, nice_to_win(attr.nice))? };
        }
    }

    Ok(())
}

pub fn get_priority_max(_pol: Policy) -> Result<isize, std::io::Error> {
    Ok(15)
}

pub fn get_priority_min(_pol: Policy) -> Result<isize, std::io::Error> {
    Ok(-15)
}

pub fn sched_yield() -> Result<(), std::io::Error> {
    todo!()
}

pub fn set_affinity(pid: Pid, set: CpuSet) -> Result<(), std::io::Error> {
    if unsafe { SetThreadAffinityMask(pid.thread_handle, set.bits[0] as usize) } == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn get_affinity(pid: Pid) -> Result<CpuSet, std::io::Error> {
    let ret = unsafe { SetThreadAffinityMask(pid.thread_handle, 1) };
    if ret == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        let ret2 = unsafe { SetThreadAffinityMask(pid.thread_handle, ret) };
        if ret2 == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(CpuSet::from_bitmask(ret as u64))
    }
}

fn nice_to_win(nice: i32) -> THREAD_PRIORITY {
    match nice.clamp(-20, 19) {
        -20..=-14 => THREAD_PRIORITY_TIME_CRITICAL,
        -13..=-8 => THREAD_PRIORITY_HIGHEST,
        -7..=-3 => THREAD_PRIORITY_ABOVE_NORMAL,
        -2..=2 => THREAD_PRIORITY_NORMAL,
        3..=7 => THREAD_PRIORITY_BELOW_NORMAL,
        8..=13 => THREAD_PRIORITY_LOWEST,
        _ => THREAD_PRIORITY_IDLE,
    }
}
fn win_to_nice(win: THREAD_PRIORITY) -> i32 {
    match win {
        THREAD_PRIORITY_TIME_CRITICAL => -20,
        THREAD_PRIORITY_HIGHEST => -13,
        THREAD_PRIORITY_ABOVE_NORMAL => -7,
        THREAD_PRIORITY_NORMAL => 0,
        THREAD_PRIORITY_BELOW_NORMAL => 7,
        THREAD_PRIORITY_LOWEST => 13,
        THREAD_PRIORITY_IDLE => 19,
        _ => unreachable!(),
    }
}
fn rt_priority_to_win(p: u32) -> THREAD_PRIORITY {
    match p.clamp(1, 99) {
        1..=14 => THREAD_PRIORITY_IDLE,
        15..=28 => THREAD_PRIORITY_LOWEST,
        29..=43 => THREAD_PRIORITY_BELOW_NORMAL,
        44..=57 => THREAD_PRIORITY_NORMAL,
        58..=71 => THREAD_PRIORITY_ABOVE_NORMAL,
        72..=85 => THREAD_PRIORITY_HIGHEST,
        _ => THREAD_PRIORITY_TIME_CRITICAL,
    }
}
fn win_to_rt_priority(win: THREAD_PRIORITY) -> u32 {
    match win {
        THREAD_PRIORITY_TIME_CRITICAL => 99,
        THREAD_PRIORITY_HIGHEST => 85,
        THREAD_PRIORITY_ABOVE_NORMAL => 71,
        THREAD_PRIORITY_NORMAL => 57,
        THREAD_PRIORITY_BELOW_NORMAL => 43,
        THREAD_PRIORITY_LOWEST => 28,
        THREAD_PRIORITY_IDLE => 14,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_nice_to_prio() {
        assert_eq!(nice_to_win(0), THREAD_PRIORITY(0));
        assert_eq!(nice_to_win(-20), THREAD_PRIORITY(15));
        assert_eq!(nice_to_win(19), THREAD_PRIORITY(-15));
        assert_eq!(nice_to_win(10), THREAD_PRIORITY(-2));
        assert_eq!(nice_to_win(-10), THREAD_PRIORITY(2));
    }
    #[test]
    fn test_prio_to_windows() {
        assert_eq!(rt_priority_to_win(1), THREAD_PRIORITY(-15));
        assert_eq!(rt_priority_to_win(99), THREAD_PRIORITY(15));
        assert_eq!(rt_priority_to_win(50), THREAD_PRIORITY(0));
        assert_eq!(rt_priority_to_win(0), THREAD_PRIORITY(-15));
        assert_eq!(rt_priority_to_win(100), THREAD_PRIORITY(15));
        assert_eq!(rt_priority_to_win(10), THREAD_PRIORITY(-15));
        assert_eq!(rt_priority_to_win(25), THREAD_PRIORITY(-2));
        assert_eq!(rt_priority_to_win(75), THREAD_PRIORITY(2));
        assert_eq!(rt_priority_to_win(90), THREAD_PRIORITY(15));
    }
}
