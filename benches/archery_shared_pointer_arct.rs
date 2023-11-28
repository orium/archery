/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

use archery::*;
use std::ops::Deref;

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn archery_shared_pointer_arct_deref(c: &mut Criterion) {
    let limit = 200_000;

    c.bench_function("archery shared pointer arct deref", move |b| {
        b.iter(|| {
            let rc: SharedPointer<_, ArcTK> = SharedPointer::new(42);

            for _ in 0..limit {
                black_box(rc.deref());
            }

            rc
        });
    });
}

fn archery_shared_pointer_arct_clone(c: &mut Criterion) {
    let limit = 100_000;

    c.bench_function("archery shared pointer arct clone and drop", move |b| {
        b.iter_with_setup(
            || Vec::with_capacity(limit),
            |mut vec| {
                vec.resize(limit, SharedPointer::<_, ArcTK>::new(42));
                vec
            },
        );
    });
}

criterion_group!(benches, archery_shared_pointer_arct_deref, archery_shared_pointer_arct_clone);
criterion_main!(benches);
