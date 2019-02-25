/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![allow(clippy::eq_op)]

use super::*;
use kind::SharedPointerKindArc;
use kind::SharedPointerKindRc;
use static_assertions::assert_impl;
use std::cell::Cell;
use std::mem;

assert_impl!(
    shared_pointer_arc_impls_send_sync;
    SharedPointer<i32, SharedPointerKindArc>,
    Send, Sync
);

#[test]
fn test_deref() {
    let ptr_42: SharedPointer<i32, SharedPointerKindRc> = SharedPointer::new(42);
    let ptr_box_dyn_hello: SharedPointer<Box<dyn ToString>, SharedPointerKindRc> =
        SharedPointer::new(Box::new("hello"));

    assert_eq!(*ptr_42, 42);
    assert_eq!(ptr_box_dyn_hello.to_string(), "hello");

    assert_eq!(*Borrow::<i32>::borrow(&ptr_42), 42);
    assert_eq!(
        Borrow::<Box<dyn ToString>>::borrow(&ptr_box_dyn_hello).to_string(),
        "hello"
    );

    assert_eq!(*ptr_42.as_ref(), 42);
    assert_eq!(ptr_box_dyn_hello.as_ref().to_string(), "hello");
}

#[test]
fn test_clone() {
    let ptr_42: SharedPointer<_, SharedPointerKindRc> = SharedPointer::new(Cell::new(42));
    let ptr_42_clone = SharedPointer::clone(&ptr_42);

    assert_eq!(ptr_42.get(), 42);
    assert_eq!(ptr_42_clone.get(), 42);

    ptr_42_clone.set(3);

    assert_eq!(ptr_42.get(), 3);
    assert_eq!(ptr_42_clone.get(), 3);

    mem::drop(ptr_42);

    assert_eq!(ptr_42_clone.get(), 3);
}

#[test]
fn test_make_mut() {
    let mut ptr_42: SharedPointer<_, SharedPointerKindRc> = SharedPointer::new(42);

    assert_eq!(*ptr_42, 42);

    *SharedPointer::make_mut(&mut ptr_42) += 1;

    assert_eq!(*ptr_42, 43);

    // Clone to force make_mut to clone the data.
    let mut ptr_42_clone = SharedPointer::clone(&ptr_42);

    assert_eq!(*ptr_42_clone, 43);

    *SharedPointer::make_mut(&mut ptr_42_clone) += 1;

    assert_eq!(*ptr_42, 43);
    assert_eq!(*ptr_42_clone, 44);

    *SharedPointer::make_mut(&mut ptr_42) *= 2;

    assert_eq!(*ptr_42, 2 * 43);
    assert_eq!(*ptr_42_clone, 44);

    mem::drop(ptr_42);

    assert_eq!(*ptr_42_clone, 44);
}

fn hash<T: Hash>(pointer: &SharedPointer<T, SharedPointerKindRc>) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    pointer.hash(&mut hasher);

    hasher.finish()
}

#[test]
fn test_hash() {
    let ptr_42: SharedPointer<_, SharedPointerKindRc> = SharedPointer::new(42);
    let ptr_hello: SharedPointer<_, SharedPointerKindRc> = SharedPointer::new("hello");

    assert_eq!(hash(&ptr_42), hash(&SharedPointer::new(42)));
    assert_eq!(hash(&ptr_hello), hash(&SharedPointer::new("hello")));
}

#[test]
fn test_eq() {
    let ptr_22: SharedPointer<_, SharedPointerKindRc> = SharedPointer::new(22);
    let ptr_42: SharedPointer<_, SharedPointerKindRc> = SharedPointer::new(42);

    assert!(ptr_22 == SharedPointer::<_, SharedPointerKindRc>::new(22));
    assert!(ptr_22 == SharedPointer::<_, SharedPointerKindArc>::new(22));
    assert!(ptr_22 == ptr_22);
    assert!(!(ptr_22 == SharedPointer::<_, SharedPointerKindRc>::new(42)));
    assert!(!(ptr_22 == SharedPointer::<_, SharedPointerKindArc>::new(42)));
    assert!(!(ptr_22 == ptr_42));

    assert!(ptr_22 != SharedPointer::<_, SharedPointerKindRc>::new(42));
    assert!(ptr_22 != SharedPointer::<_, SharedPointerKindArc>::new(42));
    assert!(ptr_22 != ptr_42);
    assert!(!(ptr_22 != SharedPointer::<_, SharedPointerKindRc>::new(22)));
    assert!(!(ptr_22 != SharedPointer::<_, SharedPointerKindArc>::new(22)));
    assert!(!(ptr_22 != ptr_22));
}

#[allow(clippy::cyclomatic_complexity)]
#[test]
fn test_ord() {
    let ptr_22: SharedPointer<_, SharedPointerKindRc> = SharedPointer::new(22);
    let ptr_42: SharedPointer<_, SharedPointerKindRc> = SharedPointer::new(42);

    assert_eq!(
        ptr_22.partial_cmp(&SharedPointer::<_, SharedPointerKindRc>::new(22)),
        Some(Ordering::Equal)
    );
    assert_eq!(
        ptr_22.partial_cmp(&SharedPointer::<_, SharedPointerKindRc>::new(42)),
        Some(Ordering::Less)
    );
    assert_eq!(
        ptr_42.partial_cmp(&SharedPointer::<_, SharedPointerKindRc>::new(22)),
        Some(Ordering::Greater)
    );

    assert_eq!(
        ptr_22.cmp(&SharedPointer::<_, SharedPointerKindRc>::new(22)),
        Ordering::Equal
    );
    assert_eq!(
        ptr_22.cmp(&SharedPointer::<_, SharedPointerKindRc>::new(42)),
        Ordering::Less
    );
    assert_eq!(
        ptr_42.cmp(&SharedPointer::<_, SharedPointerKindRc>::new(22)),
        Ordering::Greater
    );

    assert!(ptr_22 < SharedPointer::<_, SharedPointerKindRc>::new(42));
    assert!(ptr_22 < SharedPointer::<_, SharedPointerKindArc>::new(42));
    assert!(ptr_22 < ptr_42);
    assert!(!(ptr_42 < SharedPointer::<_, SharedPointerKindRc>::new(22)));
    assert!(!(ptr_42 < SharedPointer::<_, SharedPointerKindArc>::new(22)));
    assert!(!(ptr_42 < ptr_22));
    assert!(!(ptr_22 < ptr_22));

    assert!(ptr_22 <= SharedPointer::<_, SharedPointerKindRc>::new(42));
    assert!(ptr_22 <= SharedPointer::<_, SharedPointerKindArc>::new(42));
    assert!(ptr_22 <= ptr_42);
    assert!(ptr_22 <= ptr_22);
    assert!(!(ptr_42 <= SharedPointer::<_, SharedPointerKindRc>::new(22)));
    assert!(!(ptr_42 <= SharedPointer::<_, SharedPointerKindArc>::new(22)));
    assert!(!(ptr_42 <= ptr_22));

    assert!(ptr_42 > SharedPointer::<_, SharedPointerKindRc>::new(22));
    assert!(ptr_42 > SharedPointer::<_, SharedPointerKindArc>::new(22));
    assert!(ptr_42 > ptr_22);
    assert!(!(ptr_22 > SharedPointer::<_, SharedPointerKindRc>::new(42)));
    assert!(!(ptr_22 > SharedPointer::<_, SharedPointerKindArc>::new(42)));
    assert!(!(ptr_22 > ptr_42));
    assert!(!(ptr_42 > ptr_42));

    assert!(ptr_42 >= SharedPointer::<_, SharedPointerKindRc>::new(22));
    assert!(ptr_42 >= SharedPointer::<_, SharedPointerKindArc>::new(22));
    assert!(ptr_42 >= ptr_22);
    assert!(ptr_42 >= ptr_42);
    assert!(!(ptr_22 >= SharedPointer::<_, SharedPointerKindRc>::new(42)));
    assert!(!(ptr_22 >= SharedPointer::<_, SharedPointerKindArc>::new(42)));
    assert!(!(ptr_22 >= ptr_42));
}

#[test]
fn test_debug() {
    let ptr: SharedPointer<_, SharedPointerKindRc> = SharedPointer::new([1, 2, 3]);

    assert_eq!(format!("{:?}", ptr), "[1, 2, 3]");
}

#[test]
fn test_display() {
    let ptr: SharedPointer<_, SharedPointerKindRc> = SharedPointer::new("hello");

    assert_eq!(format!("{}", ptr), "hello");
}
