use crate::{
    linux::error::{ret_into_result, ret_to_result},
    sched::{Attributes, CpuSet, Pid, Policy, SchedFlags},
};
use std::{ffi::c_int, fmt::Debug};
use syscalls::{syscall, Errno, Sysno};

#[allow(non_camel_case_types)]
pub type pid_t = std::ffi::c_int;

pub const SCHED_NORMAL: u32 = 0;
pub const SCHED_FIFO: u32 = 1;
pub const SCHED_RR: u32 = 2;
pub const SCHED_BATCH: u32 = 3;
pub const SCHED_IDLE: u32 = 5;
pub const SCHED_DEADLINE: u32 = 6;
pub const SCHED_EXT: u32 = 7;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SchedAttr {
    /// Size of this structure
    pub size: u32, /* Size of this structure */
    /// Policy (SCHED_*)
    pub sched_policy: u32,
    /// Flags
    pub sched_flags: u64,
    /// Nice value (SCHED_OTHER, SCHED_BATCH)
    pub sched_nice: i32,

    /// Static priority (SCHED_FIFO, SCHED_RR)
    pub sched_priority: u32,

    /// For SCHED_DEADLINE
    pub sched_runtime: u64,
    /// For SCHED_DEADLINE
    pub sched_deadline: u64,
    /// For SCHED_DEADLINE
    pub sched_period: u64,

    /// Utilization hints
    pub sched_util_min: u32,
    /// Utilization hints
    pub sched_util_max: u32,
}
/// The sched_setattr() system call sets the scheduling policy and
/// associated attributes for the thread whose ID is specified in
/// `pid`. If `pid` equals zero, the scheduling policy and attributes of
/// the calling thread will be set.
#[allow(clippy::missing_safety_doc)]
pub unsafe fn sched_set_attr(pid: pid_t, attr: *mut SchedAttr, flags: u32) -> Result<usize, Errno> {
    syscall!(Sysno::sched_setattr, pid, attr, flags)
}
#[allow(clippy::missing_safety_doc)]
pub unsafe fn sched_get_attr(
    pid: pid_t,
    attr: *mut SchedAttr,
    size: u32,
    flags: u32,
) -> Result<usize, Errno> {
    syscall!(Sysno::sched_getattr, pid, attr, size, flags)
}

/// The [get_attr()] function wraps the `sched_getattr()` system call and fetches the scheduling policy and
/// the associated attributes for the thread whose ID is specified in pid.
pub fn get_attr(pid: Pid) -> Result<Attributes, std::io::Error> {
    let mut attr = SchedAttr {
        size: 0,
        sched_policy: 0,
        sched_flags: 0,
        sched_nice: 0,
        sched_priority: 0,
        sched_runtime: 0,
        sched_deadline: 0,
        sched_period: 0,
        sched_util_min: 0,
        sched_util_max: 0,
    };

    let ret = unsafe {
        sched_get_attr(
            pid.as_raw(),
            &mut attr,
            core::mem::size_of::<SchedAttr>() as u32,
            0,
        )
    };
    match ret {
        Ok(_) => {
            let a = Attributes {
                policy: Policy::from_raw(attr.sched_policy).unwrap(),
                flags: SchedFlags::from_bits_truncate(attr.sched_flags as i16),
                nice: attr.sched_nice,
                priority: attr.sched_priority,
                deadline_ns: attr.sched_deadline,
                period_ns: attr.sched_period,
                runtime_ns: attr.sched_runtime,
                sched_util_min: attr.sched_util_min,
                sched_util_max: attr.sched_util_max,
            };
            Ok(a)
        }
        Err(_) => Err(std::io::Error::last_os_error()),
    }
}

/// The [set_attr()] function wraps the `sched_setattr()` system call and sets the scheduling policy and
/// associated attributes for the thread whose ID is specified in pid.
pub fn set_attr(pid: Pid, attr: Attributes) -> Result<(), std::io::Error> {
    let mut attr = SchedAttr {
        size: core::mem::size_of::<SchedAttr>() as u32,
        sched_policy: attr.policy.into_raw(),
        sched_flags: attr.flags.bits() as u64,
        sched_nice: attr.nice,
        sched_priority: attr.priority,
        sched_runtime: attr.runtime_ns,
        sched_deadline: attr.deadline_ns,
        sched_period: attr.period_ns,
        sched_util_min: attr.sched_util_min,
        sched_util_max: attr.sched_util_max,
    };

    unsafe { sched_set_attr(pid.as_raw(), &mut attr, 0) }
        .or(Err(std::io::Error::last_os_error()))
        .and(Ok(()))
}

pub fn get_priority_max(pol: Policy) -> Result<isize, std::io::Error> {
    let ret = unsafe { libc::sched_get_priority_max(pol.into_raw() as c_int) };
    ret_into_result(ret).map(|val| val as isize)
}

pub fn get_priority_min(pol: Policy) -> Result<isize, std::io::Error> {
    let ret = unsafe { libc::sched_get_priority_min(pol.into_raw() as c_int) };
    ret_into_result(ret).map(|val| val as isize)
}

pub fn sched_yield() -> Result<(), std::io::Error> {
    let ret = unsafe { libc::sched_yield() };
    ret_to_result(ret, ())
}

pub fn set_affinity(pid: Pid, set: CpuSet) -> Result<(), std::io::Error> {
    let ret = unsafe {
        libc::sched_setaffinity(
            pid.as_raw(),
            CpuSet::size_of(),
            set.as_raw() as *const libc::cpu_set_t,
        )
    };
    ret_to_result(ret, ())
}

pub fn get_affinity(pid: Pid) -> Result<CpuSet, std::io::Error> {
    let mut cpuset = CpuSet::empty();
    let ret = unsafe {
        libc::sched_getaffinity(
            pid.as_raw(),
            CpuSet::size_of(),
            cpuset.as_mut_raw() as *mut libc::cpu_set_t,
        )
    };
    ret_to_result(ret, cpuset)
}

#[cfg(test)]
mod test {

    use super::*;
    use std::mem;

    #[test]
    fn set_attr() {
        let mut attr = SchedAttr {
            size: mem::size_of::<SchedAttr>() as u32,
            sched_policy: SCHED_IDLE,
            sched_flags: 0,
            sched_nice: 0,
            sched_priority: 0,
            sched_runtime: 0,
            sched_deadline: 0,
            sched_period: 0,
            sched_util_min: 0,
            sched_util_max: 0,
        };
        let ret = unsafe { sched_set_attr(0, &mut attr, 0) };
        assert_eq!(ret, Ok(0));

        let mut attr2 = unsafe { mem::zeroed::<SchedAttr>() };
        let ret = unsafe { sched_get_attr(0, &mut attr2, mem::size_of::<SchedAttr>() as u32, 0) };
        assert_eq!(ret, Ok(0));
        assert_eq!(attr2.sched_policy, { SCHED_IDLE });
    }
}
