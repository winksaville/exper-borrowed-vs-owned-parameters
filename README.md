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
wink@fwlaptop 22-12-07T03:51:00.697Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (protocol)
$ taskset -c 0 cargo bench
    Finished bench [optimized] target(s) in 0.02s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-3381d17f9a71713f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-181b8e44f009a960)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/crit_bench.rs (target/release/deps/bench_crit-d29bc3fd78f05c7a)
compare_msgnf/msgnf_default
                        time:   [955.76 ps 956.29 ps 956.90 ps]
                        change: [-0.0602% +0.0469% +0.1752%] (p = 0.49 > 0.05)
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  6 (6.00%) high mild
  10 (10.00%) high severe
compare_msgnf/borrowed  time:   [955.56 ps 956.04 ps 956.59 ps]
                        change: [-0.0344% +0.0674% +0.2039%] (p = 0.29 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe
compare_msgnf/owned     time:   [956.42 ps 957.32 ps 958.34 ps]
                        change: [+0.0163% +0.0900% +0.1711%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 17 outliers among 100 measurements (17.00%)
  7 (7.00%) high mild
  10 (10.00%) high severe
compare_msgnf/boxed_msgnf_default
                        time:   [955.97 ps 956.83 ps 957.94 ps]
                        change: [+0.0055% +0.0771% +0.1678%] (p = 0.05 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe
compare_msgnf/boxed     time:   [956.41 ps 957.27 ps 958.49 ps]
                        change: [+0.0074% +0.0847% +0.1780%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

compare_msgof/msgof_default
                        time:   [8.7810 ns 8.7933 ns 8.8056 ns]
                        change: [-0.1430% +0.0968% +0.3435%] (p = 0.44 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
compare_msgof/borrowed  time:   [10.493 ns 10.504 ns 10.515 ns]
                        change: [-0.1380% +0.0732% +0.2636%] (p = 0.47 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
compare_msgof/owned     time:   [20.234 ns 20.252 ns 20.271 ns]
                        change: [+1.6558% +1.7606% +1.8693%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
compare_msgof/boxed_msgof_default
                        time:   [17.947 ns 17.970 ns 17.994 ns]
                        change: [+3.0376% +3.2402% +3.4481%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
compare_msgof/boxed     time:   [19.334 ns 19.356 ns 19.377 ns]
                        change: [+0.9929% +1.2032% +1.4295%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

compare_msgsf/msgsf_default
                        time:   [9.7727 ns 9.7847 ns 9.7970 ns]
                        change: [+2.2154% +2.4688% +2.7384%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
compare_msgsf/borrowed  time:   [11.538 ns 11.549 ns 11.561 ns]
                        change: [+1.2428% +1.4599% +1.6632%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
compare_msgsf/owned     time:   [25.704 ns 25.724 ns 25.745 ns]
                        change: [-0.4219% -0.1272% +0.1975%] (p = 0.48 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
compare_msgsf/boxed_msgsf_default
                        time:   [18.011 ns 18.028 ns 18.045 ns]
                        change: [-0.6903% -0.5043% -0.3088%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild
compare_msgsf/boxed     time:   [20.203 ns 20.220 ns 20.237 ns]
                        change: [+1.7516% +1.9170% +2.0738%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) low mild
  2 (2.00%) high mild

compare_msgmf/msgmf_default
                        time:   [17.947 ns 17.968 ns 17.990 ns]
                        change: [+3.0913% +3.3463% +3.5974%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
compare_msgmf/borrowed  time:   [19.654 ns 19.682 ns 19.711 ns]
                        change: [+3.9934% +4.2495% +4.5465%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
compare_msgmf/owned     time:   [68.896 ns 68.939 ns 68.986 ns]
                        change: [-3.0878% -2.9956% -2.8946%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 11 outliers among 100 measurements (11.00%)
  2 (2.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe
compare_msgmf/boxed_msgmf_default
                        time:   [29.488 ns 29.522 ns 29.563 ns]
                        change: [+9.3585% +9.6261% +9.9096%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe
compare_msgmf/boxed     time:   [29.546 ns 29.570 ns 29.594 ns]
                        change: [+1.9651% +2.1397% +2.2883%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild

compare_protocol_nf/protocol_nf_default
                        time:   [955.51 ps 955.97 ps 956.50 ps]
                        change: [-3.5572% -1.9353% -0.6598%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe
compare_protocol_nf/borrowed
                        time:   [1.6727 ns 1.6736 ns 1.6748 ns]
                        change: [-1.2424% -0.5666% -0.0454%] (p = 0.06 > 0.05)
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe
compare_protocol_nf/owned
                        time:   [46.190 ns 46.219 ns 46.252 ns]
                        change: [-1.6488% -1.4422% -1.2856%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  8 (8.00%) high mild
  3 (3.00%) high severe
compare_protocol_nf/boxed_protocol_nf_default
                        time:   [15.578 ns 15.589 ns 15.601 ns]
                        change: [+4.8693% +5.0227% +5.1819%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe
compare_protocol_nf/boxed_protocol_nf
                        time:   [16.972 ns 16.984 ns 16.998 ns]
                        change: [+0.5092% +0.6572% +0.8264%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  3 (3.00%) high severe

compare_protocol_of/protocol_of_default
                        time:   [8.7700 ns 8.7837 ns 8.7971 ns]
                        change: [-0.4638% -0.2459% -0.0360%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
compare_protocol_of/borrowed
                        time:   [10.842 ns 10.854 ns 10.866 ns]
                        change: [-3.5823% -3.3679% -3.1512%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low severe
compare_protocol_of/owned
                        time:   [55.126 ns 55.153 ns 55.182 ns]
                        change: [-3.3320% -3.2225% -3.0980%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  3 (3.00%) high severe
compare_protocol_of/boxed_protocol_of_default
                        time:   [23.932 ns 23.957 ns 23.983 ns]
                        change: [+3.2748% +3.5294% +3.7628%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
compare_protocol_of/boxed_protocol_nf
                        time:   [25.708 ns 25.735 ns 25.761 ns]
                        change: [-0.7285% -0.5107% -0.2945%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

compare_protocol_sf/protocol_sf_default
                        time:   [9.7651 ns 9.7760 ns 9.7870 ns]
                        change: [+2.0635% +2.3214% +2.5792%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high severe
compare_protocol_sf/borrowed
                        time:   [11.785 ns 11.797 ns 11.808 ns]
                        change: [+2.4909% +2.7577% +3.0261%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
compare_protocol_sf/owned
                        time:   [56.679 ns 56.704 ns 56.734 ns]
                        change: [-4.2138% -4.0539% -3.9221%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  6 (6.00%) high mild
  5 (5.00%) high severe
compare_protocol_sf/boxed_protocol_sf_default
                        time:   [24.255 ns 24.282 ns 24.315 ns]
                        change: [+3.6858% +3.8761% +4.0622%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  4 (4.00%) high severe
compare_protocol_sf/boxed_protocol_nf
                        time:   [25.946 ns 25.967 ns 25.989 ns]
                        change: [+0.0625% +0.3340% +0.5927%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  3 (3.00%) high mild
  2 (2.00%) high severe

compare_protocol_mf/protocol_mf_default
                        time:   [17.960 ns 17.979 ns 17.999 ns]
                        change: [+1.6563% +1.9691% +2.3235%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
compare_protocol_mf/borrowed
                        time:   [19.732 ns 19.755 ns 19.779 ns]
                        change: [+2.2770% +2.4675% +2.6736%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  4 (4.00%) high mild
compare_protocol_mf/owned
                        time:   [74.952 ns 75.013 ns 75.083 ns]
                        change: [+2.7264% +2.9144% +3.0796%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  5 (5.00%) high mild
  3 (3.00%) high severe
compare_protocol_mf/boxed_protocol_mf_default
                        time:   [36.136 ns 36.164 ns 36.194 ns]
                        change: [+11.244% +11.420% +11.613%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  6 (6.00%) high mild
  2 (2.00%) high severe
compare_protocol_mf/boxed_protocol_nf
                        time:   [38.026 ns 38.047 ns 38.070 ns]
                        change: [+10.938% +11.181% +11.378%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  4 (4.00%) high mild
  2 (2.00%) high severe

     Running benches/iai_bench.rs (target/release/deps/iai_bench-2bcbd6eccbfa3e4c)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

wink@fwlaptop 22-12-07T03:57:52.017Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (protocol)

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
