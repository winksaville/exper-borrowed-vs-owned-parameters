/// Investigate the difference between borrowed and
/// owned parameters to functions.

// Message with one vector field
struct Message {
    v: Vec<u8>,
}

impl Default for Message {
    #[inline(never)]
    fn default() -> Self {
        Self { v: vec![2] }
    }
}

// Message with many fields
#[allow(unused)]
struct MessageMf {
    v: Vec<u8>,
    fu128: u128,
    fu64: u64,
    fu32: u32,
    fu16: u16,
    fu8: u8,
    fi128: i128,
    fi64: i64,
    fi32: i32,
    fi16: i16,
    fi8: i8,
}

impl Default for MessageMf {
    #[inline(never)]
    fn default() -> Self {
        Self {
            v: vec![2],
            fu128: 128,
            fu64: 64,
            fu32: 32,
            fu16: 16,
            fu8: 8,
            fi128: 128,
            fi64: 64,
            fi32: 32,
            fi16: 16,
            fi8: 8,
        }
    }
}

#[inline(never)]
fn message_borrowed(msg: &Message) -> u32 {
    msg.v[0] as u32
}

#[inline(never)]
fn message_owned(msg: Message) -> (u32, Message) {
    (msg.v[0] as u32, msg)
}

#[inline(never)]
fn messagemf_borrowed(msg: &MessageMf) -> u32 {
    msg.v[0] as u32
}

#[inline(never)]
fn messagemf_owned(msg: MessageMf) -> (u32, MessageMf) {
    (msg.v[0] as u32, msg)
}

#[inline(never)]
pub fn invoke_message_default() {
    Message::default();
}

#[inline(never)]
pub fn invoke_messagemf_default() {
    MessageMf::default();
}

#[inline(never)]
pub fn invoke_message_borrowed() {
    let msg = Message::default();
    let r1 = message_borrowed(&msg);
    let r2 = message_borrowed(&msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_message_owned() {
    let msg = Message::default();
    let (r1, msg) = message_owned(msg);
    let (r2, _msg) = message_owned(msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}
#[inline(never)]
pub fn invoke_messagemf_borrowed() {
    let msg = MessageMf::default();
    let r1 = messagemf_borrowed(&msg);
    let r2 = messagemf_borrowed(&msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_messagemf_owned() {
    let msg = MessageMf::default();
    let (r1, msg) = messagemf_owned(msg);
    let (r2, _msg) = messagemf_owned(msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}
