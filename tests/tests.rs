use std::time::{Duration, Instant};

use might_sleep::cpu_limiter::CpuLimiter;

mod usage {
    use std::time::Duration;

    pub const LOW: Duration = Duration::from_millis(100);
    pub const NORMAL: Duration = Duration::from_millis(10);
}

fn fuzzy_eq(a: f32, b: f32, range: f32) -> bool {
    println!("{} {}", a, b);
    (a - b).abs() < range
}

#[test]
fn test_cpu_limiter_idle() {
    const ITER: i32 = 100;
    const RANGE: f32 = (ITER as f32 / 10000.) * 2.5;

    let mut cpu_limiter = CpuLimiter::new(usage::LOW);
    let now = Instant::now();
    for i in 0..ITER {
        println!("idle_time: {i}");
        cpu_limiter.might_sleep();
    }
    assert!(fuzzy_eq(now.elapsed().as_secs_f32(), 10f32, RANGE));
}

#[test]
// this test checks that the cpu_limiter DOES NOT sleep if the computation time is slow
fn test_slow_computation() {
    const ITER: i32 = 10;
    const RANGE: f32 = (ITER as f32 / 10000.) * 2.5;

    let mut cpu_limiter = CpuLimiter::new(usage::LOW);
    let now = Instant::now();
    for i in 0..ITER {
        println!("slow_computation: {i}");
        std::thread::sleep(Duration::from_millis(200));
        cpu_limiter.might_sleep();
    }
    assert!(fuzzy_eq(now.elapsed().as_secs_f32(), 2f32, RANGE));
}

#[test]
fn test_cpu_limiter_change_duration() {
    const ITER: i32 = 100;
    const RANGE: f32 = (ITER as f32 / 10000.) * 2.5;

    let mut cpu_limiter = CpuLimiter::new(usage::LOW);
    cpu_limiter.duration = usage::NORMAL;
    let now = Instant::now();
    for i in 0..ITER {
        println!("normal_time: {i}");
        cpu_limiter.might_sleep();
    }
    assert!(fuzzy_eq(now.elapsed().as_secs_f32(), 1f32, RANGE));
}

#[test]
fn test_cpu_limiter_get_time_slept() {
    let mut cpu_limiter = CpuLimiter::new(usage::LOW);
    let now = Instant::now();
    let time_slept = cpu_limiter.might_sleep_get();
    assert!(fuzzy_eq(
        now.elapsed().as_secs_f32(),
        time_slept.as_secs_f32(),
        0.01
    ))
}
