/// Investigate the difference between borrowed and
/// owned parameters to functions.
use rand::{distributions::Uniform, Rng};

struct Message {
    v: Vec<u8>
}

#[inline(never)]
fn message_borrowed(msg: &Message) -> u32 {
    let mut sum = 0u32;
    //for v in msg.v.iter() {
    //    sum += *v as u32;
    //}
    for i in 0..msg.v.len() {
        sum += msg.v[i] as u32
    }
    sum
}

#[inline(never)]
fn message_owned(msg: Message) -> u32 {
    let mut sum = 0u32;
    //for v in msg.v.iter() {
    //    sum += *v as u32;
    //}
    for i in 0..msg.v.len() {
        sum += msg.v[i] as u32
    }
    sum
}

#[inline(never)]
fn random_array(count: usize, start_range: u8, end_range: u8) -> Vec<u8> {
    let range = Uniform::from(start_range..end_range);
    rand::thread_rng().sample_iter(&range).take(count).collect()
}

#[inline(never)]
pub fn invoke_message_borrowed() {
    let msg = Message { v: random_array(5, 0, 20) };
    let r1 = message_borrowed(&msg);
    let msg = Message { v: random_array(5, 30, 50) };
    let r2 = message_borrowed(&msg);
    assert!(r1 != r2);
}

#[inline(never)]
pub fn invoke_message_owned() {
    let msg = Message { v: random_array(5, 0, 20) };
    let r1 = message_owned(msg);
    let msg = Message { v: random_array(5, 30, 50) };
    let r2 = message_owned(msg);
    assert!(r1 != r2);
}
