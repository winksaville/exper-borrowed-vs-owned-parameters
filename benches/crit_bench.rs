use criterion::{black_box, criterion_group, criterion_main, Criterion};
use exper_borrowed_vs_owned_parameters::{
    invoke_msgmf_borrowed, invoke_msgmf_default, invoke_msgmf_owned, invoke_msgnf_borrowed,
    invoke_msgnf_default, invoke_msgnf_owned, invoke_msgof_borrowed, invoke_msgof_default,
    invoke_msgof_owned, invoke_msgsf_borrowed, invoke_msgsf_default, invoke_msgsf_owned,
};

#[allow(unused)]
fn borrowed(c: &mut Criterion) {
    c.bench_function("borrowed", |b| {
        struct S { v: Vec<u32> }
        fn fn_with_owned_param(param: &S) -> u32 {
            param.v[0] + 1
        }

        let param = S { v: vec![2] };
        b.iter(|| black_box(fn_with_owned_param(&param)));
    });
}

#[allow(unused)]
fn borrowed_with_clone(c: &mut Criterion) {
    c.bench_function("borrowed_with_clone", |b| {
        #[derive(Clone)]
        struct S { v: Vec<u32> }
        fn fn_with_owned_param(param: &S) -> u32 {
            param.v[0] + 1
        }

        let param = S { v: vec![2] };
        b.iter(|| black_box(fn_with_owned_param(&param.clone())));
    });
}

#[allow(unused)]
fn owned(c: &mut Criterion) {
    c.bench_function("owned", |b| {
        #[derive(Clone)]
        struct S { v: Vec<u32> }
        fn fn_with_owned_param(param: S) -> u32 {
            param.v[0] + 1
        }

        let param = S { v: vec![2] };
        b.iter(|| black_box(fn_with_owned_param(param.clone())));
    });
}

#[allow(unused)]
fn compare_msgnf(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_msgnf");
    group.bench_function("msgnf_default", |b| {
        b.iter(|| black_box(invoke_msgnf_default()));
    });
    group.bench_function("borrowed", |b| {
        b.iter(|| black_box(invoke_msgnf_borrowed()));
    });
    group.bench_function("owned", |b| b.iter(|| black_box(invoke_msgnf_owned())));
}

#[allow(unused)]
fn compare_msgof(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_msgof");
    group.bench_function("msgof_default", |b| {
        b.iter(|| black_box(invoke_msgof_default()));
    });
    group.bench_function("borrowed", |b| {
        b.iter(|| black_box(invoke_msgof_borrowed()));
    });
    group.bench_function("owned", |b| b.iter(|| black_box(invoke_msgof_owned())));
}

#[allow(unused)]
fn compare_msgmf(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_msgsf");
    group.bench_function("msgsf_default", |b| {
        b.iter(|| black_box(invoke_msgsf_default()));
    });
    group.bench_function("borrowed", |b| {
        b.iter(|| black_box(invoke_msgsf_borrowed()));
    });
    group.bench_function("owned", |b| b.iter(|| black_box(invoke_msgsf_owned())));
}

#[allow(unused)]
fn compare_msghf(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_msgmf");
    group.bench_function("msgmf_default", |b| {
        b.iter(|| black_box(invoke_msgmf_default()));
    });
    group.bench_function("borrowed", |b| {
        b.iter(|| black_box(invoke_msgmf_borrowed()));
    });
    group.bench_function("owned", |b| b.iter(|| black_box(invoke_msgmf_owned())));
}

criterion_group!(
    benches,
    borrowed,
    borrowed_with_clone,
    owned,
    compare_msgnf,
    compare_msgof,
    compare_msgmf,
    compare_msghf,
);
criterion_main!(benches);