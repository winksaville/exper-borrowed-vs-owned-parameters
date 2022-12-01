/// Investigate the difference between borrowed and
/// owned parameters to functions.
use std::hint::black_box;

struct Message {
    v: Vec<u8>
}

#[inline(never)]
fn message_borrowed(msg: &Message) -> u32 {
    msg.v[0] as u32
}

#[inline(never)]
fn message_borrowed_idx_loop(msg: &Message) -> u32 {
    let mut sum = 0u32;
    for i in 0..msg.v.len() {
        sum += msg.v[i] as u32
    }
    sum
}

#[inline(never)]
fn message_borrowed_iter_loop(msg: &Message) -> u32 {
    let mut sum = 0u32;
    for v in msg.v.iter() {
        sum += *v as u32
    }
    sum
}

#[inline(never)]
fn message_owned(msg: Message) -> (u32, Message) {
    (msg.v[0] as u32, msg)
}

#[inline(never)]
fn message_owned_idx_loop(msg: Message) -> (u32, Message) {
    let mut sum = 0u32;
    for i in 0..msg.v.len() {
        sum += msg.v[i] as u32
    }
    (sum, msg)
}


#[inline(never)]
fn message_owned_iter_loop(msg: Message) -> (u32, Message) {
    let mut sum = 0u32;
    for v in msg.v.iter() {
        sum += *v as u32
    }
    (sum, msg)
}

#[inline(never)]
pub fn invoke_message_borrowed() {
    let msg = Message { v: vec![2] };
    let r1 = black_box(message_borrowed(&msg));
    let r2 = black_box(message_borrowed(&msg));
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_message_borrowed_idx_loop() {
    let msg = Message { v: vec![2] };
    let r1 = black_box(message_borrowed_idx_loop(&msg));
    let r2 = black_box(message_borrowed_idx_loop(&msg));
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_message_borrowed_iter_loop() {
    let msg = Message { v: vec![2] };
    let r1 = black_box(message_borrowed_iter_loop(&msg));
    let r2 = black_box(message_borrowed_iter_loop(&msg));
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_message_owned() {
    let msg = Message { v: vec![3] };
    let (r1, msg) = black_box(message_owned(msg));
    let (r2, _msg) = black_box(message_owned(msg));
    assert!(r1 == 3);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_message_owned_idx_loop() {
    let msg = Message { v: vec![3] };
    let (r1, msg) = black_box(message_owned_idx_loop(msg));
    let (r2, _msg) = black_box(message_owned_idx_loop(msg));
    assert!(r1 == 3);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_message_owned_iter_loop() {
    let msg = Message { v: vec![3] };
    let (r1, msg) = black_box(message_owned_iter_loop(msg));
    let (r2, _msg) = black_box(message_owned_iter_loop(msg));
    assert!(r1 == 3);
    assert!(r1 == r2);
}
