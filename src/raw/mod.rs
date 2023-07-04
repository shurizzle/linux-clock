use core::{fmt, time::Duration};

#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(not(target_os = "linux"), path = "common.rs")]
mod inner;

pub use inner::*;

const NSEC_PER_SEC: u64 = 1_000_000_000;
const I64_MAX: u64 = 9_223_372_036_854_775_807;

impl Timespec {
    #[inline(always)]
    pub const fn zero() -> Self {
        Self::new(0, 0)
    }

    #[inline(always)]
    pub const fn seconds(&self) -> i64 {
        self.secs()
    }

    #[inline(always)]
    pub fn set_seconds(&mut self, secs: i64) {
        self.set_secs(secs)
    }

    #[inline(always)]
    pub const fn nanosecs(&self) -> u32 {
        self.nsecs()
    }

    #[inline(always)]
    pub const fn nanoseconds(&self) -> u32 {
        self.nsecs()
    }

    #[inline(always)]
    pub fn set_nanosecs(&mut self, nsecs: u32) {
        self.set_nsecs(nsecs)
    }

    #[inline(always)]
    pub fn set_nanoseconds(&mut self, nsecs: u32) {
        self.set_nsecs(nsecs)
    }

    pub fn sub_timespec(&self, other: &Timespec) -> Result<Duration, Duration> {
        if self >= other {
            // NOTE(eddyb) two aspects of this `if`-`else` are required for LLVM
            // to optimize it into a branchless form (see also #75545):
            //
            // 1. `self.secs() - other.secs()` shows up as a common expression
            //    in both branches, i.e. the `else` must have its `- 1`
            //    subtraction after the common one, not interleaved with it
            //    (it used to be `self.secs() - 1 - other.secs()`)
            //
            // 2. the `Duration::new` call (or any other additional complexity)
            //    is outside of the `if`-`else`, not duplicated in both branches
            //
            // Ideally this code could be rearranged such that it more
            // directly expresses the lower-cost behavior we want from it.
            let (secs, nsecs) = if self.nsecs() >= other.nsecs() {
                (
                    (self.secs() - other.secs()) as u64,
                    self.nsecs() - other.nsecs(),
                )
            } else {
                (
                    (self.secs() - other.secs() - 1) as u64,
                    self.nsecs() + (NSEC_PER_SEC as u32) - other.nsecs(),
                )
            };

            Ok(Duration::new(secs, nsecs))
        } else {
            match other.sub_timespec(self) {
                Ok(d) => Err(d),
                Err(d) => Ok(d),
            }
        }
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Timespec> {
        #[inline(always)]
        // fn checked_add_unsigned(a: i64, b: u64) -> Option<i64> {
        //     a.checked_add_unsigned(b)
        // }
        fn checked_add_unsigned(a: i64, b: u64) -> Option<i64> {
            let b = if b > I64_MAX {
                return None;
            } else {
                b as i64
            };
            a.checked_add(b)
        }

        let mut secs = checked_add_unsigned(self.secs(), other.as_secs())?;

        // Nano calculations can't overflow because nanos are <1B which fit
        // in a u32.
        let mut nsecs = other.subsec_nanos() + self.nsecs();
        if nsecs >= NSEC_PER_SEC as u32 {
            nsecs -= NSEC_PER_SEC as u32;
            secs = secs.checked_add(1)?;
        }
        Some(Timespec::new(secs, nsecs))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Timespec> {
        #[inline(always)]
        // fn checked_sub_unsigned(a: i64, b: u64) -> Option<i64> {
        //     a.checked_sub_unsigned(b)
        // }
        fn checked_sub_unsigned(a: i64, b: u64) -> Option<i64> {
            let b = if b > I64_MAX {
                return None;
            } else {
                b as i64
            };
            a.checked_sub(b)
        }

        let mut secs = checked_sub_unsigned(self.secs(), other.as_secs())?;

        // Similar to above, nanos can't overflow.
        let mut nsecs = self.nsecs() as i32 - other.subsec_nanos() as i32;
        if nsecs < 0 {
            nsecs += NSEC_PER_SEC as i32;
            secs = secs.checked_sub(1)?;
        }
        Some(Timespec::new(secs, nsecs as u32))
    }
}

impl Default for Timespec {
    #[inline]
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Debug for Timespec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timespec")
            .field("secs", &self.secs())
            .field("nsecs", &self.nsecs())
            .finish()
    }
}

impl PartialEq for Timespec {
    fn eq(&self, other: &Self) -> bool {
        self.secs() == other.secs() && self.nsecs() == other.nsecs()
    }
}

impl Eq for Timespec {}

impl PartialOrd for Timespec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.secs().partial_cmp(&other.secs()) {
            Some(core::cmp::Ordering::Equal) => (),
            ord => return ord,
        }
        self.nsecs().partial_cmp(&other.nsecs())
    }
}

impl Ord for Timespec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.secs().cmp(&other.secs()) {
            core::cmp::Ordering::Equal => (),
            ord => return ord,
        }
        self.nsecs().cmp(&other.nsecs())
    }
}

impl core::hash::Hash for Timespec {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.secs().hash(state);
        self.nsecs().hash(state);
    }
}
