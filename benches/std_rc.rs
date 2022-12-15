/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

use std::ops::Deref;
use std::rc::Rc;

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn std_rc_deref(c: &mut Criterion) {
    let limit = 200_000;

    c.bench_function("std rc deref", move |b| {
        b.iter(|| {
            let rc = Rc::new(42);

            for _ in 0..limit {
                black_box(rc.deref());
            }

            rc
        })
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
        )
    });
}

criterion_group!(benches, std_rc_deref, std_rc_clone_and_drop);
criterion_main!(benches);
