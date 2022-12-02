# Experiment with borrowed vs owned fn parameters

To make the compiler not optimize away the Message.v
I've created random_array which generates random data
at runtime obligating the compiler to **NOT** assume
anything about the array content.

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

 * invoke_message_borrowed = 559
 * invoke_message_default = 375
 * difference 559 - 375 = 184

 * invoke_messagemf_borrowed = 608
 * invoke_messagemf_default = 496
 * difference 608 - 496 = 112

we only see a small difference because pointers are small,
**NOT, this is an example where the benchmark being inconsistent.
Previously the difference was 116 - 112 = 4.
I'm thinking thsi might have to do with code alignment, see [this search](https://www.google.com/search?q=benchmark+sensitive+to+code+alignment)
and [rust 82232](https://github.com/rust-lang/rust/issues/82232)**:
 * 184 - 112 = 72

But with owned parameters:

 * invoke_message_owned = 621
 * invoke_message_default = 375
 * difference 621 - 375 = 296

 * invoke_messagemf_owned = 928
 * invoke_messagemf_default = 496
 * difference 928 - 496 = 432

a much larger difference is seen because "owned" parameters
are "moved" with a shallow copy and MessageMf is larger than
Message:
 * 432 - 296 = 136

Benchmark runs:
```
wink@3900x 22-12-03T15:45:10.219Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (add-messagemf-and-default)
$ cargo clean
wink@3900x 22-12-03T15:45:13.413Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (add-messagemf-and-default)
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
    Finished bench [optimized] target(s) in 1.49s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-9795cb9e05f14e17)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-1ec3fc8c04023579)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-bc224c1a1ab05086)
invoke_message_default
  Instructions:                 177
  L1 Accesses:                  260
  L2 Accesses:                    2
  RAM Accesses:                   3
  Estimated Cycles:             375

invoke_messagemf_default
  Instructions:                 187
  L1 Accesses:                  276
  L2 Accesses:                    2
  RAM Accesses:                   6
  Estimated Cycles:             496

invoke_message_borrowed
  Instructions:                 208
  L1 Accesses:                  304
  L2 Accesses:                    2
  RAM Accesses:                   7
  Estimated Cycles:             559

invoke_message_owned
  Instructions:                 239
  L1 Accesses:                  366
  L2 Accesses:                    2
  RAM Accesses:                   7
  Estimated Cycles:             621

invoke_messagemf_borrowed
  Instructions:                 218
  L1 Accesses:                  323
  L2 Accesses:                    1
  RAM Accesses:                   8
  Estimated Cycles:             608

invoke_messagemf_owned
  Instructions:                 293
  L1 Accesses:                  468
  L2 Accesses:                    1
  RAM Accesses:                  13
  Estimated Cycles:             928

wink@3900x 22-12-03T15:45:20.595Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (add-messagemf-and-default)
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
  Instructions:                 177 (No change)
  L1 Accesses:                  260 (No change)
  L2 Accesses:                    2 (No change)
  RAM Accesses:                   3 (No change)
  Estimated Cycles:             375 (No change)

invoke_messagemf_default
  Instructions:                 187 (No change)
  L1 Accesses:                  276 (No change)
  L2 Accesses:                    2 (No change)
  RAM Accesses:                   6 (No change)
  Estimated Cycles:             496 (No change)

invoke_message_borrowed
  Instructions:                 208 (No change)
  L1 Accesses:                  304 (No change)
  L2 Accesses:                    2 (No change)
  RAM Accesses:                   7 (No change)
  Estimated Cycles:             559 (No change)

invoke_message_owned
  Instructions:                 239 (No change)
  L1 Accesses:                  366 (No change)
  L2 Accesses:                    2 (No change)
  RAM Accesses:                   7 (No change)
  Estimated Cycles:             621 (No change)

invoke_messagemf_borrowed
  Instructions:                 218 (No change)
  L1 Accesses:                  323 (No change)
  L2 Accesses:                    1 (No change)
  RAM Accesses:                   8 (No change)
  Estimated Cycles:             608 (No change)

invoke_messagemf_owned
  Instructions:                 293 (No change)
  L1 Accesses:                  468 (No change)
  L2 Accesses:                    1 (No change)
  RAM Accesses:                  13 (No change)
  Estimated Cycles:             928 (No change)

wink@3900x 22-12-03T15:45:24.543Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (add-messagemf-and-default)

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
