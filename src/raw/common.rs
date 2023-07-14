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

#[cfg(target_os = "freebsd")]
#[repr(i32)]
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

#[cfg(target_os = "freebsd")]
#[allow(non_upper_case_globals)]
impl ClockId {
    /// Alias for [Self::RealtimeFast].
    pub const RealtimeCoarse: Self = Self::RealtimeFast;
    /// Alias for [Self::MonotonicFast].
    pub const MonotonicCoarse: Self = Self::MonotonicFast;
    /// Alias for [Self::Uptime].
    pub const Boottime: Self = Self::Uptime;
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
