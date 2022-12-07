use criterion::{black_box, criterion_group, criterion_main, Criterion};
use exper_borrowed_vs_owned_parameters::{
    invoke_boxed_msgmf, invoke_boxed_msgmf_default, invoke_boxed_msgnf, invoke_boxed_msgnf_default,
    invoke_boxed_msgof, invoke_boxed_msgof_default, invoke_boxed_msgsf, invoke_boxed_msgsf_default,
    invoke_boxed_protocol_mf, invoke_boxed_protocol_mf_default,
    invoke_boxed_protocol_mf_inline_always, invoke_boxed_protocol_mf_inline_no_suggestion,
    invoke_boxed_protocol_nf, invoke_boxed_protocol_nf_default, invoke_boxed_protocol_of,
    invoke_boxed_protocol_of_default, invoke_boxed_protocol_sf, invoke_boxed_protocol_sf_default,
    invoke_msgmf_borrowed, invoke_msgmf_default, invoke_msgmf_owned, invoke_msgnf_borrowed,
    invoke_msgnf_default, invoke_msgnf_owned, invoke_msgof_borrowed, invoke_msgof_default,
    invoke_msgof_owned, invoke_msgsf_borrowed, invoke_msgsf_default, invoke_msgsf_owned,
    invoke_protocol_mf_borrowed, invoke_protocol_mf_default, invoke_protocol_mf_owned,
    invoke_protocol_nf_borrowed, invoke_protocol_nf_default, invoke_protocol_nf_owned,
    invoke_protocol_of_borrowed, invoke_protocol_of_default, invoke_protocol_of_owned,
    invoke_protocol_sf_borrowed, invoke_protocol_sf_default, invoke_protocol_sf_owned,
};

#[allow(unused)]
fn borrowed(c: &mut Criterion) {
    c.bench_function("borrowed", |b| {
        struct S {
            v: Vec<u32>,
        }
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
        struct S {
            v: Vec<u32>,
        }
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
        struct S {
            v: Vec<u32>,
        }
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
    group.bench_function("boxed_msgnf_default", |b| {
        b.iter(|| black_box(invoke_boxed_msgnf_default()));
    });
    group.bench_function("boxed", |b| b.iter(|| black_box(invoke_boxed_msgnf())));
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
    group.bench_function("boxed_msgof_default", |b| {
        b.iter(|| black_box(invoke_boxed_msgof_default()));
    });
    group.bench_function("boxed", |b| b.iter(|| black_box(invoke_boxed_msgof())));
}

#[allow(unused)]
fn compare_msgsf(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_msgsf");
    group.bench_function("msgsf_default", |b| {
        b.iter(|| black_box(invoke_msgsf_default()));
    });
    group.bench_function("borrowed", |b| {
        b.iter(|| black_box(invoke_msgsf_borrowed()));
    });
    group.bench_function("owned", |b| b.iter(|| black_box(invoke_msgsf_owned())));
    group.bench_function("boxed_msgsf_default", |b| {
        b.iter(|| black_box(invoke_boxed_msgsf_default()));
    });
    group.bench_function("boxed", |b| b.iter(|| black_box(invoke_boxed_msgsf())));
}

#[allow(unused)]
fn compare_msgmf(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_msgmf");
    group.bench_function("msgmf_default", |b| {
        b.iter(|| black_box(invoke_msgmf_default()));
    });
    group.bench_function("borrowed", |b| {
        b.iter(|| black_box(invoke_msgmf_borrowed()));
    });
    group.bench_function("owned", |b| b.iter(|| black_box(invoke_msgmf_owned())));
    group.bench_function("boxed_msgmf_default", |b| {
        b.iter(|| black_box(invoke_boxed_msgmf_default()));
    });
    group.bench_function("boxed", |b| b.iter(|| black_box(invoke_boxed_msgmf())));
}

#[allow(unused)]
fn compare_protocol_nf(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_protocol_nf");
    group.bench_function("protocol_nf_default", |b| {
        b.iter(|| black_box(invoke_protocol_nf_default()));
    });
    group.bench_function("borrowed", |b| {
        b.iter(|| black_box(invoke_protocol_nf_borrowed()));
    });
    group.bench_function("owned", |b| {
        b.iter(|| black_box(invoke_protocol_nf_owned()))
    });
    group.bench_function("boxed_protocol_nf_default", |b| {
        b.iter(|| black_box(invoke_boxed_protocol_nf_default()));
    });
    group.bench_function("boxed_protocol_nf", |b| {
        b.iter(|| black_box(invoke_boxed_protocol_nf()))
    });
}

#[allow(unused)]
fn compare_protocol_of(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_protocol_of");
    group.bench_function("protocol_of_default", |b| {
        b.iter(|| black_box(invoke_protocol_of_default()));
    });
    group.bench_function("borrowed", |b| {
        b.iter(|| black_box(invoke_protocol_of_borrowed()));
    });
    group.bench_function("owned", |b| {
        b.iter(|| black_box(invoke_protocol_of_owned()))
    });
    group.bench_function("boxed_protocol_of_default", |b| {
        b.iter(|| black_box(invoke_boxed_protocol_of_default()));
    });
    group.bench_function("boxed_protocol_nf", |b| {
        b.iter(|| black_box(invoke_boxed_protocol_of()))
    });
}

#[allow(unused)]
fn compare_protocol_sf(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_protocol_sf");
    group.bench_function("protocol_sf_default", |b| {
        b.iter(|| black_box(invoke_protocol_sf_default()));
    });
    group.bench_function("borrowed", |b| {
        b.iter(|| black_box(invoke_protocol_sf_borrowed()));
    });
    group.bench_function("owned", |b| {
        b.iter(|| black_box(invoke_protocol_sf_owned()))
    });
    group.bench_function("boxed_protocol_sf_default", |b| {
        b.iter(|| black_box(invoke_boxed_protocol_sf_default()));
    });
    group.bench_function("boxed_protocol_nf", |b| {
        b.iter(|| black_box(invoke_boxed_protocol_sf()))
    });
}

#[allow(unused)]
fn compare_protocol_mf(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_protocol_mf");
    group.bench_function("protocol_mf_default", |b| {
        b.iter(|| black_box(invoke_protocol_mf_default()));
    });
    group.bench_function("borrowed", |b| {
        b.iter(|| black_box(invoke_protocol_mf_borrowed()));
    });
    group.bench_function("owned", |b| {
        b.iter(|| black_box(invoke_protocol_mf_owned()))
    });
    group.bench_function("boxed_protocol_mf_default", |b| {
        b.iter(|| black_box(invoke_boxed_protocol_mf_default()));
    });
    group.bench_function("boxed_protocol_nf", |b| {
        b.iter(|| black_box(invoke_boxed_protocol_mf()))
    });
    group.bench_function("boxed_protocol_nf_inline_always", |b| {
        b.iter(|| black_box(invoke_boxed_protocol_mf_inline_always()))
    });
    group.bench_function("boxed_protocol_nf_inline_no_suggestion", |b| {
        b.iter(|| black_box(invoke_boxed_protocol_mf_inline_no_suggestion()))
    });
}

criterion_group!(
    benches,
    borrowed,
    borrowed_with_clone,
    owned,
    compare_msgnf,
    compare_msgof,
    compare_msgsf,
    compare_msgmf,
    compare_protocol_nf,
    compare_protocol_of,
    compare_protocol_sf,
    compare_protocol_mf,
);
criterion_main!(benches);
