use std::time::{Duration, Instant};

use crate::usage::Usage;

/// lowers cpu usage by sleeping, if the execution is slow, no sleeping occurs.
pub struct CpuLimiter {
    usage: Usage,
    min_idle_duration: Duration,
    min_normal_duration: Duration,
    last_time: Instant,
}

impl CpuLimiter {
    /// parameters sets the duration for the specified Usage, idle then normal
    pub fn new(min_idle_duration: Duration, min_normal_duration: Duration) -> Self {
        Self {
            usage: Usage::Low,
            min_idle_duration,
            min_normal_duration,
            last_time: Instant::now(),
        }
    }

    /// set the usage mode, for either increasing or decreasing target frame time  
    pub fn change_usage(&mut self, usage: Usage) {
        self.usage = usage;
    }

    /// try to estimate the time to sleep to reach the target framerate based on the usage
    /// will not sleep if the last proccessing time was slower th
    pub fn might_sleep(&mut self) {
        let duration = match self.usage {
            Usage::Low => self.min_idle_duration,
            Usage::Normal => self.min_normal_duration,
        };
        let last_loop_time = self.last_time.elapsed();

        if let Some(diff) = duration.checked_sub(last_loop_time) {
            if !diff.is_zero() {
                std::thread::sleep(diff);
            }
        }
        self.last_time = Instant::now();
    }
}
