# might_sleep
Rust library for limiting the cpu usage by trying to target a specific frame rate.
The library will internally estimate the time we need to sleep to reach the target fps, taking the programs execution time into account.
