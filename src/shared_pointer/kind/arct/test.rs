use super::*;
use pretty_assertions::assert_eq;
use static_assertions::assert_impl_all;
use std::cell::Cell;
use std::string::ToString;

type PointerKind = ArcTK;

assert_impl_all!(ArcTK: Send, Sync);

#[test]
fn test_from_box_t() {
    let mut ptr = PointerKind::from_box(Box::new(42));

    unsafe {
        assert_eq!(ptr.deref::<i32>(), &42);

        ptr.drop::<i32>();
    }
}

#[test]
fn test_as_ptr() {
    let mut x = PointerKind::new::<&'static str>("hello");

    unsafe {
        let mut y = PointerKind::clone::<&'static str>(&x);
        let x_ptr: *const &'static str = PointerKind::as_ptr(&x);

        assert_eq!(x_ptr, PointerKind::as_ptr(&y));
        assert_eq!(*x_ptr, "hello");

        x.drop::<&'static str>();
        y.drop::<&'static str>();
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

    assert_eq!(format!("{:?}", ptr), "ArcTK");

    unsafe {
        ptr.drop::<i32>();
    }
}

#[test]
fn test_make_mut_panic_safety() {
    use std::panic::AssertUnwindSafe;
    use std::panic::catch_unwind;

    struct PanicOnClone(#[allow(dead_code)] u32);

    impl Clone for PanicOnClone {
        fn clone(&self) -> Self {
            panic!("intentional panic in T::clone");
        }
    }

    let mut ptr = PointerKind::new::<PanicOnClone>(PanicOnClone(42));

    unsafe {
        let mut ptr_clone = ptr.clone::<PanicOnClone>();

        assert_eq!(ptr.strong_count::<PanicOnClone>(), 2);

        // Trigger `make_mut` on a shared handle so it must clone via `T::clone` (which panics).
        let result = catch_unwind(AssertUnwindSafe(|| {
            ptr_clone.make_mut::<PanicOnClone>();
        }));

        assert!(result.is_err(), "make_mut should have unwound");

        // A panic in `T::clone` must not desync the strong count: both handles must still own
        // their strong reference.
        assert_eq!(ptr.strong_count::<PanicOnClone>(), 2);
        assert_eq!(ptr_clone.strong_count::<PanicOnClone>(), 2);

        ptr.drop::<PanicOnClone>();
        ptr_clone.drop::<PanicOnClone>();
    }
}
