# Usage

```sh
cargo run --release
cargo bench
```

# Sample output

```
terrible hack           time:   [44.264 ms 44.408 ms 44.571 ms]                          
                        change: [-0.5846% -0.2183% +0.1759%] (p = 0.26 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Benchmarking std::fs::read: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 12.3s, or reduce sample count to 40.
std::fs::read           time:   [121.31 ms 121.43 ms 121.56 ms]                          
                        change: [-1.7620% -1.4190% -1.0695%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  9 (9.00%) high mild
```