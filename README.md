# rt_tunex

**rt_tunex** is a cross-platform systems crate for tuning low-latency and real-time applications in Rust.

It provides a linux-like abstraction over platform-specific primitives such as:
- CPU scheduling configuration
- High-resolution and monotonic clock access
- Memory mapping (`mmap`-style APIs)


Supported platforms:
- [x] Linux (full feature set)
- [x] Windows (best-effort real-time and memory control abstractions)
- [ ] planned: macOS (POSIX-aligned fallback implementations)

> ⚠️ This crate is intended for systems programming and real-time workloads. Some features require elevated privileges or platform-specific permissions.

## Why rt_tunex?

Hard real-time systems require:
- predictable scheduling behavior
- controlled memory allocation patterns
- stable timing sources
- minimal OS jitter

Unfortunately, on Linux these APIs are not well exposed in the Rust ecosystem, and writing cross-platform code that also targets Windows and macOS is challenging.

**rt_tunex** addresses this by unifying these primitives into a single ergonomic, cross-platform Rust interface.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rt_tunex = "..."
```

## Usage

```rust
use rt_tunex::sched::{Pid, CpuSet, set_fifo, set_affinity};
use rt_tunex::mman::{mlockall, MmanFlags};
use rt_tunex::clock::{get_time, nanosleep_relative, ClockId, TimeSpec};

// Set FIFO scheduling for calling thead
set_fifo(Pid::this(), 80);

// Pin calling thread to core 0
set_affinity(Pid::this(), CpuSet::empty().insert(0));

// Pin all memory to RAM
mlockall(MmanFlags::MCL_CURRENT | MmanFlags::MCL_FUTURE);

// Measure time with ClockMonotonic
let start = get_time(ClockId::ClockMonotonic).unwrap();
let end = get_time(ClockId::ClockMonotonic).unwrap();
let duration_ms = (end - start).as_milliseconds();

// Sleep for 20ms
nanosleep_relative(ClockId::ClockMonotonic, TimeSpec::milliseconds(20));

```