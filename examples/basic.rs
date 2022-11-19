use might_sleep::prelude::*;

mod usage {
    use std::time::Duration;

    pub const LOW: Duration = Duration::from_millis(100);
    pub const NORMAL: Duration = Duration::from_millis(50);
}

fn main() {
    let mut cpu_limiter = CpuLimiter::new(usage::LOW);

    cpu_limiter.duration = usage::NORMAL;
    loop {
        println!("should be called every 50 ms");
        cpu_limiter.might_sleep();
    }
}
