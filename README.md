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
wink@fwlaptop 22-12-05T20:54:52.486Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo clean
wink@fwlaptop 22-12-05T20:54:54.756Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo run
   Compiling autocfg v1.1.0
   Compiling libc v0.2.138
   ...
   Compiling criterion v0.4.0
   Compiling exper-borrowed-vs-owned-parameters v0.4.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
    Finished dev [unoptimized + debuginfo] target(s) in 11.86s
     Running `target/debug/exper-borrowed-vs-owned-parameters`
wink@fwlaptop 22-12-05T20:55:14.758Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo run --release
   Compiling autocfg v1.1.0
   Compiling cfg-if v1.0.0
   ...
   Compiling exper-borrowed-vs-owned-parameters v0.4.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
    Finished release [optimized] target(s) in 24.51s
     Running `target/release/exper-borrowed-vs-owned-parameters`
wink@fwlaptop 22-12-05T20:55:44.364Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)

```

## Benchmarks:

Benchmark runs. NOTE: the use of `taskset -c 0` so we always use the same CPU, #0.

This summary here is that `borrowed` is faster than `owned`, which was
obvious when I learned `owned` always "moves" (shallow copy) the parameter.
But still it nice to see "proof".

Also, I'm now testing `Box<MsgXx>`, and it's nice to see that the
performance is approximatelty the same as `borrowed` even though Box<MssgXx>
is passed as an `owned` and that requires that the `msg` parameter be return
so it can be "reused".

There is one anomally, [see issue #2](https://github.com/winksaville/exper-borrowed-vs-owned-parameters/issues/2),
the single threaded performance on "fwlaptop" is
a little faster than "3900x".  But I see significant timing differences
for some benchmarks. Most commonly the MsgSf test.  For instance,
`boxed - boxed_msgsf_default` on fwlaptop we see:
  * 20.260 - 18.475  = 1.8ns:
```
compare_msgsf/boxed_msgsf_default
                        time:   [18.456 ns 18.475 ns 18.495 ns]
..
compare_msgsf/boxed     time:   [20.260 ns 20.325 ns 20.426 ns]
```
But on 3900x we see:
  * 27.430 - 19.760 = 7.6ns
```
compare_msgsf/boxed_msgsf_default
                        time:   [19.748 ns 19.760 ns 19.772 ns]
..
compare_msgsf/boxed     time:   [27.416 ns 27.430 ns 27.443 ns]
```
Here is the full output on fwlaptop:
```
wink@fwlaptop 22-12-05T20:57:30.265Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ taskset -c 0 cargo bench
   Compiling iai v0.1.1
   Compiling exper-borrowed-vs-owned-parameters v0.4.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
    Finished bench [optimized] target(s) in 4.98s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-3381d17f9a71713f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-181b8e44f009a960)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/crit_bench.rs (target/release/deps/bench_crit-d29bc3fd78f05c7a)
compare_msgnf/msgnf_default
                        time:   [956.15 ps 957.16 ps 958.31 ps]
Found 18 outliers among 100 measurements (18.00%)
  1 (1.00%) high mild
  17 (17.00%) high severe
compare_msgnf/borrowed  time:   [956.21 ps 957.16 ps 958.23 ps]
Found 20 outliers among 100 measurements (20.00%)
  5 (5.00%) high mild
  15 (15.00%) high severe
compare_msgnf/owned     time:   [956.06 ps 956.85 ps 957.73 ps]
Found 17 outliers among 100 measurements (17.00%)
  8 (8.00%) high mild
  9 (9.00%) high severe
compare_msgnf/boxed_msgnf_default
                        time:   [956.24 ps 957.21 ps 958.27 ps]
Found 20 outliers among 100 measurements (20.00%)
  3 (3.00%) high mild
  17 (17.00%) high severe
compare_msgnf/boxed     time:   [955.74 ps 956.54 ps 957.49 ps]
Found 18 outliers among 100 measurements (18.00%)
  2 (2.00%) high mild
  16 (16.00%) high severe

compare_msgof/msgof_default
                        time:   [9.0934 ns 9.1015 ns 9.1100 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe
compare_msgof/borrowed  time:   [10.930 ns 10.966 ns 11.010 ns]
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  10 (10.00%) high severe
compare_msgof/owned     time:   [20.379 ns 20.396 ns 20.414 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  3 (3.00%) high mild
  2 (2.00%) high severe
compare_msgof/boxed_msgof_default
                        time:   [18.154 ns 18.172 ns 18.191 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
compare_msgof/boxed     time:   [19.736 ns 19.766 ns 19.795 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high severe

compare_msgsf/msgsf_default
                        time:   [10.335 ns 10.348 ns 10.363 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  2 (2.00%) high severe
compare_msgsf/borrowed  time:   [12.001 ns 12.016 ns 12.031 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  4 (4.00%) high mild
  1 (1.00%) high severe
compare_msgsf/owned     time:   [27.168 ns 27.189 ns 27.210 ns]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe
compare_msgsf/boxed_msgsf_default
                        time:   [18.456 ns 18.475 ns 18.495 ns]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  4 (4.00%) high severe
compare_msgsf/boxed     time:   [20.260 ns 20.325 ns 20.426 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe

compare_msgmf/msgmf_default
                        time:   [18.074 ns 18.111 ns 18.148 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
compare_msgmf/borrowed  time:   [21.426 ns 21.450 ns 21.473 ns]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe
compare_msgmf/owned     time:   [70.076 ns 70.133 ns 70.196 ns]
Found 14 outliers among 100 measurements (14.00%)
  5 (5.00%) high mild
  9 (9.00%) high severe
compare_msgmf/boxed_msgmf_default
                        time:   [27.498 ns 27.529 ns 27.562 ns]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
compare_msgmf/boxed     time:   [29.308 ns 29.339 ns 29.372 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  3 (3.00%) high mild
  2 (2.00%) high severe

     Running benches/iai_bench.rs (target/release/deps/bench_iai-13fb485014f53b86)
invoke_msgnf_default
  Instructions:                   0
  L1 Accesses:      18446744073709551614
  L2 Accesses:                    5
  RAM Accesses:                   0
  Estimated Cycles:              23

invoke_msgof_default
  Instructions:                 165
  L1 Accesses:                  238
  L2 Accesses:                    5
  RAM Accesses:                   2
  Estimated Cycles:             333

invoke_msgsf_default
  Instructions:                 175
  L1 Accesses:                  255
  L2 Accesses:                    4
  RAM Accesses:                   5
  Estimated Cycles:             450

invoke_msgmf_default
  Instructions:                 230
  L1 Accesses:                  347
  L2 Accesses:                   10
  RAM Accesses:                  17
  Estimated Cycles:             992

invoke_boxed_msgnf_default
  Instructions:                   0
  L1 Accesses:      18446744073709551614
  L2 Accesses:                    4
  RAM Accesses:                   1
  Estimated Cycles:              53

invoke_boxed_msgof_default
  Instructions:                 456
  L1 Accesses:                  635
  L2 Accesses:                    4
  RAM Accesses:                   5
  Estimated Cycles:             830

invoke_boxed_msgsf_default
  Instructions:                 468
  L1 Accesses:                  652
  L2 Accesses:                    5
  RAM Accesses:                   8
  Estimated Cycles:             957

invoke_boxed_msgmf_default
  Instructions:                 512
  L1 Accesses:                  731
  L2 Accesses:                    6
  RAM Accesses:                  25
  Estimated Cycles:            1636

invoke_msgnf_borrowed
  Instructions:                   0
  L1 Accesses:      18446744073709551614
  L2 Accesses:                    4
  RAM Accesses:                   1
  Estimated Cycles:              53

invoke_msgnf_owned
  Instructions:                   0
  L1 Accesses:      18446744073709551613
  L2 Accesses:                    5
  RAM Accesses:                   1
  Estimated Cycles:              57

invoke_msgof_borrowed
  Instructions:                 208
  L1 Accesses:                  300
  L2 Accesses:                    4
  RAM Accesses:                   6
  Estimated Cycles:             530

invoke_msgof_owned
  Instructions:                 249
  L1 Accesses:                  373
  L2 Accesses:                    3
  RAM Accesses:                   7
  Estimated Cycles:             633

invoke_msgsf_borrowed
  Instructions:                 218
  L1 Accesses:                  318
  L2 Accesses:                    4
  RAM Accesses:                   7
  Estimated Cycles:             583

invoke_msgsf_owned
  Instructions:                 303
  L1 Accesses:                  468
  L2 Accesses:                    6
  RAM Accesses:                  16
  Estimated Cycles:            1058

invoke_msgmf_borrowed
  Instructions:                 273
  L1 Accesses:                  409
  L2 Accesses:                   10
  RAM Accesses:                  20
  Estimated Cycles:            1159

invoke_msgmf_owned
  Instructions:                 863
  L1 Accesses:                 1268
  L2 Accesses:                   33
  RAM Accesses:                  25
  Estimated Cycles:            2308

invoke_boxed_msgnf
  Instructions:                  17
  L1 Accesses:                   18
  L2 Accesses:                    5
  RAM Accesses:                   1
  Estimated Cycles:              78

invoke_boxed_msgof
  Instructions:                 509
  L1 Accesses:                  710
  L2 Accesses:                    3
  RAM Accesses:                   6
  Estimated Cycles:             935

invoke_boxed_msgsf
  Instructions:                 521
  L1 Accesses:                  727
  L2 Accesses:                    4
  RAM Accesses:                   9
  Estimated Cycles:            1062

invoke_boxed_msgmf
  Instructions:                 565
  L1 Accesses:                  803
  L2 Accesses:                    7
  RAM Accesses:                  27
  Estimated Cycles:            1783

wink@fwlaptop 22-12-05T21:01:04.933Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
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
