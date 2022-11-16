use std::time::Duration;

use might_sleep::prelude::*;

fn main() {
    let mut cpu_limiter = CpuLimiter::new(
        Duration::from_millis(100), // used in Usage::Low
        Duration::from_millis(50),  // used in Usage::Normal
    );

    cpu_limiter.usage = Usage::Normal;
    loop {
        println!("should be called every 50 ms");
        cpu_limiter.might_sleep();
    }
}
