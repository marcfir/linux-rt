use bitflags::bitflags;
use std::fmt::Debug;

/// The [get_attr()] function wraps the `sched_getattr()` system call and fetches the scheduling policy and
/// the associated attributes for the thread whose ID is specified in pid.
pub fn get_attr(pid: Pid) -> Result<Attributes, std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::sched::get_attr(pid)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::sched::get_attr(pid)
    }
}

/// The [set_attr()] function wraps the `sched_setattr()` system call and sets the scheduling policy and
/// associated attributes for the thread whose ID is specified in pid.
pub fn set_attr(pid: Pid, attr: Attributes) -> Result<(), std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::sched::set_attr(pid, attr)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::sched::set_attr(pid, attr)
    }
}

/// Sets the scheduling policy with a `nice` value to other.
/// See [Attributes::nice] for more info.
pub fn set_other(pid: Pid, nice: i32) -> Result<(), std::io::Error> {
    let att_other = Attributes {
        policy: Policy::Normal,
        nice,
        deadline_ns: 0,
        period_ns: 0,
        flags: SchedFlags::empty(),
        priority: 0,
        runtime_ns: 0,
        sched_util_min: 0,
        sched_util_max: 0,
    };
    set_attr(pid, att_other)
}
pub fn set_batch(pid: Pid, nice: i32) -> Result<(), std::io::Error> {
    let att_batch = Attributes {
        policy: Policy::Batch,
        nice,
        deadline_ns: 0,
        period_ns: 0,
        flags: SchedFlags::empty(),
        priority: 0,
        runtime_ns: 0,
        sched_util_min: 0,
        sched_util_max: 0,
    };
    set_attr(pid, att_batch)
}
pub fn set_idle(pid: Pid) -> Result<(), std::io::Error> {
    let att_batch = Attributes {
        policy: Policy::Idle,
        nice: 0,
        deadline_ns: 0,
        period_ns: 0,
        flags: SchedFlags::empty(),
        priority: 0,
        runtime_ns: 0,
        sched_util_min: 0,
        sched_util_max: 0,
    };
    set_attr(pid, att_batch)
}
pub fn set_fifo(pid: Pid, priority: u32) -> Result<(), std::io::Error> {
    let att_batch = Attributes {
        policy: Policy::Fifo,
        nice: 0,
        deadline_ns: 0,
        period_ns: 0,
        flags: SchedFlags::empty(),
        priority,
        runtime_ns: 0,
        sched_util_min: 0,
        sched_util_max: 0,
    };
    set_attr(pid, att_batch)
}
pub fn set_rr(pid: Pid, priority: u32) -> Result<(), std::io::Error> {
    let att_batch = Attributes {
        policy: Policy::RoundRobin,
        nice: 0,
        deadline_ns: 0,
        period_ns: 0,
        flags: SchedFlags::empty(),
        priority,
        runtime_ns: 0,
        sched_util_min: 0,
        sched_util_max: 0,
    };
    set_attr(pid, att_batch)
}
pub fn set_deadline(
    pid: Pid,
    deadline_ns: u64,
    period_ns: u64,
    runtime_ns: u64,
) -> Result<(), std::io::Error> {
    if !((runtime_ns <= deadline_ns) && (deadline_ns <= period_ns)) {
        println!("Error: params are not sched_runtime <= sched_deadline <= sched_period!");
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
    };
    if runtime_ns < 1024 || deadline_ns < 1024 || period_ns < 1024 {
        println!("Error: params are y1024");
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
    }
    let att_batch = Attributes {
        policy: Policy::Deadline,
        nice: 0,
        deadline_ns,
        period_ns,
        flags: SchedFlags::empty(),
        priority: 0,
        runtime_ns,
        sched_util_min: 0,
        sched_util_max: 0,
    };
    set_attr(pid, att_batch)
}

pub fn get_priority_max(pol: Policy) -> Result<isize, std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::sched::get_priority_max(pol)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::sched::get_priority_max(pol)
    }
}

pub fn get_priority_min(pol: Policy) -> Result<isize, std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::sched::get_priority_min(pol)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::sched::get_priority_min(pol)
    }
}

pub fn sched_yield() -> Result<(), std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::sched::sched_yield()
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::sched::sched_yield()
    }
}

pub fn set_affinity(pid: Pid, set: CpuSet) -> Result<(), std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::sched::set_affinity(pid, set)
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::sched::set_affinity(pid, set)
    }
}

pub fn get_affinity(pid: Pid) -> Result<CpuSet, std::io::Error> {
    #[cfg(target_family = "unix")]
    {
        crate::linux::sched::get_affinity()
    }
    #[cfg(target_os = "windows")]
    {
        crate::windows::sched::get_affinity(pid)
    }
}

/// Currently, Linux supports the scheduling policies defined in this enum.
#[derive(Debug, Clone, PartialEq)]
pub enum Policy {
    ///The standard round-robin time-sharing policy
    Normal,
    /// For "batch" style execution of processes
    Batch,
    /// For running very low priority background jobs
    Idle,
    /// Various "real-time" policies are also supported, for special
    /// time-critical applications that need precise control over the way
    /// in which runnable threads are selected for execution.
    /// The real-time policies that may be specified in policy
    /// are:a first-in, first-out policy; and
    Fifo,
    /// a round-robin policy.
    RoundRobin,
    /// a deadline scheduling policy;
    Deadline,
    Ext,
}
#[cfg(target_family = "unix")]
impl Policy {
    /// Consume the policy to convert it into a raw value.
    pub fn into_raw(self) -> u32 {
        self.as_raw()
    }

    /// Get the policy as raw value.
    pub fn as_raw(&self) -> u32 {
        match self {
            Policy::Batch => crate::linux::sched::SCHED_BATCH,
            Policy::Deadline => crate::linux::sched::SCHED_DEADLINE,
            Policy::Fifo => crate::linux::sched::SCHED_FIFO,
            Policy::Idle => crate::linux::sched::SCHED_IDLE,
            Policy::Normal => crate::linux::sched::SCHED_NORMAL,
            Policy::RoundRobin => crate::linux::sched::SCHED_RR,
            Policy::Ext => crate::linux::sched::SCHED_EXT,
        }
    }

    /// Create a [Policy] from a raw value.
    pub fn from_raw(raw: u32) -> Result<Policy, Error> {
        match raw {
            crate::linux::sched::SCHED_NORMAL => Ok(Policy::Normal),
            crate::linux::sched::SCHED_FIFO => Ok(Policy::Fifo),
            crate::linux::sched::SCHED_RR => Ok(Policy::RoundRobin),
            crate::linux::sched::SCHED_BATCH => Ok(Policy::Batch),
            crate::linux::sched::SCHED_IDLE => Ok(Policy::Idle),
            crate::linux::sched::SCHED_DEADLINE => Ok(Policy::Deadline),
            crate::linux::sched::SCHED_EXT => Ok(Policy::Ext),
            _ => Err(Error),
        }
    }
}

bitflags! {
    /// These flags control the scheduling behavior
    #[derive(Debug, Clone, PartialEq)]
    pub struct SchedFlags: std::ffi::c_short {
        /// Children created by fork(2) do not inherit
        /// privileged scheduling policies. See sched(7) for
        /// details.
        const SCHED_FLAG_RESET_ON_FORK = 0x01;
        ///This flag allows a SCHED_DEADLINE thread to reclaim bandwidth unused by other real-time threads.
        const SCHED_FLAG_RECLAIM = 0x02;
        /// his flag allows an application to get informed
        /// about run-time overruns in SCHED_DEADLINE threads.
        /// Such overruns may be caused by (for example) coarse
        /// execution time accounting or incorrect parameter
        /// assignment. Notification takes the form of a
        /// SIGXCPU signal which is generated on each overrun.
        /// This SIGXCPU signal is process-directed (see
        /// signal(7)) rather than thread-directed. This is
        /// probably a bug. On the one hand, sched_setattr()
        /// is being used to set a per-thread attribute. On
        /// the other hand, if the process-directed signal is
        /// delivered to a thread inside the process other than
        /// the one that had a run-time overrun, the
        /// application has no way of knowing which thread
        /// overran.
        const SCHED_FLAG_DL_OVERRUN = 0x04;
        const SCHED_FLAG_KEEP_PARAMS = 0x10;
        /// These flags indicate that the sched_util_min or
        /// sched_util_max fields, respectively, are present,
        /// representing the expected minimum and maximum
        /// utilization of the thread.
        ///
        /// The utilization attributes provide the scheduler
        /// with boundaries within which it should schedule the
        /// thread, potentially informing its decisions
        /// regarding task placement and frequency selection.
        const SCHED_FLAG_UTIL_CLAMP_MIN = 0x20;
        /// See SCHED_FLAG_UTIL_CLAMP_MIN
        const SCHED_FLAG_UTIL_CLAMP_MAX	= 0x40;
    }
}

///Structure containing the scheduling policy and attributes for the specified thread.
#[derive(Debug, Clone, PartialEq)]
pub struct Attributes {
    /// This field specifies the scheduling policy, as one of the values of the enum.
    pub policy: Policy,
    /// These flags control scheduling behavior:
    pub flags: SchedFlags,
    /// This field specifies the nice value to be set when
    /// specifying sched_policy as `SCHED_OTHER` or `SCHED_BATCH`.
    /// The nice value is a number in the range -20 (high
    /// priority) to +19 (low priority); see sched(7).
    pub nice: i32,
    /// This field specifies the static priority to be set when specifying `policy` as
    /// `Fifo` or `RoundRobin`. The allowed range of priorities for these policies can be
    /// determined using sched_get_priority_min(2) and sched_get_priority_max(2). For
    /// other `policy`, this field must be specified as 0.
    pub priority: u32,
    /// This field specifies the "Runtime" parameter for deadline scheduling. The value is
    /// expressed in nanoseconds. This field is used only for the `Deadline` `policy`.
    pub runtime_ns: u64,
    /// This field specifies the "Deadline" parameter for deadline scheduling. The value
    /// is expressed in nanoseconds. This field is used only for the `Deadline` `policy`.
    pub deadline_ns: u64,
    /// This field specifies the "Period" parameter for deadline scheduling. The value is
    /// expressed in nanoseconds. This field is used only for the `Deadline` `policy`.
    pub period_ns: u64,

    /// These fields specify the expected minimum and maximum utilization, respectively. They are ignored
    /// unless their corresponding SCHED_FLAG_UTIL_CLAMP_MIN or SCHED_FLAG_UTIL_CLAMP_MAX is set in sched_flags.
    ///
    /// Utilization is a value in the range [0, 1024], representing the percentage of CPU time used by a
    /// task when running at the maximum frequency on the highest capacity CPU of the system. This is a
    /// fixed point representation, where 1024 corresponds to 100%, and 0 corresponds to 0%. For example,
    /// a 20% utilization task is a task running for 2ms every 10ms at maximum frequency and is
    /// represented by a utilization value of 0.2 * 1024 = 205.
    ///
    /// A task with a minimum utilization value larger than 0 is more likely scheduled on a CPU with a
    /// capacity big enough to fit the specified value. A task with a maximum utilization value smaller
    /// than 1024 is more likely scheduled on a CPU with no more capacity than the specified value.
    ///
    /// A task utilization boundary can be reset by setting its field to UINT32_MAX (since Linux 5.11)
    pub sched_util_min: u32,
    /// See `sched_util_min`
    pub sched_util_max: u32,
}

/// Process identifier.
/// Newtype arround `pid_t`
#[cfg(target_family = "unix")]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pid(libc::pid_t);
/// Process identifier.
/// Newtype arround `pid_t`
#[cfg(target_os = "windows")]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pid {
    pub(crate) process_handle: windows::Win32::Foundation::HANDLE,
    pub(crate) thread_handle: windows::Win32::Foundation::HANDLE,
}

#[cfg(target_family = "unix")]
impl Pid {
    /// Gets a raw `pid_t` from a [Pid]
    pub fn as_raw(&self) -> libc::pid_t {
        self.0
    }
    /// Creates a [Pid] from a raw `pid_t`
    pub fn from_raw(raw: libc::pid_t) -> Self {
        Self(raw)
    }
}

impl Pid {
    /// Returns the [Pid] of the calling process/thread
    pub fn this() -> Self {
        #[cfg(target_family = "unix")]
        {
            Self(0)
        }
        #[cfg(target_os = "windows")]
        {
            Self {
                process_handle: unsafe { windows::Win32::System::Threading::GetCurrentProcess() },
                thread_handle: unsafe { windows::Win32::System::Threading::GetCurrentThread() },
            }
        }
    }
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
#[derive(PartialEq, Clone)]
pub struct CpuSet {
    pub(crate) bits: [Map; CPU_SET_SIZE],
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

    /// Create a [CpuSet] from a slice of cores
    pub fn from_slice(slice_of_cores: impl AsRef<[usize]>) -> Self {
        let mut cpuset = CpuSet::empty();
        slice_of_cores.as_ref().iter().for_each(|core| {
            cpuset.set(*core);
        });
        cpuset
    }

    /// Create a full [CpuSet]
    pub const fn full() -> Self {
        Self {
            bits: [Map::MAX; CPU_SET_SIZE],
        }
    }
    #[cfg(target_family = "unix")]
    pub(crate) const fn as_raw(&self) -> *const CpuSet {
        self
    }
    #[cfg(target_family = "unix")]
    pub(crate) const fn as_mut_raw(&mut self) -> *mut CpuSet {
        self
    }

    /// Add CPU `core` to the [CpuSet] using the builder pattern.
    pub const fn insert(self, core: usize) -> Self {
        let mut cs = self;
        cs.set(core);
        cs
    }

    /// Add CPU `core` to the [CpuSet].
    pub const fn set(&mut self, core: usize) {
        let idx = core / Map::BITS as usize;
        let bit = core % Map::BITS as usize;
        self.bits[idx] |= 1 << bit;
    }

    /// Clear CPU `core` from the [CpuSet] using the builder pattern.
    pub const fn remove(self, core: usize) -> Self {
        let mut cs = self;
        cs.clear(core);
        cs
    }

    /// Clear CPU `core` from the [CpuSet].
    pub const fn clear(&mut self, core: usize) {
        let idx = core / Map::BITS as usize;
        let bit = core % Map::BITS as usize;
        self.bits[idx] &= !(1 << bit);
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

    /// Return the maximum number of CPU in CpuSet
    pub const fn count() -> usize {
        Self::size_of() * 8
    }

    pub fn used_cores(&self) -> Vec<usize> {
        let mut cores = Vec::new();

        for core in 0..CPU_SET_SIZE * Map::BITS as usize {
            if self.is_set(core) {
                cores.push(core);
            }
        }

        cores
    }
}

impl Debug for CpuSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuSet")
            .field("cores", &self.used_cores())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::sched::*;

    #[test]
    fn test_setattr() {
        let attr = Attributes {
            policy: Policy::Batch,
            nice: 7,
            deadline_ns: 0,
            period_ns: 0,
            flags: SchedFlags::empty(),
            priority: 0,
            runtime_ns: 0,
            sched_util_min: 0,
            sched_util_max: 0,
        };
        println!("{attr:?}");
        set_attr(Pid::this(), attr).unwrap();
        let a = get_attr(Pid::this()).unwrap();

        assert_eq!(a.policy, Policy::Batch);
        assert_eq!(a.nice, 7);
    }
    #[test]
    fn test_setter() {
        let res = set_other(Pid::this(), -20);
        if res.is_ok() {
            let a = get_attr(Pid::this()).unwrap();
            assert_eq!(a.policy, Policy::Normal);
            assert_eq!(a.nice, -20);
        }

        set_batch(Pid::this(), 19).unwrap();
        let a = get_attr(Pid::this()).unwrap();
        assert_eq!(a.policy, Policy::Batch);
        assert_eq!(a.nice, 19);

        set_idle(Pid::this()).unwrap();
        let a = get_attr(Pid::this()).unwrap();
        assert_eq!(a.policy, Policy::Idle);

        #[cfg(target_family = "unix")]
        {
            let res = set_fifo(Pid::this(), 60);
            if res.is_ok() {
                let a = get_attr(Pid::this()).unwrap();
                assert_eq!(a.policy, Policy::Fifo);
                assert_eq!(a.nice, 0);
                assert_eq!(a.priority, 60);
            }

            let res = set_rr(Pid::this(), 98);
            if res.is_ok() {
                let a = get_attr(Pid::this()).unwrap();
                assert_eq!(a.policy, Policy::RoundRobin);
                assert_eq!(a.nice, 0);
                assert_eq!(a.priority, 98);
            }
            let res = set_deadline(Pid::this(), 1_000_000, 1_000_000, 50_000);
            if res.is_ok() {
                let a = get_attr(Pid::this()).unwrap();
                assert_eq!(a.policy, Policy::Deadline);
                assert_eq!(a.nice, 0);
                assert_eq!(a.priority, 0);
                assert_eq!(a.deadline_ns, 1_000_000);
                assert_eq!(a.period_ns, 1_000_000);
                assert_eq!(a.runtime_ns, 50_000);
            }
        }
    }

    #[test]
    fn test_prio() {
        get_priority_max(Policy::Fifo).unwrap();
        get_priority_min(Policy::Fifo).unwrap();
    }

    #[test]
    fn test_affinity() {
        let set = get_affinity(Pid::this()).unwrap();
        set_affinity(Pid::this(), set.clone()).unwrap();
        let set = set.insert(0);
        set_affinity(Pid::this(), set).unwrap();
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

        let test = CpuSet::empty().insert(1);

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

        let test = CpuSet::from_slice([1, 2]);

        assert_eq!(
            test,
            CpuSet {
                #[cfg(not(target_pointer_width = "32"))]
                bits: [6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                #[cfg(target_pointer_width = "32")]
                bits: [
                    6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
        .remove(0);
        assert_eq!(test.bits[0], 0xFFFE);
        let test = test.remove(7).remove(9).remove(8).remove(4);
        assert_eq!(test.bits[0], 0xFC6E);
        #[cfg(not(target_pointer_width = "32"))]
        assert_eq!(CpuSet::empty().insert(63).bits[0], 0x8000000000000000);
        #[cfg(target_pointer_width = "32")]
        assert_eq!(CpuSet::empty().set(63).bits[1], 0x80000000);

        #[cfg(not(target_pointer_width = "32"))]
        assert_eq!(CpuSet::empty().insert(64).bits[1], 1);
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
    fn test_cpuset_usedcores() {
        let test = CpuSet {
            #[cfg(not(target_pointer_width = "32"))]
            bits: [0x66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            #[cfg(target_pointer_width = "32")]
            bits: [
                0x66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ],
        };
        assert_eq!(test.used_cores(), vec![1, 2, 5, 6])
    }

    #[test]
    fn test_cpuset_debug() {
        let cpuset = CpuSet {
            #[cfg(not(target_pointer_width = "32"))]
            bits: [0x66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            #[cfg(target_pointer_width = "32")]
            bits: [
                0x66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0,
            ],
        };

        let output = format!("{:?}", cpuset);

        assert_eq!(output, "CpuSet { cores: [1, 2, 5, 6] }");
    }
}
