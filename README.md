# Experiment with borrowed vs owned fn parameters

To make the compiler not optimize away code I'm using #[inline(never)].
**But an aggravating problem is that although multiple runs of `cargo bench`
yield identical results, as can be see by the second run always reports
`(No change)`. I do not get consistent results when I make changes to
the code. Such as adding a new test or re-ordering tests I can get
differing results.**

Here is the library [lib.rs](/src/lib.rs)

Here is [main.rs](/src/main.rs):

Here are some runs:
```
wink@3900x 22-12-02T21:46:33.755Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (add-messagemf-and-default)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/exper-borrowed-vs-owned-parameters`
wink@3900x 22-12-02T21:46:38.353Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (add-messagemf-and-default)
$ cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/exper-borrowed-vs-owned-parameters`
wink@3900x 22-12-02T21:46:40.902Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (add-messagemf-and-default)
```

## Benchmarks:

From the runs you can see `borrowed` is significantly faster than `owned`. I was
initially surprised because in my mind the `owned` would have just
been passing "pointers"/"references" as `borrowed` is. But no, the
with `owned` items they are actually "moved" via a shallow copy.
See [here](https://hashrust.com/blog/moves-copies-and-clones-in-rust/).

The invoke_message_default and invoke_messagefm_default times
can be subtracted from other times to see the effect of message
passing more directly.

For example, borrowed parameters:

 * invoke_message_borrowed = 421
 * invoke_message_default = 339
 * difference 421 - 339 = 82

 * invoke_messagemf_borrowed = 436
 * invoke_messagemf_default = 392
 * difference 436 - 392 = 44

we only see a small difference because pointers are small,
**NOT, this is an example where the benchmark being inconsistent.
Previously the difference was 116 - 112 = 4.
So, I added `#![feature(fn_align)]` and used `#[repr(align(32))]` and
that really didn't help :(**
 * 82 - 44 = 38

But with owned parameters:

 * invoke_message_owned = 551
 * invoke_message_default = 339
 * difference 551 - 339 = 212

 * invoke_messagemf_owned = 824
 * invoke_messagemf_default = 392
 * difference 824 - 392 = 432

a much larger difference is seen because "owned" parameters
are "moved" with a shallow copy and MessageMf is larger than
Message:
 * 432 - 212 = 220

Benchmark runs:
```
wink@3900x 22-12-03T17:25:40.634Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (add-fn_align)
$ cargo clean
wink@3900x 22-12-03T17:25:45.333Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (add-fn_align)
$ cargo bench
   Compiling libc v0.2.137
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling volatile v0.4.5
   Compiling iai v0.1.1
   Compiling getrandom v0.2.8
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling exper-borrowed-vs-owned-parameters v0.3.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
    Finished bench [optimized] target(s) in 1.52s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-9795cb9e05f14e17)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-1ec3fc8c04023579)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-bc224c1a1ab05086)
invoke_message_default
  Instructions:                 149
  L1 Accesses:                  214
  L2 Accesses:                    4
  RAM Accesses:                   3
  Estimated Cycles:             339

invoke_messagemf_default
  Instructions:                 159
  L1 Accesses:                  232
  L2 Accesses:                    4
  RAM Accesses:                   4
  Estimated Cycles:             392

invoke_message_borrowed
  Instructions:                 180
  L1 Accesses:                  261
  L2 Accesses:                    4
  RAM Accesses:                   4
  Estimated Cycles:             421

invoke_message_owned
  Instructions:                 211
  L1 Accesses:                  321
  L2 Accesses:                    4
  RAM Accesses:                   6
  Estimated Cycles:             551

invoke_messagemf_borrowed
  Instructions:                 190
  L1 Accesses:                  281
  L2 Accesses:                    3
  RAM Accesses:                   4
  Estimated Cycles:             436

invoke_messagemf_owned
  Instructions:                 265
  L1 Accesses:                  424
  L2 Accesses:                    3
  RAM Accesses:                  11
  Estimated Cycles:             824

wink@3900x 22-12-03T17:25:52.025Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (add-fn_align)
$ cargo bench
    Finished bench [optimized] target(s) in 0.00s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-9795cb9e05f14e17)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-1ec3fc8c04023579)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-bc224c1a1ab05086)
invoke_message_default
  Instructions:                 149 (No change)
  L1 Accesses:                  214 (No change)
  L2 Accesses:                    4 (No change)
  RAM Accesses:                   3 (No change)
  Estimated Cycles:             339 (No change)

invoke_messagemf_default
  Instructions:                 159 (No change)
  L1 Accesses:                  232 (No change)
  L2 Accesses:                    4 (No change)
  RAM Accesses:                   4 (No change)
  Estimated Cycles:             392 (No change)

invoke_message_borrowed
  Instructions:                 180 (No change)
  L1 Accesses:                  261 (No change)
  L2 Accesses:                    4 (No change)
  RAM Accesses:                   4 (No change)
  Estimated Cycles:             421 (No change)

invoke_message_owned
  Instructions:                 211 (No change)
  L1 Accesses:                  321 (No change)
  L2 Accesses:                    4 (No change)
  RAM Accesses:                   6 (No change)
  Estimated Cycles:             551 (No change)

invoke_messagemf_borrowed
  Instructions:                 190 (No change)
  L1 Accesses:                  281 (No change)
  L2 Accesses:                    3 (No change)
  RAM Accesses:                   4 (No change)
  Estimated Cycles:             436 (No change)

invoke_messagemf_owned
  Instructions:                 265 (No change)
  L1 Accesses:                  424 (No change)
  L2 Accesses:                    3 (No change)
  RAM Accesses:                  11 (No change)
  Estimated Cycles:             824 (No change)

wink@3900x 22-12-03T17:26:04.538Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (add-fn_align)
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
