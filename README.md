```
$ cargo bench
    Finished bench [optimized] target(s) in 0.07s
     Running target/release/deps/async_benchmark-8063bcc0c73e79f2

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/release/deps/async_benchmark-0ddcd3d1059347b8

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/release/deps/async_benchmark-8bceaaee837124b3
smol timers             time:   [146.53 us 148.31 us 150.12 us]
                        change: [-4.5745% -3.0465% -1.4961%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking tokio timers: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.2s, enable flat sampling, or reduce sample count to 50.
tokio timers            time:   [1.6265 ms 1.6376 ms 1.6468 ms]
                        change: [-1.6622% -0.6180% +0.4660%] (p = 0.26 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  8 (8.00%) low mild
```

