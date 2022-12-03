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

 * invoke_message_borrowed:    487
 * invoke_message_default:   - 341
 *                difference = 146

 * invoke_messagemf_borrowed:  570
 * invoke_messagemf_default: - 454
 *                difference = 116

we only see a small difference because pointers are small (bigger than I'd hoped for!)),
 * 146 - 116 = 30

But with owned parameters:

 * invoke_message_owned:       587
 * invoke_message_default:   - 341
 *                difference = 246

 * invoke_messagemf_owned:     928
 * invoke_messagemf_default: - 454
 *                difference = 474

a much larger difference is seen because "owned" parameters
are "moved" with a shallow copy and MessageMf is larger than
Message:
 * 474 - 246 = 228

Benchmark runs. NOTE: the use of `taskset 1` so we always use the same CPU.
I also ran this **without** clean and the previous run was without
`invoke_messagemf_borrowed`. So below we see the "(change)" after adding
`invoke_messagemf_borrowed2`. Alas, there is non-trivial differences in
the output of `taskset 1 cargo bench`. Of note the number of instructions
were identical, but in most of the other lines there are differences :(
```
wink@3900x 22-12-03T22:12:43.165Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ taskset -c 1 cargo bench
   Compiling exper-borrowed-vs-owned-parameters v0.3.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
    Finished bench [optimized] target(s) in 0.56s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-4aa3e300c69dce12)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-113fd2b76ef4a085)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-25abf9316704aeaf)
invoke_message_default
  Instructions:                 151 (No change)
  L1 Accesses:                  216 (+0.465116%)
  L2 Accesses:                    4 (No change)
  RAM Accesses:                   3 (-25.00000%)
  Estimated Cycles:             341 (-9.066667%)

invoke_messagemf_default
  Instructions:                 161 (No change)
  L1 Accesses:                  234 (+1.298701%)
  L2 Accesses:                    2 (-33.33333%)
  RAM Accesses:                   6 (-25.00000%)
  Estimated Cycles:             454 (-13.68821%)

invoke_message_borrowed
  Instructions:                 182 (No change)
  L1 Accesses:                  262 (+0.769231%)
  L2 Accesses:                    3 (-25.00000%)
  RAM Accesses:                   6 (-14.28571%)
  Estimated Cycles:             487 (-7.238095%)

invoke_message_owned
  Instructions:                 213 (No change)
  L1 Accesses:                  322 (+0.625000%)
  L2 Accesses:                    4 (-20.00000%)
  RAM Accesses:                   7 (-12.50000%)
  Estimated Cycles:             587 (-6.080000%)

invoke_messagemf_borrowed
  Instructions:                 192 (No change)
  L1 Accesses:                  280 (+1.083032%)
  L2 Accesses:                    2 (No change)
  RAM Accesses:                   8 (-27.27273%)
  Estimated Cycles:             570 (-15.17857%)

invoke_messagemf_owned
  Instructions:                 267 (No change)
  L1 Accesses:                  423 (+0.475059%)
  L2 Accesses:                    3 (+50.00000%)
  RAM Accesses:                  14 (-17.64706%)
  Estimated Cycles:             928 (-9.551657%)

invoke_messagemf_borrowed2
  Instructions:                 192
  L1 Accesses:                  282
  L2 Accesses:                    1
  RAM Accesses:                   7
  Estimated Cycles:             532

wink@3900x 22-12-03T22:12:55.881Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
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
