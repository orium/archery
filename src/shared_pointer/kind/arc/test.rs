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
fn test_from_box_t() {
    let mut ptr = PointerKind::from_box(Box::new(42));

    unsafe {
        assert_eq!(ptr.deref::<i32>(), &42);

        ptr.drop::<i32>();
    }
}

#[test]
fn test_deref() {
    let mut ptr_42 = PointerKind::new::<i32>(42);
    let mut ptr_box_dyn_hello = PointerKind::new::<Box<dyn ToString>>(Box::new("hello"));

    unsafe {
        assert_eq!(ptr_42.deref::<i32>(), &42);
        assert_eq!(ptr_box_dyn_hello.deref::<Box<dyn ToString>>().to_string(), "hello");

        ptr_42.drop::<i32>();
        ptr_box_dyn_hello.drop::<Box<dyn ToString>>();
    }
}

#[test]
fn test_try_unwrap() {
    let ptr = PointerKind::new::<i32>(42);

    unsafe {
        assert_eq!(ptr.try_unwrap::<i32>().unwrap(), 42);
    }

    let ptr = PointerKind::new::<i32>(42);

    unsafe {
        let ptr_clone = ptr.clone::<i32>();

        let mut ptr_clone = ptr_clone.try_unwrap::<i32>().unwrap_err();
        let mut ptr = ptr.try_unwrap::<i32>().unwrap_err();

        assert_eq!(ptr.deref::<i32>(), &42);
        assert_eq!(ptr_clone.deref::<i32>(), &42);

        ptr.drop::<i32>();
        ptr_clone.drop::<i32>();
    }
}

#[test]
fn test_get_mut() {
    let mut ptr = PointerKind::new::<i32>(42);

    unsafe {
        assert_eq!(ptr.deref::<i32>(), &42);

        *ptr.get_mut::<i32>().unwrap() += 1;

        assert_eq!(ptr.deref::<i32>(), &43);

        let mut ptr_clone = ptr.clone::<i32>();

        assert_eq!(ptr.get_mut::<i32>(), None);
        assert_eq!(ptr_clone.get_mut::<i32>(), None);

        ptr.drop::<i32>();

        *ptr_clone.get_mut::<i32>().unwrap() += 1;

        assert_eq!(ptr_clone.deref::<i32>(), &44);

        ptr_clone.drop::<i32>();
    }
}

#[test]
fn test_make_mut() {
    let mut ptr = PointerKind::new::<i32>(42);

    unsafe {
        assert_eq!(ptr.deref::<i32>(), &42);

        *ptr.make_mut::<i32>() += 1;

        assert_eq!(ptr.deref::<i32>(), &43);

        // Clone to force make_mut to clone the data.
        let mut ptr_clone = ptr.clone::<i32>();

        assert_eq!(ptr_clone.deref::<i32>(), &43);

        *ptr_clone.make_mut::<i32>() += 1;

        assert_eq!(ptr.deref::<i32>(), &43);
        assert_eq!(ptr_clone.deref::<i32>(), &44);

        *ptr.make_mut::<i32>() *= 2;

        assert_eq!(ptr.deref::<i32>(), &(2 * 43));
        assert_eq!(ptr_clone.deref::<i32>(), &44);

        ptr.drop::<i32>();

        assert_eq!(ptr_clone.deref::<i32>(), &44);

        ptr_clone.drop::<i32>();
    }
}

#[test]
fn test_strong_count() {
    let mut ptr = PointerKind::new::<i32>(42);

    unsafe {
        assert_eq!(ptr.strong_count::<i32>(), 1);

        let mut ptr_clone = ptr.clone::<i32>();

        assert_eq!(ptr.strong_count::<i32>(), 2);
        assert_eq!(ptr_clone.strong_count::<i32>(), 2);

        ptr.drop::<i32>();

        assert_eq!(ptr_clone.strong_count::<i32>(), 1);

        ptr_clone.drop::<i32>();
    }
}

#[test]
fn test_clone() {
    let mut ptr = PointerKind::new::<Cell<i32>>(Cell::new(42));

    unsafe {
        let mut ptr_clone = ptr.clone::<Cell<i32>>();

        assert_eq!(ptr.deref::<Cell<i32>>().get(), 42);
        assert_eq!(ptr_clone.deref::<Cell<i32>>().get(), 42);

        ptr_clone.deref::<Cell<i32>>().set(3);

        assert_eq!(ptr.deref::<Cell<i32>>().get(), 3);
        assert_eq!(ptr_clone.deref::<Cell<i32>>().get(), 3);

        ptr.drop::<Cell<i32>>();

        assert_eq!(ptr_clone.deref::<Cell<i32>>().get(), 3);

        ptr_clone.drop::<Cell<i32>>();
    }
}

#[test]
fn test_debug() {
    let mut ptr = PointerKind::new::<i32>(42);

    assert_eq!(format!("{:?}", ptr), "SharedPointerKindArc");

    unsafe {
        ptr.drop::<i32>();
    }
}
