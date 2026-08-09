use crate::shared_pointer::kind::SharedPointerKind;
use crate::shared_pointer::kind::erased_ptr::ErasedPtr;
use alloc::boxed::Box;
use alloc::rc::Rc;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::mem::ManuallyDrop;

/// [Type constructors](https://en.wikipedia.org/wiki/Type_constructor) for
/// [`Rc`] pointers.
pub struct RcK {
    /// A pointer previously obtained from [`Rc::into_raw()`] for the `T` this instance was
    /// constructed with, and round-tripped through [`Rc::from_raw()`] on every operation. This
    /// avoids relying on the layout of `Rc<T>` (which is not part of Rust's stable ABI) to
    /// perform the type erasure.
    ///
    /// The referenced [`Rc`] allocation is kept alive by this pointer (which owns one strong
    /// reference) until [`SharedPointerKind::drop()`] is called.
    inner: ErasedPtr,
}

impl RcK {
    #[inline(always)]
    fn new_from_inner<T>(rc: Rc<T>) -> RcK {
        RcK { inner: ErasedPtr::new(Rc::into_raw(rc)) }
    }

    /// Reconstructs a non-owning view of the inner [`Rc<T>`].
    ///
    /// The returned [`ManuallyDrop`] must not be unwrapped: dropping the inner [`Rc`] would
    /// decrement a refcount that this instance still logically owns.
    ///
    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`.
    #[inline(always)]
    unsafe fn as_inner<T>(&self) -> ManuallyDrop<Rc<T>> {
        // SAFETY: By the type-parameter invariant, `self.inner` was produced by
        // `Rc::into_raw::<T>` and points to a live `Rc<T>` allocation. Wrapping the
        // reconstructed `Rc` in `ManuallyDrop` prevents it from decrementing the strong count
        // when this local goes out of scope.
        ManuallyDrop::new(unsafe { Rc::from_raw(self.inner.cast::<T>()) })
    }

    /// Takes ownership of the inner [`Rc<T>`], consuming `self`.
    ///
    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`.
    #[inline(always)]
    unsafe fn take_inner<T>(self) -> Rc<T> {
        // SAFETY: By the type-parameter invariant, `self.inner` was produced by
        // `Rc::into_raw::<T>`. `self` is consumed by value and `RcK` has no `Drop` impl, so the
        // ownership of the strong reference transfers cleanly to the returned `Rc<T>`.
        unsafe { Rc::from_raw(self.inner.cast::<T>()) }
    }
}

unsafe impl SharedPointerKind for RcK {
    #[inline(always)]
    fn new<T>(v: T) -> RcK {
        RcK::new_from_inner(Rc::new(v))
    }

    #[inline(always)]
    fn from_box<T>(v: Box<T>) -> RcK {
        RcK::new_from_inner::<T>(Rc::from(v))
    }

    #[inline(always)]
    unsafe fn as_ptr<T>(&self) -> *const T {
        // SAFETY: The type-parameter invariant is forwarded to `ErasedPtr::cast`.
        unsafe { self.inner.cast::<T>() }
    }

    #[inline(always)]
    unsafe fn deref<T>(&self) -> &T {
        // SAFETY: By the type-parameter invariant, `self.inner` was produced by
        // `Rc::into_raw::<T>`, so it points to a valid `T` inside an allocation that is kept
        // alive by `self`. The returned reference is tied to the lifetime of `&self`.
        unsafe { &*self.inner.cast::<T>() }
    }

    #[inline(always)]
    unsafe fn try_unwrap<T>(self) -> Result<T, RcK> {
        // SAFETY: The type-parameter invariant is forwarded to `take_inner`.
        let rc: Rc<T> = unsafe { self.take_inner::<T>() };

        Rc::try_unwrap(rc).map_err(RcK::new_from_inner)
    }

    #[inline(always)]
    unsafe fn get_mut<T>(&mut self) -> Option<&mut T> {
        // SAFETY: The type-parameter invariant is forwarded to `map_owned`; `Rc::from_raw` and
        // `Rc::into_raw` are an inverse pair for `Rc<T>`.
        let ret: Option<*mut T> = unsafe {
            self.inner.map_owned::<T, Rc<T>, _>(Rc::from_raw, Rc::as_ptr, |rc| {
                Rc::get_mut(rc).map(core::ptr::from_mut)
            })
        };

        // SAFETY: If `Rc::get_mut` returned `Some`, no other strong or weak reference existed
        // at the time of the call, so we have exclusive access to the `T`. The allocation is
        // kept alive for at least as long as `self` because `self.inner` still owns a strong
        // reference. The returned reference's lifetime is bound to `&mut self`, so no other
        // access through `self` can occur while it is live.
        ret.map(|p| unsafe { &mut *p })
    }

    #[inline(always)]
    unsafe fn make_mut<T: Clone>(&mut self) -> &mut T {
        // SAFETY: The type-parameter invariant is forwarded to `map_owned`; `Rc::from_raw` and
        // `Rc::into_raw` are an inverse pair for `Rc<T>`.
        let ret: *mut T = unsafe {
            self.inner.map_owned::<T, Rc<T>, _>(Rc::from_raw, Rc::as_ptr, |rc| {
                core::ptr::from_mut(Rc::make_mut(rc))
            })
        };

        // SAFETY: `Rc::make_mut` guarantees exclusive access to the (possibly freshly cloned)
        // `T`. The allocation is kept alive by `self.inner`. The returned reference's lifetime
        // is bound to `&mut self`, so no other access through `self` can occur while it is
        // live.
        unsafe { &mut *ret }
    }

    #[inline(always)]
    unsafe fn strong_count<T>(&self) -> usize {
        // SAFETY: The type-parameter invariant is forwarded to `as_inner`.
        let rc: ManuallyDrop<Rc<T>> = unsafe { self.as_inner::<T>() };

        Rc::strong_count(&*rc)
    }

    #[inline(always)]
    unsafe fn clone<T>(&self) -> RcK {
        // SAFETY: The type-parameter invariant is forwarded to `as_inner`.
        let rc: ManuallyDrop<Rc<T>> = unsafe { self.as_inner::<T>() };

        RcK::new_from_inner(Rc::clone(&*rc))
    }

    #[inline(always)]
    unsafe fn drop<T>(&mut self) {
        // SAFETY: By the type-parameter invariant, `self.inner` was produced by
        // `Rc::into_raw::<T>`. Reconstructing the `Rc<T>` and letting it drop decrements the
        // strong count matching the initial `Rc::into_raw`. The caller guarantees this is the
        // last use of `self`.
        drop(unsafe { Rc::from_raw(self.inner.cast::<T>()) });
    }
}

impl Debug for RcK {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("RcK")
    }
}

#[cfg(test)]
mod test;
