use std::time::{Duration, Instant};

use might_sleep::{cpu_limiter::CpuLimiter, usage::Usage};

// 10 ms error is okay for our tests
fn fuzzy_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.01
}

#[test]
fn test_cpu_limiter_idle() {
    let mut cpu_limiter = CpuLimiter::new(Duration::from_millis(100), Duration::from_millis(10));
    let now = Instant::now();
    for i in 0..100 {
        println!("idle_time: {i}");
        cpu_limiter.might_sleep();
    }
    assert!(fuzzy_eq(now.elapsed().as_secs_f32(), 10f32));
}

#[test]
// this test checks that the cpu_limiter DOES NOT sleep if the computation time is slow
fn test_slow_computation() {
    let mut cpu_limiter = CpuLimiter::new(Duration::from_millis(100), Duration::from_millis(10));
    let now = Instant::now();
    for i in 0..10 {
        println!("slow_computation: {i}");
        std::thread::sleep(Duration::from_millis(200));
        cpu_limiter.might_sleep();
    }
    assert!(fuzzy_eq(now.elapsed().as_secs_f32(), 2f32));
}

#[test]
fn test_cpu_limiter_normal() {
    let mut cpu_limiter = CpuLimiter::new(Duration::from_millis(100), Duration::from_millis(10));
    cpu_limiter.change_usage(Usage::Normal);
    let now = Instant::now();
    for i in 0..100 {
        println!("normal_time: {i}");
        cpu_limiter.might_sleep();
    }
    assert!(fuzzy_eq(now.elapsed().as_secs_f32(), 1f32));
}
