# Experiment with borrowed vs owned fn parameters

To make the compiler not optimize away the Message.v
I've created random_array which generates random data
at runtime obligating the compiler to **NOT** assume
anything about the array content.

Here is the library:
```
wink@3900x 22-12-02T01:29:56.450Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cat -n src/lib.rs 
     1  /// Investigate the difference between borrowed and
     2  /// owned parameters to functions.
     3
     4
     5  struct Message {
     6      v: Vec<u8>,
     7  }
     8
     9  #[inline(never)]
    10  fn message_borrowed(msg: &Message) -> u32 {
    11      msg.v[0] as u32
    12  }
    13
    14  #[inline(never)]
    15  fn message_borrowed_idx_loop(msg: &Message) -> u32 {
    16      let mut sum = 0u32;
    17      for i in 0..msg.v.len() {
    18          sum += msg.v[i] as u32
    19      }
    20      sum
    21  }
    22
    23  #[inline(never)]
    24  fn message_borrowed_iter_loop(msg: &Message) -> u32 {
    25      let mut sum = 0u32;
    26      for v in msg.v.iter() {
    27          sum += *v as u32
    28      }
    29      sum
    30  }
    31
    32  #[inline(never)]
    33  fn message_owned(msg: Message) -> (u32, Message) {
    34      (msg.v[0] as u32, msg)
    35  }
    36
    37  #[inline(never)]
    38  fn message_owned_idx_loop(msg: Message) -> (u32, Message) {
    39      let mut sum = 0u32;
    40      for i in 0..msg.v.len() {
    41          sum += msg.v[i] as u32
    42      }
    43      (sum, msg)
    44  }
    45
    46
    47  #[inline(never)]
    48  fn message_owned_iter_loop(msg: Message) -> (u32, Message) {
    49      let mut sum = 0u32;
    50      for v in msg.v.iter() {
    51          sum += *v as u32
    52      }
    53      (sum, msg)
    54  }
    55
    56  #[inline(never)]
    57  pub fn invoke_message_borrowed() {
    58      let msg = Message { v: vec![2] };
    59      let r1 = message_borrowed(&msg);
    60      let r2 = message_borrowed(&msg);
    61      assert!(r1 == 2);
    62      assert!(r1 == r2);
    63  }
    64
    65  #[inline(never)]
    66  pub fn invoke_message_borrowed_idx_loop() {
    67      let msg = Message { v: vec![2] };
    68      let r1 = message_borrowed_idx_loop(&msg);
    69      let r2 = message_borrowed_idx_loop(&msg);
    70      assert!(r1 == 2);
    71      assert!(r1 == r2);
    72  }
    73
    74  #[inline(never)]
    75  pub fn invoke_message_borrowed_iter_loop() {
    76      let msg = Message { v: vec![2] };
    77      let r1 = message_borrowed_iter_loop(&msg);
    78      let r2 = message_borrowed_iter_loop(&msg);
    79      assert!(r1 == 2);
    80      assert!(r1 == r2);
    81  }
    82
    83  #[inline(never)]
    84  pub fn invoke_message_owned() {
    85      let msg = Message { v: vec![3] };
    86      let (r1, msg) = message_owned(msg);
    87      let (r2, _msg) = message_owned(msg);
    88      assert!(r1 == 3);
    89      assert!(r1 == r2);
    90  }
    91
    92  #[inline(never)]
    93  pub fn invoke_message_owned_idx_loop() {
    94      let msg = Message { v: vec![3] };
    95      let (r1, msg) = message_owned_idx_loop(msg);
    96      let (r2, _msg) = message_owned_idx_loop(msg);
    97      assert!(r1 == 3);
    98      assert!(r1 == r2);
    99  }
   100
   101  #[inline(never)]
   102  pub fn invoke_message_owned_iter_loop() {
   103      let msg = Message { v: vec![3] };
   104      let (r1, msg) = message_owned_iter_loop(msg);
   105      let (r2, _msg) = message_owned_iter_loop(msg);
   106      assert!(r1 == 3);
   107      assert!(r1 == r2);
   108  }
```

Here is main.rs:
```
wink@3900x 22-12-01T19:31:45.523Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (wip-black-box)
$ cat -n src/main.rs
     1	use exper_borrowed_vs_owned_parameters::{
     2	    invoke_message_borrowed,
     3	    invoke_message_borrowed_idx_loop,
     4	    invoke_message_borrowed_iter_loop,
     5	    invoke_message_owned,
     6	    invoke_message_owned_idx_loop,
     7	    invoke_message_owned_iter_loop,
     8	};
     9	
    10	
    11	fn main() {
    12	    invoke_message_borrowed();
    13	    invoke_message_borrowed_idx_loop();
    14	    invoke_message_borrowed_iter_loop();
    15	    invoke_message_owned();
    16	    invoke_message_owned_idx_loop();
    17	    invoke_message_owned_iter_loop();
    18	}

```

Here are some runs:
```
wink@3900x 22-12-01T19:31:02.590Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (wip-black-box)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/exper-borrowed-vs-owned-parameters`
wink@3900x 22-12-01T19:32:22.394Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (wip-black-box)
$ cargo run --release
   Compiling exper-borrowed-vs-owned-parameters v0.1.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
    Finished release [optimized] target(s) in 0.21s
     Running `target/release/exper-borrowed-vs-owned-parameters`
wink@3900x 22-12-01T19:32:28.042Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (wip-black-box)

```

## Benchmarks:

From the runs you can see `borrowed` is faster than `owned`. I was
initially surprised because in my mind the `owned` would have just
been passing "pointers"/"references" as `borrowed` is. But no, the
structs are actually "moves" via a shallow copy.
See [here](https://hashrust.com/blog/moves-copies-and-clones-in-rust/).

The xxx_idx_loop and xxx_iter_loop that iterators aren't always
faster. You can see that `message_owned_iter` is a tiny bit slower
than `message_owned_idx_loop` and for `Message { v: vec![0, 1, 2, 3] }`
I've seen that iter is always slower! This is nothing more than an
observation and I'm not sure how much we can trust the `iai` benchmark
tool, so I'm not making any hard assumptions, just presenting the
information and may look at it more closely in the future.

Here is the bench.rs, which is almost exactly like main.rs:
```
wink@3900x 22-12-01T19:33:00.715Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (wip-black-box)
$ cat -n benches/bench.rs 
     1	use exper_borrowed_vs_owned_parameters::{
     2	    invoke_message_borrowed,
     3	    invoke_message_borrowed_idx_loop,
     4	    invoke_message_borrowed_iter_loop,
     5	    invoke_message_owned,
     6	    invoke_message_owned_idx_loop,
     7	    invoke_message_owned_iter_loop,
     8	};
     9	
    10	iai::main!(
    11	    invoke_message_borrowed,
    12	    invoke_message_borrowed_idx_loop,
    13	    invoke_message_borrowed_iter_loop,
    14	    invoke_message_owned,
    15	    invoke_message_owned_idx_loop,
    16	    invoke_message_owned_iter_loop,
    17	);

```

Benchmark runs:
```
wink@3900x 22-12-02T01:39:58.821Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo clean
wink@3900x 22-12-02T01:40:02.836Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
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
   Compiling exper-borrowed-vs-owned-parameters v0.2.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
    Finished bench [optimized] target(s) in 1.54s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-9133e7cce954512a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-689a3e8a4f5acab2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-64cc5dc6a48cc1fe)
invoke_message_borrowed
  Instructions:                 196
  L1 Accesses:                  286
  L2 Accesses:                    1
  RAM Accesses:                   4
  Estimated Cycles:             431

invoke_message_borrowed_idx_loop
  Instructions:                 210
  L1 Accesses:                  293
  L2 Accesses:                    2
  RAM Accesses:                   6
  Estimated Cycles:             513

invoke_message_borrowed_iter_loop
  Instructions:                 190
  L1 Accesses:                  270
  L2 Accesses:                    2
  RAM Accesses:                   4
  Estimated Cycles:             420

invoke_message_owned
  Instructions:                 228
  L1 Accesses:                  348
  L2 Accesses:                    1
  RAM Accesses:                   6
  Estimated Cycles:             563

invoke_message_owned_idx_loop
  Instructions:                 234
  L1 Accesses:                  343
  L2 Accesses:                    2
  RAM Accesses:                   6
  Estimated Cycles:             563

invoke_message_owned_iter_loop
  Instructions:                 238
  L1 Accesses:                  349
  L2 Accesses:                    1
  RAM Accesses:                   5
  Estimated Cycles:             529

wink@3900x 22-12-02T01:40:11.370Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo bench
    Finished bench [optimized] target(s) in 0.00s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-9133e7cce954512a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-689a3e8a4f5acab2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-64cc5dc6a48cc1fe)
invoke_message_borrowed
  Instructions:                 196 (No change)
  L1 Accesses:                  286 (No change)
  L2 Accesses:                    1 (No change)
  RAM Accesses:                   4 (No change)
  Estimated Cycles:             431 (No change)

invoke_message_borrowed_idx_loop
  Instructions:                 210 (No change)
  L1 Accesses:                  293 (No change)
  L2 Accesses:                    2 (No change)
  RAM Accesses:                   6 (No change)
  Estimated Cycles:             513 (No change)

invoke_message_borrowed_iter_loop
  Instructions:                 190 (No change)
  L1 Accesses:                  270 (No change)
  L2 Accesses:                    2 (No change)
  RAM Accesses:                   4 (No change)
  Estimated Cycles:             420 (No change)

invoke_message_owned
  Instructions:                 228 (No change)
  L1 Accesses:                  348 (No change)
  L2 Accesses:                    1 (No change)
  RAM Accesses:                   6 (No change)
  Estimated Cycles:             563 (No change)

invoke_message_owned_idx_loop
  Instructions:                 234 (No change)
  L1 Accesses:                  343 (No change)
  L2 Accesses:                    2 (No change)
  RAM Accesses:                   6 (No change)
  Estimated Cycles:             563 (No change)

invoke_message_owned_iter_loop
  Instructions:                 238 (No change)
  L1 Accesses:                  349 (No change)
  L2 Accesses:                    1 (No change)
  RAM Accesses:                   5 (No change)
  Estimated Cycles:             529 (No change)

```

## Asm code

The assembler code can be found at
[/asm](https://github.com/winksaville/exper-borrowed-vs-owned-parameters/tree/main/asm)
and is generated with `./gen_asm.sh`.


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
