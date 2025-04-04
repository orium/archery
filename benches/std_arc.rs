/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

use std::ops::Deref;
use std::sync::Arc;

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn std_arc_deref(c: &mut Criterion) {
    let limit = 200_000;

    c.bench_function("std arc deref", move |b| {
        b.iter(|| {
            let rc = Arc::new(42);

            for _ in 0..limit {
                black_box(rc.deref());
            }

            rc
        });
    });
}

fn std_arc_clone(c: &mut Criterion) {
    let limit = 100_000;

    c.bench_function("std arc clone and drop", move |b| {
        b.iter_with_setup(
            || Vec::with_capacity(limit),
            |mut vec| {
                vec.resize(limit, Arc::new(42));
                vec
            },
        );
    });
}

criterion_group!(benches, std_arc_deref, std_arc_clone);
criterion_main!(benches);
