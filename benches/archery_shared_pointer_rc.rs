#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

use archery::*;
use std::ops::Deref;

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn archery_shared_pointer_rc_deref(c: &mut Criterion) {
    let limit = 200_000;

    c.bench_function("archery shared pointer rc deref", move |b| {
        b.iter(|| {
            let rc: SharedPointer<_, RcK> = SharedPointer::new(42);

            for _ in 0..limit {
                black_box(rc.deref());
            }

            rc
        });
    });
}

fn archery_shared_pointer_rc_clone(c: &mut Criterion) {
    let limit = 100_000;

    c.bench_function("archery shared pointer rc clone and drop", move |b| {
        b.iter_with_setup(
            || Vec::with_capacity(limit),
            |mut vec| {
                vec.resize(limit, SharedPointer::<_, RcK>::new(42));
                vec
            },
        );
    });
}

criterion_group!(benches, archery_shared_pointer_rc_deref, archery_shared_pointer_rc_clone);
criterion_main!(benches);
