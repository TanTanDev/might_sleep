use std::time::{Duration, Instant};

/// Lowers cpu usage by sleeping, if the execution is slow, no sleeping occurs.
///
/// # Examples
/// ```
/// # use std::time::Duration;
/// use might_sleep::prelude::*;
///
/// let mut cpu_limiter = CpuLimiter::new(Duration::from_millis(100));
///
/// for i in 0..10 {
///     cpu_limiter.might_sleep();
///     cpu_limiter.duration = Duration::from_millis(20);
/// }
/// ```
pub struct CpuLimiter {
    pub duration: Duration,
    last_time: Instant,
}

impl CpuLimiter {
    /// Creates a new CpuLimiter with the specified Duration for sleeping
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            last_time: Instant::now(),
        }
    }

    /// Try to estimate the time to sleep to reach the target framerate based on the current Duration
    /// Will not sleep if the last proccessing time surpasses the current Duration
    pub fn might_sleep(&mut self) {
        let last_loop_time = self.last_time.elapsed();

        if let Some(diff) = self.duration.checked_sub(last_loop_time) {
            if !diff.is_zero() {
                std::thread::sleep(diff);
            }
        }
        self.last_time = Instant::now();
    }

    /// Try to estimate the time to sleep to reach the target framerate based on the current Duration
    /// Will not sleep if the last proccessing time surpasses the current Duration
    ///
    /// Returns the time it slept
    pub fn might_sleep_get(&mut self) -> Duration {
        let mut sleep_duration = Duration::ZERO;

        let last_loop_time = self.last_time.elapsed();

        if let Some(diff) = self.duration.checked_sub(last_loop_time) {
            if !diff.is_zero() {
                std::thread::sleep(diff);
                sleep_duration = diff;
            }
        }
        self.last_time = Instant::now();

        sleep_duration
    }
}
