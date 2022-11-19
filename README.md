![](logo.png)

[![Crates.io](https://img.shields.io/crates/v/might_sleep.svg)](https://crates.io/crates/might_sleep)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/TanTanDev/might_sleep/blob/main/LICENSE)
# might_sleep

Rust library for limiting the cpu usage by trying to target a specific frame rate.
The library will internally estimate the time we need to sleep to reach the target fps, taking the programs execution time into account.

example code:
```rs
mod usage {
    use std::time::Duration;

    pub const LOW: Duration = Duration::from_millis(100);
    pub const NORMAL: Duration = Duration::from_millis(50);
}

fn main() {
    let mut cpu_limiter = CpuLimiter::new(usage::LOW);

    cpu_limiter.duration = usage::NORMAL; // usage is now normal (50ms delay)
    loop {
        println!("should be called every 50 ms");
        cpu_limiter.might_sleep();
    }
}
```

## License

might_sleep is free and open source! All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](docs/LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](docs/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
