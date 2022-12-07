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
wink@fwlaptop 22-12-07T22:32:24.372Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/exper-borrowed-vs-owned-parameters`
discrimiant::Nf Discriminant(0)
discrimiant::Of Discriminant(1)
discrimiant::Sf Discriminant(2)
discrimiant::Mf Discriminant(3)
wink@fwlaptop 22-12-07T22:32:28.672Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/exper-borrowed-vs-owned-parameters`
discrimiant::Nf Discriminant(0)
discrimiant::Of Discriminant(1)
discrimiant::Sf Discriminant(2)
discrimiant::Mf Discriminant(3)
wink@fwlaptop 22-12-07T22:32:30.310Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
```

## Benchmarks:

Benchmark runs. NOTE: the use of `taskset -c 0` so we always use the same CPU, #0.

This summary here is that `borrowed` is faster than `owned`, which was
obvious when I learned `owned` always "moves" (shallow copy) the parameter.
But still it nice to see "proof".

I'm now testing `Box<MsgXx>`, and it's nice to see that the
performance is approximatelty the same as `borrowed` even though Box<MssgXx>
is passed as an `owned` and that requires that the `msg` parameter be return
so it can be "reused".

And now I've created `Protocol` which is an enum with variants for each `MsgXx`.
And allows a routine to take and `MsgXx` type as a parameter. This is how
`proc-macro-hsm1` works. In addition I've validated that passing around `owned`
`Box<Protocol>` seems to be efficient. Allocating them individually isn't efficient,
but I'd like to create an "Pool" or other mechanism where the multiple `Protocol` are
preallocated. And the use a "get" for each variant that returns an initialized instance.
And then has a "drop" that places it back into the pool.

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
wink@fwlaptop 22-12-07T22:04:01.614Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ taskset -c 0 cargo bench
   Compiling exper-borrowed-vs-owned-parameters v0.6.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
    Finished bench [optimized] target(s) in 7.70s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-c9df0434d200847b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-96d09c41c651e971)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/crit_bench.rs (target/release/deps/bench_crit-657488c7198c4ae5)
borrowed                time:   [438.53 ps 439.74 ps 441.02 ps]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  1 (1.00%) high severe

borrowed_with_clone     time:   [8.9634 ns 8.9779 ns 8.9941 ns]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

owned                   time:   [8.9490 ns 8.9593 ns 8.9702 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

Benchmarking compare_msgnf/msgnf_default: Collecting 100 samples in estimated 5.0000 s (5.2B iterations)^C
wink@fwlaptop 22-12-07T22:04:49.126Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo clean
wink@fwlaptop 22-12-07T22:04:51.551Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ taskset -c 0 cargo bench
   Compiling autocfg v1.1.0
   Compiling cfg-if v1.0.0
   ...
   Compiling volatile v0.4.5
   Compiling exper-borrowed-vs-owned-parameters v0.6.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
   Compiling iai v0.1.1
    Finished bench [optimized] target(s) in 58.50s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-c9df0434d200847b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-96d09c41c651e971)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/crit_bench.rs (target/release/deps/bench_crit-657488c7198c4ae5)
borrowed                time:   [437.44 ps 438.65 ps 440.00 ps]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild

borrowed_with_clone     time:   [8.9701 ns 8.9859 ns 9.0053 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  4 (4.00%) high severe

owned                   time:   [8.9660 ns 8.9775 ns 8.9903 ns]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

compare_msgnf/msgnf_default
                        time:   [955.59 ps 956.51 ps 957.71 ps]
Found 16 outliers among 100 measurements (16.00%)
  6 (6.00%) high mild
  10 (10.00%) high severe
compare_msgnf/borrowed  time:   [955.65 ps 956.93 ps 958.83 ps]
Found 17 outliers among 100 measurements (17.00%)
  3 (3.00%) high mild
  14 (14.00%) high severe
compare_msgnf/owned     time:   [955.48 ps 956.34 ps 957.46 ps]
Found 18 outliers among 100 measurements (18.00%)
  4 (4.00%) high mild
  14 (14.00%) high severe
compare_msgnf/boxed_msgnf_default
                        time:   [955.40 ps 956.41 ps 957.65 ps]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe
compare_msgnf/boxed     time:   [955.38 ps 956.31 ps 957.53 ps]
Found 18 outliers among 100 measurements (18.00%)
  4 (4.00%) high mild
  14 (14.00%) high severe

compare_msgof/msgof_default
                        time:   [8.7820 ns 8.7949 ns 8.8088 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) low mild
  1 (1.00%) high severe
compare_msgof/borrowed  time:   [10.490 ns 10.505 ns 10.522 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
compare_msgof/owned     time:   [20.384 ns 20.400 ns 20.420 ns]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
compare_msgof/boxed_msgof_default
                        time:   [17.404 ns 17.427 ns 17.452 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high severe
compare_msgof/boxed     time:   [19.266 ns 19.285 ns 19.306 ns]
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low severe
  4 (4.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe

compare_msgsf/msgsf_default
                        time:   [9.5561 ns 9.5682 ns 9.5806 ns]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
compare_msgsf/borrowed  time:   [11.487 ns 11.502 ns 11.521 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  3 (3.00%) high severe
compare_msgsf/owned     time:   [25.917 ns 25.952 ns 25.994 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe
compare_msgsf/boxed_msgsf_default
                        time:   [17.776 ns 17.904 ns 18.103 ns]
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  6 (6.00%) high mild
  3 (3.00%) high severe
compare_msgsf/boxed     time:   [19.633 ns 19.657 ns 19.684 ns]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  4 (4.00%) high mild
  5 (5.00%) high severe

compare_msgmf/msgmf_default
                        time:   [17.984 ns 18.005 ns 18.027 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
compare_msgmf/borrowed  time:   [18.859 ns 18.878 ns 18.901 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high severe
compare_msgmf/owned     time:   [68.718 ns 68.754 ns 68.795 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
compare_msgmf/boxed_msgmf_default
                        time:   [26.619 ns 26.656 ns 26.706 ns]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  1 (1.00%) high mild
  4 (4.00%) high severe
compare_msgmf/boxed     time:   [28.580 ns 28.610 ns 28.646 ns]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high severe

compare_protocol_nf/protocol_nf_default
                        time:   [955.45 ps 956.13 ps 957.20 ps]
Found 19 outliers among 100 measurements (19.00%)
  8 (8.00%) high mild
  11 (11.00%) high severe
compare_protocol_nf/borrowed
                        time:   [1.6736 ns 1.6772 ns 1.6816 ns]
Found 20 outliers among 100 measurements (20.00%)
  6 (6.00%) high mild
  14 (14.00%) high severe
compare_protocol_nf/owned
                        time:   [46.325 ns 46.403 ns 46.510 ns]
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe
compare_protocol_nf/boxed_protocol_nf_default
                        time:   [14.365 ns 14.375 ns 14.387 ns]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe
compare_protocol_nf/boxed_protocol_nf
                        time:   [16.178 ns 16.195 ns 16.213 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

compare_protocol_of/protocol_of_default
                        time:   [8.7804 ns 8.7922 ns 8.8045 ns]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
compare_protocol_of/borrowed
                        time:   [10.755 ns 10.772 ns 10.795 ns]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe
compare_protocol_of/owned
                        time:   [57.827 ns 57.867 ns 57.918 ns]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe
compare_protocol_of/boxed_protocol_of_default
                        time:   [22.450 ns 22.468 ns 22.487 ns]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe
compare_protocol_of/boxed_protocol_nf
                        time:   [25.600 ns 25.618 ns 25.636 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

compare_protocol_sf/protocol_sf_default
                        time:   [9.5520 ns 9.5745 ns 9.6020 ns]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low severe
  3 (3.00%) low mild
  1 (1.00%) high severe
compare_protocol_sf/borrowed
                        time:   [11.725 ns 11.735 ns 11.747 ns]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe
compare_protocol_sf/owned
                        time:   [60.314 ns 60.360 ns 60.429 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe
compare_protocol_sf/boxed_protocol_sf_default
                        time:   [23.461 ns 23.488 ns 23.518 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe
compare_protocol_sf/boxed_protocol_nf
                        time:   [27.423 ns 27.464 ns 27.512 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  3 (3.00%) high mild
  2 (2.00%) high severe

compare_protocol_mf/protocol_mf_default
                        time:   [17.748 ns 17.784 ns 17.819 ns]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
compare_protocol_mf/borrowed
                        time:   [19.748 ns 19.772 ns 19.798 ns]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
compare_protocol_mf/owned
                        time:   [72.185 ns 72.258 ns 72.350 ns]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild
  3 (3.00%) high severe
compare_protocol_mf/boxed_protocol_mf_default
                        time:   [31.829 ns 31.870 ns 31.927 ns]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  7 (7.00%) high severe
compare_protocol_mf/boxed_protocol_nf
                        time:   [34.567 ns 34.607 ns 34.658 ns]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
compare_protocol_mf/boxed_protocol_nf_inline_always
                        time:   [34.065 ns 34.101 ns 34.145 ns]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  5 (5.00%) high severe
compare_protocol_mf/boxed_protocol_nf_inline_no_suggestion
                        time:   [33.956 ns 33.999 ns 34.056 ns]
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  6 (6.00%) high severe

     Running benches/iai_bench.rs (target/release/deps/bench_iai-5c6b806ea9423a70)
invoke_msgnf_default
  Instructions:                   0
  L1 Accesses:      18446744073709551612
  L2 Accesses:                    2
  RAM Accesses:                   2
  Estimated Cycles:              76

invoke_msgof_default
  Instructions:                 157
  L1 Accesses:                  227
  L2 Accesses:                    3
  RAM Accesses:                   3
  Estimated Cycles:             347

invoke_msgsf_default
  Instructions:                 167
  L1 Accesses:                  243
  L2 Accesses:                    3
  RAM Accesses:                   6
  Estimated Cycles:             468

invoke_msgmf_default
  Instructions:                 222
  L1 Accesses:                  336
  L2 Accesses:                    8
  RAM Accesses:                  18
  Estimated Cycles:            1006

invoke_boxed_msgnf_default
  Instructions:                   0
  L1 Accesses:      18446744073709551611
  L2 Accesses:                    3
  RAM Accesses:                   2
  Estimated Cycles:              80

invoke_boxed_msgof_default
  Instructions:                 448
  L1 Accesses:                  622
  L2 Accesses:                    4
  RAM Accesses:                   6
  Estimated Cycles:             852

invoke_boxed_msgsf_default
  Instructions:                 460
  L1 Accesses:                  639
  L2 Accesses:                    5
  RAM Accesses:                   9
  Estimated Cycles:             979

invoke_boxed_msgmf_default
  Instructions:                 504
  L1 Accesses:                  718
  L2 Accesses:                    6
  RAM Accesses:                  26
  Estimated Cycles:            1658

invoke_msgnf_borrowed
  Instructions:                   0
  L1 Accesses:      18446744073709551612
  L2 Accesses:                    2
  RAM Accesses:                   2
  Estimated Cycles:              76

invoke_msgnf_owned
  Instructions:                   0
  L1 Accesses:      18446744073709551612
  L2 Accesses:                    2
  RAM Accesses:                   2
  Estimated Cycles:              76

invoke_msgof_borrowed
  Instructions:                 208
  L1 Accesses:                  302
  L2 Accesses:                    2
  RAM Accesses:                   6
  Estimated Cycles:             522

invoke_msgof_owned
  Instructions:                 249
  L1 Accesses:                  374
  L2 Accesses:                    2
  RAM Accesses:                   7
  Estimated Cycles:             629

invoke_msgsf_borrowed
  Instructions:                 218
  L1 Accesses:                  316
  L2 Accesses:                    6
  RAM Accesses:                   7
  Estimated Cycles:             591

invoke_msgsf_owned
  Instructions:                 303
  L1 Accesses:                  469
  L2 Accesses:                    5
  RAM Accesses:                  16
  Estimated Cycles:            1054

invoke_msgmf_borrowed
  Instructions:                 273
  L1 Accesses:                  411
  L2 Accesses:                    8
  RAM Accesses:                  20
  Estimated Cycles:            1151

invoke_msgmf_owned
  Instructions:                 863
  L1 Accesses:                 1271
  L2 Accesses:                   30
  RAM Accesses:                  25
  Estimated Cycles:            2296

invoke_boxed_msgnf
  Instructions:                  17
  L1 Accesses:                   20
  L2 Accesses:                    3
  RAM Accesses:                   1
  Estimated Cycles:              70

invoke_boxed_msgof
  Instructions:                 509
  L1 Accesses:                  710
  L2 Accesses:                    2
  RAM Accesses:                   7
  Estimated Cycles:             965

invoke_boxed_msgsf
  Instructions:                 521
  L1 Accesses:                  727
  L2 Accesses:                    3
  RAM Accesses:                  10
  Estimated Cycles:            1092

invoke_boxed_msgmf
  Instructions:                 565
  L1 Accesses:                  804
  L2 Accesses:                    5
  RAM Accesses:                  28
  Estimated Cycles:            1809

invoke_protocol_nf_default
  Instructions:                  17
  L1 Accesses:                   21
  L2 Accesses:                    2
  RAM Accesses:                   1
  Estimated Cycles:              66

invoke_protocol_of_default
  Instructions:                 182
  L1 Accesses:                  261
  L2 Accesses:                    3
  RAM Accesses:                   2
  Estimated Cycles:             346

invoke_protocol_sf_default
  Instructions:                 192
  L1 Accesses:                  277
  L2 Accesses:                    3
  RAM Accesses:                   5
  Estimated Cycles:             467

invoke_protocol_mf_default
  Instructions:                 247
  L1 Accesses:                  371
  L2 Accesses:                    7
  RAM Accesses:                  17
  Estimated Cycles:            1001

invoke_protocol_nf_borrowed
  Instructions:                  31
  L1 Accesses:                   38
  L2 Accesses:                    4
  RAM Accesses:                   3
  Estimated Cycles:             163

invoke_protocol_nf_owned
  Instructions:                 564
  L1 Accesses:                  822
  L2 Accesses:                   19
  RAM Accesses:                  11
  Estimated Cycles:            1302

invoke_protocol_of_borrowed
  Instructions:                 214
  L1 Accesses:                  310
  L2 Accesses:                    4
  RAM Accesses:                   5
  Estimated Cycles:             505

invoke_protocol_of_owned
  Instructions:                 744
  L1 Accesses:                 1086
  L2 Accesses:                   22
  RAM Accesses:                  13
  Estimated Cycles:            1651

invoke_protocol_sf_borrowed
  Instructions:                 224
  L1 Accesses:                  320
  L2 Accesses:                    9
  RAM Accesses:                   9
  Estimated Cycles:             680

invoke_protocol_sf_owned
  Instructions:                 764
  L1 Accesses:                 1115
  L2 Accesses:                   25
  RAM Accesses:                  18
  Estimated Cycles:            1870

invoke_protocol_mf_borrowed
  Instructions:                 279
  L1 Accesses:                  420
  L2 Accesses:                    9
  RAM Accesses:                  19
  Estimated Cycles:            1130

invoke_protocol_mf_owned
  Instructions:                 876
  L1 Accesses:                 1285
  L2 Accesses:                   32
  RAM Accesses:                  26
  Estimated Cycles:            2355

invoke_boxed_protocol_nf_default
  Instructions:                 402
  L1 Accesses:                  532
  L2 Accesses:                   12
  RAM Accesses:                  22
  Estimated Cycles:            1362

invoke_boxed_protocol_of_default
  Instructions:                 569
  L1 Accesses:                  777
  L2 Accesses:                   12
  RAM Accesses:                  23
  Estimated Cycles:            1642

invoke_boxed_protocol_sf_default
  Instructions:                 581
  L1 Accesses:                  796
  L2 Accesses:                   12
  RAM Accesses:                  25
  Estimated Cycles:            1731

invoke_boxed_protocol_mf_default
  Instructions:                 635
  L1 Accesses:                  895
  L2 Accesses:                   13
  RAM Accesses:                  34
  Estimated Cycles:            2150

invoke_boxed_protocol_nf
  Instructions:                 429
  L1 Accesses:                  569
  L2 Accesses:                   11
  RAM Accesses:                  23
  Estimated Cycles:            1429

invoke_boxed_protocol_of
  Instructions:                 612
  L1 Accesses:                  839
  L2 Accesses:                   13
  RAM Accesses:                  25
  Estimated Cycles:            1779

invoke_boxed_protocol_sf
  Instructions:                 624
  L1 Accesses:                  858
  L2 Accesses:                   13
  RAM Accesses:                  27
  Estimated Cycles:            1868

invoke_boxed_protocol_mf
  Instructions:                 678
  L1 Accesses:                  957
  L2 Accesses:                   13
  RAM Accesses:                  37
  Estimated Cycles:            2317

invoke_boxed_protocol_mf_inline_always
  Instructions:                 653
  L1 Accesses:                  923
  L2 Accesses:                   13
  RAM Accesses:                  34
  Estimated Cycles:            2178

invoke_boxed_protocol_mf_inline_no_suggestion
  Instructions:                 653
  L1 Accesses:                  923
  L2 Accesses:                   13
  RAM Accesses:                  34
  Estimated Cycles:            2178
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
