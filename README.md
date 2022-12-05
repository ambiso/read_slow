# Usage

```sh
cargo run --release
cargo bench
```

# Sample output

```
Benchmarking std::fs::read: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 11.9s, or reduce sample count to 40.
std::fs::read           time:   [118.00 ms 118.14 ms 118.32 ms]                          
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

terrible hack           time:   [43.437 ms 43.576 ms 43.782 ms]                          
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

Benchmarking terrible hack guess: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 12.1s, or reduce sample count to 40.
terrible hack guess     time:   [121.80 ms 122.14 ms 122.51 ms]                                
Found 15 outliers among 100 measurements (15.00%)
  13 (13.00%) high mild
  2 (2.00%) high severe
```