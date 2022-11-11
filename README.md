# might_sleep
![](logo.png)

Rust library for limiting the cpu usage by trying to target a specific frame rate.
The library will internally estimate the time we need to sleep to reach the target fps, taking the programs execution time into account.

example code:
```rs
let mut cpu_limiter = CpuLimiter::new(
  Duration::from_millis(100), // used in Usage::Low
  Duration::from_millis(10)   // used in Usage::Normal
);

// will now target 10 ms between frames
cpu_limiter.change_usage(Usage::Normal);
loop {
    cpu_limiter.might_sleep();
}

```
