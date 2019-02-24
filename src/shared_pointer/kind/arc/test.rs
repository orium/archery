/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use super::*;
use pretty_assertions::assert_eq;
use static_assertions::assert_impl;
use std::cell::Cell;

type PointerKind = SharedPointerKindArc;

assert_impl!(
    shared_pointer_kind_arc_impls_send_sync;
    SharedPointerKindArc,
    Send,
    Sync
);

#[test]
fn test_deref() {
    let mut ptr_42 = PointerKind::new::<i32>(42);
    let mut ptr_box_dyn_hello = PointerKind::new::<Box<dyn ToString>>(Box::new("hello"));

    unsafe {
        assert_eq!(ptr_42.deref::<i32>(), &42);
        assert_eq!(
            ptr_box_dyn_hello.deref::<Box<dyn ToString>>().to_string(),
            "hello"
        );

        ptr_42.drop::<i32>();
        ptr_box_dyn_hello.drop::<Box<dyn ToString>>();
    }
}

#[test]
fn test_clone() {
    let mut ptr_42 = PointerKind::new::<Cell<i32>>(Cell::new(42));

    unsafe {
        let mut ptr_42_clone = ptr_42.clone::<Cell<i32>>();

        assert_eq!(ptr_42.deref::<Cell<i32>>().get(), 42);
        assert_eq!(ptr_42_clone.deref::<Cell<i32>>().get(), 42);

        ptr_42_clone.deref::<Cell<i32>>().set(3);

        assert_eq!(ptr_42.deref::<Cell<i32>>().get(), 3);
        assert_eq!(ptr_42_clone.deref::<Cell<i32>>().get(), 3);

        ptr_42.drop::<Cell<i32>>();

        assert_eq!(ptr_42_clone.deref::<Cell<i32>>().get(), 3);

        ptr_42_clone.drop::<Cell<i32>>();
    }
}

#[test]
fn test_make_mut() {
    let mut ptr_42 = PointerKind::new::<i32>(42);

    unsafe {
        assert_eq!(ptr_42.deref::<i32>(), &42);

        *ptr_42.make_mut::<i32>() += 1;

        assert_eq!(ptr_42.deref::<i32>(), &43);

        // Clone to force make_mut to clone the data.
        let mut ptr_42_clone = ptr_42.clone::<i32>();

        assert_eq!(ptr_42_clone.deref::<i32>(), &43);

        *ptr_42_clone.make_mut::<i32>() += 1;

        assert_eq!(ptr_42.deref::<i32>(), &43);
        assert_eq!(ptr_42_clone.deref::<i32>(), &44);

        *ptr_42.make_mut::<i32>() *= 2;

        assert_eq!(ptr_42.deref::<i32>(), &(2 * 43));
        assert_eq!(ptr_42_clone.deref::<i32>(), &44);

        ptr_42.drop::<i32>();

        assert_eq!(ptr_42_clone.deref::<i32>(), &44);

        ptr_42_clone.drop::<i32>();
    }
}

#[test]
fn test_debug() {
    let mut ptr_42 = PointerKind::new::<i32>(42);

    assert_eq!(format!("{:?}", ptr_42), "SharedPointerKindArc");

    unsafe {
        ptr_42.drop::<i32>();
    }
}
