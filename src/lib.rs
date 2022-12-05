/// Investigate the difference between borrowed and
/// owned parameters to functions.

// Message with no fields
#[derive(Default)]
pub struct MsgNf;

// Message with one vector field
pub struct MsgOf {
    v: Vec<u8>,
}

impl Default for MsgOf {
    #[inline(never)]
    fn default() -> Self {
        Self { v: vec![2] }
    }
}

// Message with some fields
#[allow(unused)]
pub struct MsgSf {
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

impl Default for MsgSf {
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

// Message with many fields
#[allow(unused)]
pub struct MsgMf {
    v: Vec<u8>,
    f0u128: u128,
    f0u64: u64,
    f0u32: u32,
    f0u16: u16,
    f0u8: u8,
    f0i128: i128,
    f0i64: i64,
    f0i32: i32,
    f0i16: i16,
    f0i8: i8,
    f1u128: u128,
    f1u64: u64,
    f1u32: u32,
    f1u16: u16,
    f1u8: u8,
    f1i128: i128,
    f1i64: i64,
    f1i32: i32,
    f1i16: i16,
    f1i8: i8,
    f2u128: u128,
    f2u64: u64,
    f2u32: u32,
    f2u16: u16,
    f2u8: u8,
    f2i128: i128,
    f2i64: i64,
    f2i32: i32,
    f2i16: i16,
    f2i8: i8,
    f3u128: u128,
    f3u64: u64,
    f3u32: u32,
    f3u16: u16,
    f3u8: u8,
    f3i128: i128,
    f3i64: i64,
    f3i32: i32,
    f3i16: i16,
    f3i8: i8,
    f4u128: u128,
    f4u64: u64,
    f4u32: u32,
    f4u16: u16,
    f4u8: u8,
    f4i128: i128,
    f4i64: i64,
    f4i32: i32,
    f4i16: i16,
    f4i8: i8,
    f5u128: u128,
    f5u64: u64,
    f5u32: u32,
    f5u16: u16,
    f5u8: u8,
    f5i128: i128,
    f5i64: i64,
    f5i32: i32,
    f5i16: i16,
    f5i8: i8,
    f6u128: u128,
    f6u64: u64,
    f6u32: u32,
    f6u16: u16,
    f6u8: u8,
    f6i128: i128,
    f6i64: i64,
    f6i32: i32,
    f6i16: i16,
    f6i8: i8,
    f7u128: u128,
    f7u64: u64,
    f7u32: u32,
    f7u16: u16,
    f7u8: u8,
    f7i128: i128,
    f7i64: i64,
    f7i32: i32,
    f7i16: i16,
    f7i8: i8,
    f8u128: u128,
    f8u64: u64,
    f8u32: u32,
    f8u16: u16,
    f8u8: u8,
    f8i128: i128,
    f8i64: i64,
    f8i32: i32,
    f8i16: i16,
    f8i8: i8,
    f9u128: u128,
    f9u64: u64,
    f9u32: u32,
    f9u16: u16,
    f9u8: u8,
    f9i128: i128,
    f9i64: i64,
    f9i32: i32,
    f9i16: i16,
    f9i8: i8,
}

impl Default for MsgMf {
    #[inline(never)]
    fn default() -> Self {
        Self {
            v: vec![2],
            f0u128: 128,
            f0u64: 64,
            f0u32: 32,
            f0u16: 16,
            f0u8: 8,
            f0i128: 128,
            f0i64: 64,
            f0i32: 32,
            f0i16: 16,
            f0i8: 8,
            f1u128: 128,
            f1u64: 64,
            f1u32: 32,
            f1u16: 16,
            f1u8: 8,
            f1i128: 128,
            f1i64: 64,
            f1i32: 32,
            f1i16: 16,
            f1i8: 8,
            f2u128: 128,
            f2u64: 64,
            f2u32: 32,
            f2u16: 16,
            f2u8: 8,
            f2i128: 128,
            f2i64: 64,
            f2i32: 32,
            f2i16: 16,
            f2i8: 8,
            f3u128: 128,
            f3u64: 64,
            f3u32: 32,
            f3u16: 16,
            f3u8: 8,
            f3i128: 128,
            f3i64: 64,
            f3i32: 32,
            f3i16: 16,
            f3i8: 8,
            f4u128: 128,
            f4u64: 64,
            f4u32: 32,
            f4u16: 16,
            f4u8: 8,
            f4i128: 128,
            f4i64: 64,
            f4i32: 32,
            f4i16: 16,
            f4i8: 8,
            f5u128: 128,
            f5u64: 64,
            f5u32: 32,
            f5u16: 16,
            f5u8: 8,
            f5i128: 128,
            f5i64: 64,
            f5i32: 32,
            f5i16: 16,
            f5i8: 8,
            f6u128: 128,
            f6u64: 64,
            f6u32: 32,
            f6u16: 16,
            f6u8: 8,
            f6i128: 128,
            f6i64: 64,
            f6i32: 32,
            f6i16: 16,
            f6i8: 8,
            f7u128: 128,
            f7u64: 64,
            f7u32: 32,
            f7u16: 16,
            f7u8: 8,
            f7i128: 128,
            f7i64: 64,
            f7i32: 32,
            f7i16: 16,
            f7i8: 8,
            f8u128: 128,
            f8u64: 64,
            f8u32: 32,
            f8u16: 16,
            f8u8: 8,
            f8i128: 128,
            f8i64: 64,
            f8i32: 32,
            f8i16: 16,
            f8i8: 8,
            f9u128: 128,
            f9u64: 64,
            f9u32: 32,
            f9u16: 16,
            f9u8: 8,
            f9i128: 128,
            f9i64: 64,
            f9i32: 32,
            f9i16: 16,
            f9i8: 8,
        }
    }
}

#[inline(never)]
pub fn msgnf_borrowed(msg: &MsgNf) -> u32 {
    match msg {
        MsgNf => 2,
    }
}

#[inline(never)]
pub fn msgnf_owned(msg: MsgNf) -> (u32, MsgNf) {
    let v = match msg {
        MsgNf => 2,
    };
    (v, MsgNf)
}

#[inline(never)]
pub fn msgof_borrowed(msg: &MsgOf) -> u32 {
    msg.v[0] as u32
}

#[inline(never)]
pub fn msgof_owned(msg: MsgOf) -> (u32, MsgOf) {
    (msg.v[0] as u32, msg)
}

#[inline(never)]
pub fn msgsf_borrowed(msg: &MsgSf) -> u32 {
    msg.v[0] as u32
}

#[inline(never)]
pub fn msgsf_owned(msg: MsgSf) -> (u32, MsgSf) {
    (msg.v[0] as u32, msg)
}

#[inline(never)]
pub fn msgmf_borrowed(msg: &MsgMf) -> u32 {
    msg.v[0] as u32
}

#[inline(never)]
pub fn msgmf_owned(msg: MsgMf) -> (u32, MsgMf) {
    (msg.v[0] as u32, msg)
}

#[inline(never)]
pub fn invoke_msgnf_default() {
    MsgNf::default();
}

#[inline(never)]
pub fn invoke_msgof_default() {
    MsgOf::default();
}

#[inline(never)]
pub fn invoke_msgsf_default() {
    MsgSf::default();
}

#[inline(never)]
pub fn invoke_msgmf_default() {
    MsgMf::default();
}

#[inline(never)]
pub fn invoke_msgnf_borrowed() {
    let msg = MsgNf::default();
    let r1 = msgnf_borrowed(&msg);
    let r2 = msgnf_borrowed(&msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_msgnf_owned() {
    let msg = MsgNf::default();
    let (r1, msg) = msgnf_owned(msg);
    let (r2, _msg) = msgnf_owned(msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_msgof_borrowed() {
    let msg = MsgOf::default();
    let r1 = msgof_borrowed(&msg);
    let r2 = msgof_borrowed(&msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_msgof_owned() {
    let msg = MsgOf::default();
    let (r1, msg) = msgof_owned(msg);
    let (r2, _msg) = msgof_owned(msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_msgsf_borrowed() {
    let msg = MsgSf::default();
    let r1 = msgsf_borrowed(&msg);
    let r2 = msgsf_borrowed(&msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_msgsf_owned() {
    let msg = MsgSf::default();
    let (r1, msg) = msgsf_owned(msg);
    let (r2, _msg) = msgsf_owned(msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_msgmf_borrowed() {
    let msg = MsgMf::default();
    let r1 = msgmf_borrowed(&msg);
    let r2 = msgmf_borrowed(&msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_msgmf_owned() {
    let msg = MsgMf::default();
    let (r1, msg) = msgmf_owned(msg);
    let (r2, _msg) = msgmf_owned(msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_boxed_msgof_default() {
    Box::<MsgOf>::default();
}

#[inline(never)]
pub fn invoke_boxed_msgsf_default() {
    Box::<MsgSf>::default();
}

#[inline(never)]
pub fn invoke_boxed_msgmf_default() {
    Box::<MsgMf>::default();
}

#[inline(never)]
pub fn invoke_boxed_msgnf_default() {
    Box::<MsgNf>::default();
}

#[inline(never)]
fn boxed_msgnf(msg: Box<MsgNf>) -> (u32, Box<MsgNf>) {
    let v = match *msg {
        MsgNf => 2,
    };
    (v, msg)
}

#[inline(never)]
pub fn boxed_msgof(msg: Box<MsgOf>) -> (u32, Box<MsgOf>) {
    (msg.v[0] as u32, msg)
}

#[inline(never)]
pub fn boxed_msgsf(msg: Box<MsgSf>) -> (u32, Box<MsgSf>) {
    (msg.v[0] as u32, msg)
}

#[inline(never)]
pub fn boxed_msgmf(msg: Box<MsgMf>) -> (u32, Box<MsgMf>) {
    (msg.v[0] as u32, msg)
}

#[inline(never)]
pub fn invoke_boxed_msgnf() {
    let msg = Box::<MsgNf>::default();
    let (r1, msg) = boxed_msgnf(msg);
    let (r2, _msg) = boxed_msgnf(msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_boxed_msgof() {
    let msg = Box::<MsgOf>::default();
    let (r1, msg) = boxed_msgof(msg);
    let (r2, _msg) = boxed_msgof(msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_boxed_msgsf() {
    let msg = Box::<MsgSf>::default();
    let (r1, msg) = boxed_msgsf(msg);
    let (r2, _msg) = boxed_msgsf(msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}

#[inline(never)]
pub fn invoke_boxed_msgmf() {
    let msg = Box::<MsgMf>::default();
    let (r1, msg) = boxed_msgmf(msg);
    let (r2, _msg) = boxed_msgmf(msg);
    assert!(r1 == 2);
    assert!(r1 == r2);
}
