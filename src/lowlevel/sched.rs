use std::ffi::c_int;

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

#[cfg(target_pointer_width = "32")]
const CPU_SET_SIZE: usize = 32;
#[cfg(target_pointer_width = "32")]
type Map = u32;
#[cfg(not(target_pointer_width = "32"))]
const CPU_SET_SIZE: usize = 16;
#[cfg(not(target_pointer_width = "32"))]
type Map = u64;

/// A CPU affinity mask is represented by this structure.
#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct CpuSet {
    bits: [Map; CPU_SET_SIZE],
}
impl CpuSet {
    /// Create a empty [CpuSet]
    pub const fn empty() -> Self {
        Self {
            bits: [0; CPU_SET_SIZE],
        }
    }

    /// Create a [CpuSet] from a bitmask
    pub const fn from_bitmask(bitmask: u64) -> Self {
        let mut cpuset = CpuSet::empty();
        #[cfg(not(target_pointer_width = "32"))]
        {
            cpuset.bits[0] = bitmask;
        }
        #[cfg(target_pointer_width = "32")]
        {
            let bytes = bitmask.to_le_bytes();
            let low = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            let high = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
            cpuset.bits[0] = low;
            cpuset.bits[1] = high;
        }
        cpuset
    }

    /// Create a full [CpuSet]
    pub const fn full() -> Self {
        Self {
            bits: [Map::MAX; CPU_SET_SIZE],
        }
    }

    pub(crate) const fn as_raw(&self) -> *const CpuSet {
        self
    }

    pub(crate) const fn as_mut_raw(&mut self) -> *mut CpuSet {
        self
    }

    /// Add CPU `core` to the [CpuSet].
    pub const fn set(self, core: usize) -> Self {
        let mut cs = self;
        let idx = core / Map::BITS as usize;
        let bit = core % Map::BITS as usize;
        cs.bits[idx] |= 1 << bit;
        cs
    }

    /// Clear CPU `core` to the [CpuSet].
    pub const fn clear(self, core: usize) -> Self {
        let mut cs = self;
        let idx = core / Map::BITS as usize;
        let bit = core % Map::BITS as usize;
        cs.bits[idx] &= !(1 << bit);
        cs
    }

    /// Checks whether the `core` is set in the [CpuSet].
    pub const fn is_set(&self, core: usize) -> bool {
        let idx = core / Map::BITS as usize;
        let bit = core % Map::BITS as usize;
        self.bits[idx] & (1 << bit) > 0
    }

    /// Returns the size of [CpuSet] in bytes
    pub const fn size_of() -> usize {
        size_of::<Self>()
    }
}

/// Sets the CPU affinity mask of the thread whose
/// ID is pid to the value specified by mask.  If pid is zero, then
/// the calling thread is used.  The argument cpusetsize is the length
/// (in bytes) of the data pointed to by mask.  Normally this argument
/// would be specified as sizeof(cpu_set_t).
///
/// If the thread specified by pid is not currently running on one of
/// the CPUs specified in mask, then that thread is migrated to one of
/// the CPUs specified in mask.
///
/// On success, the raw sched_getaffinity() system call returns the
/// number of bytes placed copied into the mask buffer; this will be
/// the minimum of cpusetsize and the size (in bytes) of the cpumask_t
/// data type that is used internally by the kernel to represent the
/// CPU set bit mask.
#[allow(clippy::missing_safety_doc)]
pub unsafe fn sched_set_affinity(
    pid: pid_t,
    cpusetsize: usize,
    mask: *const CpuSet,
) -> Result<usize, Errno> {
    syscall!(Sysno::sched_setaffinity, pid, cpusetsize, mask)
}

/// writes the affinity mask of the thread whose
/// ID is pid into the cpu_set_t structure pointed to by mask.  The
/// cpusetsize argument specifies the size (in bytes) of mask.  If pid
/// is zero, then the mask of the calling thread is returned.
///
/// Returns the number of bytes written to mask
#[allow(clippy::missing_safety_doc)]
pub unsafe fn sched_get_affinity(
    pid: pid_t,
    cpusetsize: usize,
    mask: *mut CpuSet,
) -> Result<usize, Errno> {
    syscall!(Sysno::sched_getaffinity, pid, cpusetsize, mask)
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn sched_yield() -> Result<usize, Errno> {
    syscall!(Sysno::sched_yield)
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn sched_get_priority_min(policy: c_int) -> Result<usize, Errno> {
    syscall!(Sysno::sched_get_priority_min, policy)
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn sched_get_priority_max(policy: c_int) -> Result<usize, Errno> {
    syscall!(Sysno::sched_get_priority_max, policy)
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

    #[test]
    fn test_cpuset() {
        let test = CpuSet::full();

        assert_eq!(
            test,
            CpuSet {
                #[cfg(not(target_pointer_width = "32"))]
                bits: [u64::MAX; 16],
                #[cfg(target_pointer_width = "32")]
                bits: [u32::MAX; 32],
            },
        );

        let test = CpuSet::empty();
        assert_eq!(
            test,
            CpuSet {
                #[cfg(not(target_pointer_width = "32"))]
                bits: [0; 16],
                #[cfg(target_pointer_width = "32")]
                bits: [0; 32],
            }
        );

        let test = CpuSet::empty().set(1);

        assert_eq!(
            test,
            CpuSet {
                #[cfg(not(target_pointer_width = "32"))]
                bits: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                #[cfg(target_pointer_width = "32")]
                bits: [
                    2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0
                ],
            }
        );
    }

    #[test]
    fn test_cpuset_ops() {
        let test = CpuSet {
            #[cfg(not(target_pointer_width = "32"))]
            bits: [0xFFFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            #[cfg(target_pointer_width = "32")]
            bits: [
                0xFFFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0,
            ],
        }
        .clear(0);
        assert_eq!(test.bits[0], 0xFFFE);
        let test = test.clear(7).clear(9).clear(8).clear(4);
        assert_eq!(test.bits[0], 0xFC6E);
        #[cfg(not(target_pointer_width = "32"))]
        assert_eq!(CpuSet::empty().set(63).bits[0], 0x8000000000000000);
        #[cfg(target_pointer_width = "32")]
        assert_eq!(CpuSet::empty().set(63).bits[1], 0x80000000);

        #[cfg(not(target_pointer_width = "32"))]
        assert_eq!(CpuSet::empty().set(64).bits[1], 1);
        #[cfg(target_pointer_width = "32")]
        assert_eq!(CpuSet::empty().set(64).bits[3], 1);

        #[cfg(not(target_pointer_width = "32"))]
        assert_eq!(
            CpuSet::from_bitmask(0x808000841000410).bits[0],
            0x808000841000410
        );
        #[cfg(target_pointer_width = "32")]
        assert_eq!(
            CpuSet::from_bitmask(0x808000841000410).bits[0..=1],
            [0x41000410, 0x8080008]
        );
    }

    #[test]
    fn test_affinity() {
        let mut cs_libc = unsafe { std::mem::zeroed() };
        unsafe { libc::CPU_ZERO(&mut cs_libc) };
        let ret_libc =
            unsafe { libc::sched_getaffinity(0, size_of_val(&cs_libc), &mut cs_libc as *mut _) };
        assert_eq!(ret_libc, 0);

        let mut cs = CpuSet::empty();
        let ret = unsafe { sched_get_affinity(0, CpuSet::size_of(), cs.as_mut_raw()) };
        assert!(ret.is_ok());
        assert!(ret.unwrap() > 0); // Check whether the result reflects at least one byte being written.
        assert_eq!(
            unsafe { std::mem::transmute::<libc::cpu_set_t, [Map; CPU_SET_SIZE]>(cs_libc) },
            cs.bits
        );
    }
}
