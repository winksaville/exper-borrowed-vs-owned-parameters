# Experiment with borrowed vs owned fn parameters

I'm using #[inline(never)] in src/lib.rs so we actually
benchmark parameter passing costs, otherwise parameter
passing will be optimized away in some cases.

Also, I'm using profile.dev and profile.release in Cargo.toml
in attempting to get more consistency in the benchmark numbers.
In particular the `codegen-units=1` for profile.release.

Here is the library [lib.rs](/src/lib.rs)

Here is [main.rs](/src/main.rs):

Here are some runs:
```
wink@3900x 22-12-05T02:17:04.328Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/exper-borrowed-vs-owned-parameters`
wink@3900x 22-12-05T02:17:07.952Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/exper-borrowed-vs-owned-parameters`
wink@3900x 22-12-05T02:17:18.365Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
```

## Benchmarks:

Benchmark runs. NOTE: the use of `taskset -c 15` so we always use the same CPU, #15.

This summary here is that `borrowed` is faster than `owned`, which was
obvious when I learned `owned` always "moves" (shallow copy) the parameter.
But still it nice to see "proof", gtgt
```
wink@3900x 22-12-05T02:11:59.323Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo clean
wink@3900x 22-12-05T02:12:02.950Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ taskset -c 15 cargo bench
   Compiling autocfg v1.1.0
   Compiling libc v0.2.138
   ...
   Compiling volatile v0.4.5
   Compiling exper-borrowed-vs-owned-parameters v0.4.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
   Compiling iai v0.1.1
    Finished bench [optimized] target(s) in 50.43s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-3381d17f9a71713f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-181b8e44f009a960)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/crit_bench.rs (target/release/deps/bench_crit-d29bc3fd78f05c7a)
borrowed                time:   [551.36 ps 552.26 ps 553.18 ps]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

borrowed_with_clone     time:   [10.604 ns 10.661 ns 10.735 ns]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe

owned                   time:   [11.032 ns 11.072 ns 11.124 ns]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) high mild
  11 (11.00%) high severe

compare_msgnf/msgnf_default
                        time:   [1.0877 ns 1.0906 ns 1.0944 ns]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
compare_msgnf/borrowed  time:   [1.0860 ns 1.0885 ns 1.0915 ns]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
compare_msgnf/owned     time:   [1.0932 ns 1.0949 ns 1.0969 ns]
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) low mild
  2 (2.00%) high mild
  5 (5.00%) high severe

compare_msgof/msgof_default
                        time:   [9.9932 ns 9.9990 ns 10.006 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe
compare_msgof/borrowed  time:   [12.845 ns 12.867 ns 12.892 ns]
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
compare_msgof/owned     time:   [23.443 ns 23.461 ns 23.479 ns]

compare_msgsf/msgsf_default
                        time:   [10.771 ns 10.782 ns 10.793 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
compare_msgsf/borrowed  time:   [13.340 ns 13.379 ns 13.429 ns]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe
compare_msgsf/owned     time:   [25.008 ns 25.038 ns 25.077 ns]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

compare_msgmf/msgmf_default
                        time:   [22.775 ns 22.859 ns 22.951 ns]
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
compare_msgmf/borrowed  time:   [24.119 ns 24.270 ns 24.435 ns]
Found 25 outliers among 100 measurements (25.00%)
  13 (13.00%) low mild
  3 (3.00%) high mild
  9 (9.00%) high severe
compare_msgmf/owned     time:   [76.900 ns 77.034 ns 77.193 ns]

     Running benches/iai_bench.rs (target/release/deps/bench_iai-13fb485014f53b86)
invoke_msgnf_default
  Instructions:                   0
  L1 Accesses:      18446744073709551612
  L2 Accesses:                    1
  RAM Accesses:                   3
  Estimated Cycles:             106

invoke_msgof_default
  Instructions:                 151
  L1 Accesses:                  213
  L2 Accesses:                    3
  RAM Accesses:                   7
  Estimated Cycles:             473

invoke_msgsf_default
  Instructions:                 161
  L1 Accesses:                  230
  L2 Accesses:                    3
  RAM Accesses:                   9
  Estimated Cycles:             560

invoke_msgmf_default
  Instructions:                 216
  L1 Accesses:                  325
  L2 Accesses:                    7
  RAM Accesses:                  20
  Estimated Cycles:            1060

invoke_msgnf_borrowed
  Instructions:                   0
  L1 Accesses:      18446744073709551610
  L2 Accesses:                    2
  RAM Accesses:                   4
  Estimated Cycles:             144

invoke_msgnf_owned
  Instructions:                   0
  L1 Accesses:      18446744073709551609
  L2 Accesses:                    3
  RAM Accesses:                   4
  Estimated Cycles:             148

invoke_msgof_borrowed
  Instructions:                 177
  L1 Accesses:                  256
  L2 Accesses:                    3
  RAM Accesses:                   8
  Estimated Cycles:             551

invoke_msgof_owned
  Instructions:                 218
  L1 Accesses:                  327
  L2 Accesses:                    2
  RAM Accesses:                  11
  Estimated Cycles:             722

invoke_msgsf_borrowed
  Instructions:                 187
  L1 Accesses:                  273
  L2 Accesses:                    2
  RAM Accesses:                  11
  Estimated Cycles:             668

invoke_msgsf_owned
  Instructions:                 272
  L1 Accesses:                  427
  L2 Accesses:                    3
  RAM Accesses:                  17
  Estimated Cycles:            1037

invoke_msgmf_borrowed
  Instructions:                 273
  L1 Accesses:                  411
  L2 Accesses:                    8
  RAM Accesses:                  20
  Estimated Cycles:            1151

invoke_msgmf_owned
  Instructions:                 863
  L1 Accesses:                 1268
  L2 Accesses:                   32
  RAM Accesses:                  26
  Estimated Cycles:            2338

wink@3900x 22-12-05T02:15:29.519Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)

```

## Asm code

The assembler code can be found at [/asm](/asm)
and is generated with [`./gen_asm.sh`](/gen_asm.sh).


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
