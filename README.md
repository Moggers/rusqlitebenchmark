## Braindead Simple Sqlite Benchmark

This just runs 20'000 very very simple queries in sequence on both rusqlite and sqlx (using sqlite).

This is just to try and benchmark the *latency* difference between rusqlite and sqlx.

## Usage

`cargo test -- -Zunstable-options --report-time --ensure-time`

## My Results

```
➜  testasyncsqlite git:(master) ✗ cargo test -- -Zunstable-options --report-time --ensure-time                                                                        [25/08/3|11:09:05]
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running tests/test_rusqlite.rs (target/debug/deps/test_rusqlite-60326a86102a2654)

running 1 test
test rusqlite_tests::test_rusqlite ... ok <0.163s>

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s

     Running tests/test_sqlx.rs (target/debug/deps/test_sqlx-e5d7c49519befc95)

running 1 test
test sqlx_tests::test_sqlx ... ok <0.455s>

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.46s
```
