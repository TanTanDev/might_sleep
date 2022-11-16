use std::time::Duration;

use might_sleep::prelude::*;

// flips the usage every 10 ticks, showing the target frame rate changing
fn main() {
    let mut cpu_limiter = CpuLimiter::new(
        Duration::from_millis(100), // used in Usage::Low
        Duration::from_millis(30),  // used in Usage::Normal
    );

    let mut i = 0;
    loop {
        i += 1;
        if i % 10 == 0 {
            toggle_usage(&mut cpu_limiter);
        }
        println!("i: {:?}", i);
        cpu_limiter.might_sleep();
    }
}

fn toggle_usage(cpu_limiter: &mut CpuLimiter) {
    let opposite_usage = match cpu_limiter.usage {
        Usage::Low => Usage::Normal,
        Usage::Normal => Usage::Low,
    };
    println!("set usage to: {:?}", opposite_usage);
    cpu_limiter.usage = opposite_usage;
}
