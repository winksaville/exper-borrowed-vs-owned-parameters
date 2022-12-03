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
wink@3900x 22-12-03T21:10:18.213Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/exper-borrowed-vs-owned-parameters`
wink@3900x 22-12-03T21:10:19.245Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/exper-borrowed-vs-owned-parameters`
wink@3900x 22-12-03T21:10:25.282Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
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

 * invoke_message_borrowed:    525
 * invoke_message_default:   - 375
 *                difference = 150

 * invoke_messagemf_borrowed:  672
 * invoke_messagemf_default: - 526
 *                difference = 146

we only see a small difference because pointers are small,
 * 150 - 146 = 4

But with owned parameters:

 * invoke_message_owned:       625
 * invoke_message_default:   - 375
 *                difference = 250

 * invoke_messagemf_owned:    1026
 * invoke_messagemf_default: - 526
 *                difference = 500

a much larger difference is seen because "owned" parameters
are "moved" with a shallow copy and MessageMf is larger than
Message:
 * 500 - 250 = 250

Benchmark runs NOTE: the use of `taskset 1` so we always use the same CPU:
```
wink@3900x 22-12-03T21:55:12.832Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo clean ; taskset 1 cargo bench
   Compiling libc v0.2.138
   Compiling cfg-if v1.0.0
   Compiling getrandom v0.2.8
   Compiling rand_core v0.6.4
   Compiling ppv-lite86 v0.2.17
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling volatile v0.4.5
   Compiling exper-borrowed-vs-owned-parameters v0.3.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
   Compiling iai v0.1.1
    Finished bench [optimized] target(s) in 2.94s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-4aa3e300c69dce12)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-113fd2b76ef4a085)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-25abf9316704aeaf)
invoke_message_default
  Instructions:                 151
  L1 Accesses:                  215
  L2 Accesses:                    4
  RAM Accesses:                   4
  Estimated Cycles:             375

invoke_messagemf_default
  Instructions:                 161
  L1 Accesses:                  231
  L2 Accesses:                    3
  RAM Accesses:                   8
  Estimated Cycles:             526

invoke_message_borrowed
  Instructions:                 182
  L1 Accesses:                  260
  L2 Accesses:                    4
  RAM Accesses:                   7
  Estimated Cycles:             525

invoke_message_owned
  Instructions:                 213
  L1 Accesses:                  320
  L2 Accesses:                    5
  RAM Accesses:                   8
  Estimated Cycles:             625

invoke_messagemf_borrowed
  Instructions:                 192
  L1 Accesses:                  277
  L2 Accesses:                    2
  RAM Accesses:                  11
  Estimated Cycles:             672

invoke_messagemf_owned
  Instructions:                 267
  L1 Accesses:                  421
  L2 Accesses:                    2
  RAM Accesses:                  17
  Estimated Cycles:            1026

wink@3900x 22-12-03T21:55:28.091Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
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
