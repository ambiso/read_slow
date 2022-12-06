# Usage

```sh
cargo run --release
cargo bench
```

# Sample output

```
Benchmarking std_read 1 byte aligned: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 11.7s, or reduce sample count to 40.
std_read 1 byte aligned time:   [116.54 ms 116.72 ms 116.93 ms]                                    
                        change: [-4.2368% -3.8186% -3.4325%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  7 (7.00%) high mild
  7 (7.00%) high severe

std_read 32 byte aligned                                                                            
                        time:   [42.619 ms 42.772 ms 42.948 ms]
                        change: [+1.5968% +2.1003% +2.6183%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) high mild
  6 (6.00%) high severe
```