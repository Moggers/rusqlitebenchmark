## Braindead Simple Sqlite Benchmark

This just runs 20'000 very very simple queries in sequence on both rusqlite and sqlx (using sqlite).

This is just to try and benchmark the *latency* difference between rusqlite and sqlx.

## Usage

`cargo test --release -- -Zunstable-options --report-time --ensure-time`

## My Results

```
➜  testasyncsqlite git:(master) cargo test --release -- -Zunstable-options --report-time --ensure-time                                                                [25/08/3|11:19:48]
    Finished `release` profile [optimized] target(s) in 0.06s
     Running tests/test_rusqlite.rs (target/release/deps/test_rusqlite-d3ca9014eb5b08c5)

running 1 test
test rusqlite_tests::test_rusqlite ... ok <0.069s>

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s

     Running tests/test_sqlx.rs (target/release/deps/test_sqlx-c03b517d722665ea)

running 1 test
test sqlx_tests::test_sqlx ... ok <0.402s>

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.40s
```
