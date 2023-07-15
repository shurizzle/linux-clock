use std::mem::MaybeUninit;

use crate::Errno;

#[cfg(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "watchos",
    target_os = "tvos"
))]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClockId {
    /// The system's real time (i.e. wall time) clock, expressed as the amount
    /// of time since the Epoch.  This is the same as the value returned by
    /// gettimeofday(2).
    Realtime = libc::CLOCK_REALTIME,

    /// Clock that increments monotonically, tracking the time since an
    /// arbitrary point like [`Monotonic`]. However, this clock is
    /// unaffected by frequency or time adjustments. It should not be compared
    /// to other system time sources.
    ///
    /// [`Monotonic`]: Self::Monotonic
    MonotonicRaw = libc::CLOCK_MONOTONIC_RAW,

    /// Like [`MonotonicRaw`], but reads a value cached by the system at
    /// context switch. This can be read faster, but at a loss of accuracy as
    /// it may return values that are milliseconds old.
    ///
    /// [`MonotonicRaw`]: Self::MonotonicRaw
    MonotonicRawApprox = libc::CLOCK_MONOTONIC_RAW_APPROX,

    /// Clock that increments monotonically, tracking the time since an
    /// arbitrary point, and will continue to increment while the system is
    /// asleep.
    Monotonic = libc::CLOCK_MONOTONIC,

    /// Clock that increments monotonically, in the same manner as
    /// [`MonotonicRaw`], but that does not increment while the system is
    /// asleep. The returned value is identical to the result of
    /// mach_absolute_time() after the appropriate mach_timebase conversion is
    /// applied.
    ///
    /// [`MonotonicRaw`]: Self::MonotonicRaw
    UptimeRaw = libc::CLOCK_UPTIME_RAW,

    // Like [`UptimeRaw`], but reads a value cached by the system at
    // context switch. This can be read faster, but at a loss of accuracy as
    // it may return values that are milliseconds old.
    ///
    /// [`UptimeRaw`]: Self::UptimeRaw
    UptimeRawApprox = libc::CLOCK_UPTIME_RAW_APPROX,

    /// Clock that tracks the amount of CPU (in user- or kernel-mode) used by
    /// the calling process.
    ProcessCputimeId = libc::CLOCK_PROCESS_CPUTIME_ID,

    /// Clock that tracks the amount of CPU (in user- or kernel-mode) used by
    /// the calling thread.
    ThreadCputimeId = libc::CLOCK_THREAD_CPUTIME_ID,
}

#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
#[cfg_attr(target_os = "freebsd", repr(i32))]
#[cfg_attr(target_os = "dragonfly", repr(u64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClockId {
    /// Increments as a wall clock should.
    Realtime = libc::CLOCK_REALTIME,
    /// Same as [Self::Realtime] but get the most exact value as possible, at the expense of execution time
    RealtimePrecise = libc::CLOCK_REALTIME_PRECISE,
    /// Same as [Self::Realtime] but do not perform a full time counter query, so the accuracy is one timer tick.
    RealtimeFast = libc::CLOCK_REALTIME_FAST,
    /// Increments in SI seconds.
    Monotonic = libc::CLOCK_MONOTONIC,
    /// Same as [Self::Monotonic] but get the most exact value as possible, at the expense of execution time
    MonotonicPrecise = libc::CLOCK_MONOTONIC_PRECISE,
    /// Same as [Self::Monotonic] but do not perform a full time counter query, so the accuracy is one timer tick.
    MonotonicFast = libc::CLOCK_MONOTONIC_FAST,
    /// Starts at zero when the kernel boots and increments monotonically in SI seconds while the machine is running.
    Uptime = libc::CLOCK_UPTIME,
    /// Same as [Self::Uptime] but get the most exact value as possible, at the expense of execution time
    UptimePrecise = libc::CLOCK_UPTIME_PRECISE,
    /// Same as [Self::Uptime] but do not perform a full time counter query, so the accuracy is one timer tick.
    UptimeFast = libc::CLOCK_UPTIME_FAST,
    /// Increments only when the CPU is running in user mode on behalf of the calling process.
    Virtual = libc::CLOCK_VIRTUAL,
    /// Increments when the CPU is running in user or kernel mode.
    Prof = libc::CLOCK_PROF,
    /// Returns the current second without performing a full time counter query, using an in-kernel cached value of the current second.
    Second = libc::CLOCK_SECOND,
    /// Returns the execution time of the calling process.
    ProcessCputimeId = libc::CLOCK_PROCESS_CPUTIME_ID,
    /// Returns the execution time of the calling thread.
    ThreadCputimeId = libc::CLOCK_THREAD_CPUTIME_ID,
}

#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
#[allow(non_upper_case_globals)]
impl ClockId {
    /// Alias for [Self::RealtimeFast].
    pub const RealtimeCoarse: Self = Self::RealtimeFast;
    /// Alias for [Self::MonotonicFast].
    pub const MonotonicCoarse: Self = Self::MonotonicFast;
    /// Alias for [Self::Uptime].
    pub const Boottime: Self = Self::Uptime;
}

#[cfg(target_os = "netbsd")]
mod sys {
    #![allow(
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
        deref_nullptr,
        dead_code
    )]

    include!(concat!(env!("OUT_DIR"), "/bindings/time.rs"));
}

#[cfg(target_os = "netbsd")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClockId {
    /// Identifies the realtime clock for the system. For this clock, the values specified by clock_settime() and obtained by clock_gettime() represent the amount of time (in seconds and nanoseconds) since 00:00 Universal Coordinated Time, January 1, 1970.
    Realtime = self::sys::CLOCK_REALTIME,

    /// Identifies a clock that increases at a steady rate (monotonically). This clock is not affected by calls to adjtime(2) and settimeofday(2) and will fail with an EINVAL error if it's the clock specified in a call to clock_settime(). The origin of the clock is unspecified.
    Monotonic = self::sys::CLOCK_MONOTONIC,

    /// Identifies a clock that increments only when the CPU is running in user mode on behalf of the calling process.
    Virtual = self::sys::CLOCK_VIRTUAL,

    /// Identifies a clock that increments when the CPU is running in user or kernel mode on behalf of the calling process.
    Prof = self::sys::CLOCK_PROF,

    /// Identifies a per process clock based on tick values. This clock is not settable.
    ProcessCputimeId = self::sys::CLOCK_PROCESS_CPUTIME_ID,

    /// Identifies a per thread clock based on tick values. This clock is not settable.
    ThreadCputimeId = self::sys::CLOCK_THREAD_CPUTIME_ID,
}

#[cfg(target_os = "openbsd")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClockId {
    /// The Coordinated Universal Time (UTC) clock. Its absolute value is the time elapsed since Jan 1 1970 00:00:00 UTC (the Epoch). The clock normally advances continuously, though it may jump discontinuously if a process calls settimeofday(2) or clock_settime().
    Realtime = libc::CLOCK_REALTIME,

    /// The monotonic clock. Its absolute value is meaningless. The clock begins at an undefined positive point and advances continuously.
    Monotonic = libc::CLOCK_MONOTONIC,

    /// The uptime clock. Its absolute value is the time elapsed since the system booted. The clock begins at zero and advances continuously.
    Boottime = libc::CLOCK_BOOTTIME,

    /// The runtime clock. Its absolute value is the time elapsed since the system booted less any time the system was suspended. The clock begins at zero and advances while the system is not suspended.
    Uptime = libc::CLOCK_UPTIME,

    /// The process CPU clock. Its absolute value begins at zero and advances while the calling process is running in user or kernel mode.
    ProcessCputimeId = libc::CLOCK_PROCESS_CPUTIME_ID,

    /// The thread CPU clock. Its absolute value begins at zero and advances while the calling thread is running in user or kernel mode.
    ThreadCputimeId = libc::CLOCK_THREAD_CPUTIME_ID,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Timespec(libc::timespec);

impl Timespec {
    #[inline(always)]
    pub const fn new(secs: i64, nsecs: u32) -> Self {
        Self(libc::timespec {
            tv_sec: secs as _,
            tv_nsec: nsecs as _,
        })
    }

    #[inline(always)]
    pub fn now(clockid: ClockId) -> Result<Self, Errno> {
        let mut buf = MaybeUninit::<libc::timespec>::uninit();
        if unsafe { libc::clock_gettime(clockid as _, buf.as_mut_ptr()) } == -1 {
            Err(Errno::last_os_error())
        } else {
            Ok(Self(unsafe { buf.assume_init() }))
        }
    }

    #[inline(always)]
    pub const fn secs(&self) -> i64 {
        self.0.tv_sec as _
    }

    #[inline(always)]
    pub fn set_secs(&mut self, secs: i64) {
        self.0.tv_sec = secs as _;
    }

    #[inline(always)]
    pub const fn nsecs(&self) -> u32 {
        self.0.tv_nsec as _
    }

    #[inline(always)]
    pub fn set_nsecs(&mut self, nsecs: u32) {
        self.0.tv_nsec = nsecs as _;
    }

    #[inline]
    pub fn set_clock(&self) -> Result<(), Errno> {
        if unsafe { libc::clock_settime(ClockId::Realtime as _, &self.0 as *const _) } == -1 {
            Err(Errno::last_os_error())
        } else {
            Ok(())
        }
    }
}
