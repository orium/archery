/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![allow(clippy::eq_op)]

use super::*;
use kind::ArcK;
use kind::RcK;
use static_assertions::assert_impl_all;
use std::cell::Cell;
use std::mem;

assert_impl_all!(SharedPointer<i32, ArcK>: Send, Sync);

#[test]
fn test_deref() {
    let ptr_42: SharedPointer<i32, RcK> = SharedPointer::new(42);
    let ptr_box_dyn_hello: SharedPointer<Box<dyn ToString>, RcK> =
        SharedPointer::new(Box::new("hello"));

    assert_eq!(*ptr_42, 42);
    assert_eq!(ptr_box_dyn_hello.to_string(), "hello");

    assert_eq!(*Borrow::<i32>::borrow(&ptr_42), 42);
    assert_eq!(Borrow::<Box<dyn ToString>>::borrow(&ptr_box_dyn_hello).to_string(), "hello");

    assert_eq!(*ptr_42.as_ref(), 42);
    assert_eq!(ptr_box_dyn_hello.as_ref().to_string(), "hello");
}

#[test]
fn test_try_unwrap() {
    let ptr: SharedPointer<_, RcK> = SharedPointer::new(42);

    assert_eq!(SharedPointer::try_unwrap(ptr).unwrap(), 42);

    let ptr: SharedPointer<_, RcK> = SharedPointer::new(42);
    let ptr_clone = SharedPointer::clone(&ptr);

    let ptr_clone = SharedPointer::try_unwrap(ptr_clone).unwrap_err();
    let ptr = SharedPointer::try_unwrap(ptr).unwrap_err();

    assert_eq!(*ptr, 42);
    assert_eq!(*ptr_clone, 42);
}

#[test]
fn test_get_mut() {
    let mut ptr: SharedPointer<_, RcK> = SharedPointer::new(42);

    assert_eq!(*ptr, 42);

    *SharedPointer::get_mut(&mut ptr).unwrap() += 1;

    assert_eq!(*ptr, 43);

    let mut ptr_clone = SharedPointer::clone(&ptr);

    assert_eq!(SharedPointer::get_mut(&mut ptr), None);
    assert_eq!(SharedPointer::get_mut(&mut ptr_clone), None);

    mem::drop(ptr);

    *SharedPointer::get_mut(&mut ptr_clone).unwrap() += 1;

    assert_eq!(*ptr_clone, 44);
}

#[test]
fn test_strong_count() {
    let ptr: SharedPointer<_, RcK> = SharedPointer::new(42);

    assert_eq!(SharedPointer::strong_count(&ptr), 1);

    let ptr_clone = SharedPointer::clone(&ptr);

    assert_eq!(SharedPointer::strong_count(&ptr), 2);
    assert_eq!(SharedPointer::strong_count(&ptr_clone), 2);

    mem::drop(ptr);

    assert_eq!(SharedPointer::strong_count(&ptr_clone), 1);
}

#[test]
fn test_ptr_eq() {
    let ptr: SharedPointer<_, RcK> = SharedPointer::new(42);
    let ptr_same_content: SharedPointer<_, RcK> = SharedPointer::new(42);
    let ptr_clone: SharedPointer<_, _> = SharedPointer::clone(&ptr);

    assert!(SharedPointer::ptr_eq(&ptr, &ptr));
    assert!(!SharedPointer::ptr_eq(&ptr, &ptr_same_content));
    assert!(SharedPointer::ptr_eq(&ptr, &ptr_clone));
}

#[test]
fn test_make_mut() {
    let mut ptr: SharedPointer<_, RcK> = SharedPointer::new(42);

    assert_eq!(*ptr, 42);

    *SharedPointer::make_mut(&mut ptr) += 1;

    assert_eq!(*ptr, 43);

    // Clone to force make_mut to clone the data.
    let mut ptr_clone = SharedPointer::clone(&ptr);

    assert_eq!(*ptr_clone, 43);

    *SharedPointer::make_mut(&mut ptr_clone) += 1;

    assert_eq!(*ptr, 43);
    assert_eq!(*ptr_clone, 44);

    *SharedPointer::make_mut(&mut ptr) *= 2;

    assert_eq!(*ptr, 2 * 43);
    assert_eq!(*ptr_clone, 44);

    mem::drop(ptr);

    assert_eq!(*ptr_clone, 44);
}

#[test]
fn test_clone() {
    let ptr: SharedPointer<_, RcK> = SharedPointer::new(Cell::new(42));
    let ptr_clone = SharedPointer::clone(&ptr);

    assert_eq!(ptr.get(), 42);
    assert_eq!(ptr_clone.get(), 42);

    ptr_clone.set(3);

    assert_eq!(ptr.get(), 3);
    assert_eq!(ptr_clone.get(), 3);

    mem::drop(ptr);

    assert_eq!(ptr_clone.get(), 3);
}

fn hash<T: Hash, P: SharedPointerKind>(pointer: &SharedPointer<T, P>) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    pointer.hash(&mut hasher);

    hasher.finish()
}

#[test]
fn test_hash() {
    let ptr_42: SharedPointer<_, RcK> = SharedPointer::new(42);
    let ptr_hello: SharedPointer<_, RcK> = SharedPointer::new("hello");

    assert_eq!(hash(&ptr_42), hash(&SharedPointer::<_, RcK>::new(42)));
    assert_eq!(hash(&ptr_hello), hash(&SharedPointer::<_, RcK>::new("hello")));
}

#[test]
fn test_hash_pointer_kind_consistent() {
    let ptr_hello_rc: SharedPointer<_, RcK> = SharedPointer::new("hello");
    let ptr_hello_arc: SharedPointer<_, ArcK> = SharedPointer::new("hello");

    assert_eq!(hash(&ptr_hello_rc), hash(&ptr_hello_arc));
}

#[test]
fn test_eq() {
    let ptr_22: SharedPointer<_, RcK> = SharedPointer::new(22);
    let ptr_42: SharedPointer<_, RcK> = SharedPointer::new(42);

    assert!(ptr_22 == SharedPointer::<_, RcK>::new(22));
    assert!(ptr_22 == SharedPointer::<_, ArcK>::new(22));
    assert!(ptr_22 == ptr_22);
    assert!(!(ptr_22 == SharedPointer::<_, RcK>::new(42)));
    assert!(!(ptr_22 == SharedPointer::<_, ArcK>::new(42)));
    assert!(!(ptr_22 == ptr_42));

    assert!(ptr_22 != SharedPointer::<_, RcK>::new(42));
    assert!(ptr_22 != SharedPointer::<_, ArcK>::new(42));
    assert!(ptr_22 != ptr_42);
    assert!(!(ptr_22 != SharedPointer::<_, RcK>::new(22)));
    assert!(!(ptr_22 != SharedPointer::<_, ArcK>::new(22)));
    assert!(!(ptr_22 != ptr_22));
}

#[allow(clippy::cyclomatic_complexity)]
#[test]
fn test_ord() {
    let ptr_22: SharedPointer<_, RcK> = SharedPointer::new(22);
    let ptr_42: SharedPointer<_, RcK> = SharedPointer::new(42);

    assert_eq!(ptr_22.partial_cmp(&SharedPointer::<_, RcK>::new(22)), Some(Ordering::Equal));
    assert_eq!(ptr_22.partial_cmp(&SharedPointer::<_, RcK>::new(42)), Some(Ordering::Less));
    assert_eq!(ptr_42.partial_cmp(&SharedPointer::<_, RcK>::new(22)), Some(Ordering::Greater));

    assert_eq!(ptr_22.cmp(&SharedPointer::<_, RcK>::new(22)), Ordering::Equal);
    assert_eq!(ptr_22.cmp(&SharedPointer::<_, RcK>::new(42)), Ordering::Less);
    assert_eq!(ptr_42.cmp(&SharedPointer::<_, RcK>::new(22)), Ordering::Greater);

    assert!(ptr_22 < SharedPointer::<_, RcK>::new(42));
    assert!(ptr_22 < SharedPointer::<_, ArcK>::new(42));
    assert!(ptr_22 < ptr_42);
    assert!(!(ptr_42 < SharedPointer::<_, RcK>::new(22)));
    assert!(!(ptr_42 < SharedPointer::<_, ArcK>::new(22)));
    assert!(!(ptr_42 < ptr_22));
    assert!(!(ptr_22 < ptr_22));

    assert!(ptr_22 <= SharedPointer::<_, RcK>::new(42));
    assert!(ptr_22 <= SharedPointer::<_, ArcK>::new(42));
    assert!(ptr_22 <= ptr_42);
    assert!(ptr_22 <= ptr_22);
    assert!(!(ptr_42 <= SharedPointer::<_, RcK>::new(22)));
    assert!(!(ptr_42 <= SharedPointer::<_, ArcK>::new(22)));
    assert!(!(ptr_42 <= ptr_22));

    assert!(ptr_42 > SharedPointer::<_, RcK>::new(22));
    assert!(ptr_42 > SharedPointer::<_, ArcK>::new(22));
    assert!(ptr_42 > ptr_22);
    assert!(!(ptr_22 > SharedPointer::<_, RcK>::new(42)));
    assert!(!(ptr_22 > SharedPointer::<_, ArcK>::new(42)));
    assert!(!(ptr_22 > ptr_42));
    assert!(!(ptr_42 > ptr_42));

    assert!(ptr_42 >= SharedPointer::<_, RcK>::new(22));
    assert!(ptr_42 >= SharedPointer::<_, ArcK>::new(22));
    assert!(ptr_42 >= ptr_22);
    assert!(ptr_42 >= ptr_42);
    assert!(!(ptr_22 >= SharedPointer::<_, RcK>::new(42)));
    assert!(!(ptr_22 >= SharedPointer::<_, ArcK>::new(42)));
    assert!(!(ptr_22 >= ptr_42));
}

#[test]
fn test_default() {
    let ptr: SharedPointer<i32, RcK> = Default::default();

    assert_eq!(*ptr, 0);
}

#[test]
fn test_from_box_t() {
    let ptr: SharedPointer<i32, RcK> = SharedPointer::from(Box::new(42));

    assert_eq!(*ptr, 42);
}

#[test]
fn test_from_t() {
    let ptr: SharedPointer<i32, RcK> = SharedPointer::from(42);

    assert_eq!(*ptr, 42);
}

#[test]
fn test_debug() {
    let ptr: SharedPointer<_, RcK> = SharedPointer::new([1, 2, 3]);

    assert_eq!(format!("{:?}", ptr), "[1, 2, 3]");
}

#[cfg(not(miri))] // Miri doesn't like this one.
#[test]
fn test_fmt_pointer() {
    let ptr: SharedPointer<_, RcK> = SharedPointer::new(314);

    assert_eq!(format!("{:p}", ptr), format!("{:p}", ptr.deref() as *const i32));
}

#[test]
fn test_display() {
    let ptr: SharedPointer<_, RcK> = SharedPointer::new("hello");

    assert_eq!(format!("{}", ptr), "hello");
}
