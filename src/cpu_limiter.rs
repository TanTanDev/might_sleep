use std::time::{Duration, Instant};

/// lowers cpu usage by sleeping, if the execution is slow, no sleeping occurs.
pub struct CpuLimiter {
    pub duration: Duration,
    last_time: Instant,
}

impl CpuLimiter {
    /// parameters sets the duration for the specified Usage, idle then normal
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            last_time: Instant::now(),
        }
    }

    /// try to estimate the time to sleep to reach the target framerate based on the usage
    /// will not sleep if the last proccessing time was slower th
    pub fn might_sleep(&mut self) {
        let last_loop_time = self.last_time.elapsed();

        if let Some(diff) = self.duration.checked_sub(last_loop_time) {
            if !diff.is_zero() {
                std::thread::sleep(diff);
            }
        }
        self.last_time = Instant::now();
    }
}
