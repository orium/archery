use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::ops::Deref;
use std::rc::Rc;

fn std_rc_deref(c: &mut Criterion) {
    let limit = 200_000;

    c.bench_function("std rc deref", move |b| {
        b.iter(|| {
            let rc = Rc::new(42);

            for _ in 0..limit {
                black_box(rc.deref());
            }

            rc
        });
    });
}

fn std_rc_clone_and_drop(c: &mut Criterion) {
    let limit = 100_000;

    c.bench_function("std rc clone and drop", move |b| {
        b.iter_with_setup(
            || Vec::with_capacity(limit),
            |mut vec| {
                vec.resize(limit, Rc::new(42));
                vec
            },
        );
    });
}

criterion_group!(benches, std_rc_deref, std_rc_clone_and_drop);
criterion_main!(benches);
