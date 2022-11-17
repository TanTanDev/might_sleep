use might_sleep::prelude::*;

mod usage {
    use std::time::Duration;

    pub const LOW: Duration = Duration::from_millis(100);
    pub const NORMAL: Duration = Duration::from_millis(30);
}

// flips the usage every 10 ticks, showing the target frame rate changing
fn main() {
    let mut cpu_limiter = CpuLimiter::new(usage::LOW);

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
    let opposite_usage = match cpu_limiter.duration {
        usage::LOW => usage::NORMAL,
        _ => usage::LOW,
    };
    println!("set usage to: {:?}", opposite_usage);
    cpu_limiter.duration = opposite_usage;
}
