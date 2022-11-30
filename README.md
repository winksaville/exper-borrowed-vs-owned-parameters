# Experiment with borrowed vs owned fn parameters

To make the compiler not optimize away the Message.v
I've created random_array which generates random data
at runtime obligating the compiler to **NOT** assume
anything about the array content.

Here is the library:
```
wink@3900x 22-11-30T18:39:22.649Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cat -n src/lib.rs
     1	/// Investigate the difference between borrowed and
     2	/// owned parameters to functions.
     3	use rand::{distributions::Uniform, Rng};
     4	
     5	struct Message {
     6	    v: Vec<u8>
     7	}
     8	
     9	#[inline(never)]
    10	fn message_borrowed(msg: &Message) -> u32 {
    11	    let mut sum = 0u32;
    12	    //for v in msg.v.iter() {
    13	    //    sum += *v as u32;
    14	    //}
    15	    for i in 0..msg.v.len() {
    16	        sum += msg.v[i] as u32
    17	    }
    18	    sum
    19	}
    20	
    21	#[inline(never)]
    22	fn message_owned(msg: Message) -> u32 {
    23	    let mut sum = 0u32;
    24	    //for v in msg.v.iter() {
    25	    //    sum += *v as u32;
    26	    //}
    27	    for i in 0..msg.v.len() {
    28	        sum += msg.v[i] as u32
    29	    }
    30	    sum
    31	}
    32	
    33	#[inline(never)]
    34	fn random_array(count: usize, start_range: u8, end_range: u8) -> Vec<u8> {
    35	    let range = Uniform::from(start_range..end_range);
    36	    rand::thread_rng().sample_iter(&range).take(count).collect()
    37	}
    38	
    39	#[inline(never)]
    40	pub fn invoke_message_borrowed() {
    41	    let msg = Message { v: random_array(5, 0, 20) };
    42	    let r1 = message_borrowed(&msg);
    43	    let msg = Message { v: random_array(5, 30, 50) };
    44	    let r2 = message_borrowed(&msg);
    45	    assert!(r1 != r2);
    46	}
    47	
    48	#[inline(never)]
    49	pub fn invoke_message_owned() {
    50	    let msg = Message { v: random_array(5, 0, 20) };
    51	    let r1 = message_owned(msg);
    52	    let msg = Message { v: random_array(5, 30, 50) };
    53	    let r2 = message_owned(msg);
    54	    assert!(r1 != r2);
    55	}
```

Here is main.rs:
```
wink@3900x 22-11-30T18:41:29.537Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cat -n src/main.rs
     1  use exper_borrowed_vs_owned_parameters::{invoke_message_borrowed, invoke_message_owned};
     2
     3  fn main() {
     4      invoke_message_borrowed();
     5      invoke_message_owned();
     6  }
```

Here are some runs:
```
wink@3900x 22-11-30T18:40:07.298Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/exper-borrowed-vs-owned-parameters`
wink@3900x 22-11-30T18:40:09.544Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/exper-borrowed-vs-owned-parameters`
```

## Benchmarks:

Here is the bench.rs:
```
wink@3900x 22-11-30T18:40:24.125Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cat -n benches/bench.rs
     1	use exper_borrowed_vs_owned_parameters::{invoke_message_borrowed, invoke_message_owned};
     2	
     3	iai::main!(
     4	    invoke_message_borrowed,
     5	    invoke_message_owned,
     6	);
```

Benchmark runs:
```
wink@3900x 22-11-30T18:43:15.610Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo clean
wink@3900x 22-11-30T18:43:50.230Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
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
   Compiling exper-borrowed-vs-owned-parameters v0.1.0 (/home/wink/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters)
    Finished bench [optimized] target(s) in 1.53s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-ce8142689a9f18f0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-6c3af512bd8f03a6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-13d87267f3a54c65)
invoke_message_borrowed
  Instructions:                3286
  L1 Accesses:                 4326
  L2 Accesses:                   14
  RAM Accesses:                 109
  Estimated Cycles:            8211

invoke_message_owned
  Instructions:                3160
  L1 Accesses:                 4176
  L2 Accesses:                   14
  RAM Accesses:                 108
  Estimated Cycles:            8026

wink@3900x 22-11-30T18:43:57.303Z:~/prgs/rust/myrepos/exper-borrowed-vs-owned-parameters (main)
$ cargo bench
    Finished bench [optimized] target(s) in 0.00s
     Running unittests src/lib.rs (target/release/deps/exper_borrowed_vs_owned_parameters-ce8142689a9f18f0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/exper_borrowed_vs_owned_parameters-6c3af512bd8f03a6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bench.rs (target/release/deps/bench-13d87267f3a54c65)
invoke_message_borrowed
  Instructions:                3286 (No change)
  L1 Accesses:                 4326 (No change)
  L2 Accesses:                   14 (No change)
  RAM Accesses:                 109 (No change)
  Estimated Cycles:            8211 (No change)

invoke_message_owned
  Instructions:                3160 (No change)
  L1 Accesses:                 4176 (No change)
  L2 Accesses:                   14 (No change)
  RAM Accesses:                 108 (No change)
  Estimated Cycles:            8026 (No change)
```

## Asm code

The assembler code can be found at
https://github.com/winksaville/exper-borrowed-vs-owned-parameters/tree/main/asm

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
