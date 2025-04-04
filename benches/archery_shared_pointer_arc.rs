/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

use archery::*;
use std::ops::Deref;

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn archery_shared_pointer_arc_deref(c: &mut Criterion) {
    let limit = 200_000;

    c.bench_function("archery shared pointer arc deref", move |b| {
        b.iter(|| {
            let rc: SharedPointer<_, ArcK> = SharedPointer::new(42);

            for _ in 0..limit {
                black_box(rc.deref());
            }

            rc
        });
    });
}

fn archery_shared_pointer_arc_clone(c: &mut Criterion) {
    let limit = 100_000;

    c.bench_function("archery shared pointer arc clone and drop", move |b| {
        b.iter_with_setup(
            || Vec::with_capacity(limit),
            |mut vec| {
                vec.resize(limit, SharedPointer::<_, ArcK>::new(42));
                vec
            },
        );
    });
}

criterion_group!(benches, archery_shared_pointer_arc_deref, archery_shared_pointer_arc_clone);
criterion_main!(benches);
