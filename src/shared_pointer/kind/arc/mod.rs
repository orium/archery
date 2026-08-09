use crate::shared_pointer::kind::SharedPointerKind;
use crate::shared_pointer::kind::erased_ptr::ErasedPtr;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::mem::ManuallyDrop;

/// [Type constructors](https://en.wikipedia.org/wiki/Type_constructor) for
/// [`Arc`] pointers.
pub struct ArcK {
    /// A pointer previously obtained from [`Arc::into_raw()`] for the `T` this instance was
    /// constructed with, and round-tripped through [`Arc::from_raw()`] on every operation. This
    /// avoids relying on the layout of `Arc<T>` (which is not part of Rust's stable ABI) to
    /// perform the type erasure.
    ///
    /// The referenced [`Arc`] allocation is kept alive by this pointer (which owns one strong
    /// reference) until [`SharedPointerKind::drop()`] is called.
    inner: ErasedPtr,
}

// SAFETY: `ArcK` is a type-erased handle to an `Arc<T>` for some `T`. `Arc<T>` is `Send + Sync`
// when `T: Send + Sync`. `ArcK` itself is unconditionally `Send + Sync` because it has no
// knowledge of `T`; the safe wrapper `SharedPointer<T, ArcK>` gates its own `Send`/`Sync` impls
// on `T: Send + Sync` (see the "Wrapping in a safe API" section on `SharedPointerKind`).
unsafe impl Send for ArcK {}
unsafe impl Sync for ArcK {}

impl ArcK {
    #[inline(always)]
    fn new_from_inner<T>(arc: Arc<T>) -> ArcK {
        ArcK { inner: ErasedPtr::new(Arc::into_raw(arc)) }
    }

    /// Reconstructs a non-owning view of the inner [`Arc<T>`].
    ///
    /// The returned [`ManuallyDrop`] must not be unwrapped: dropping the inner [`Arc`] would
    /// decrement a refcount that this instance still logically owns.
    ///
    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`.
    #[inline(always)]
    unsafe fn as_inner<T>(&self) -> ManuallyDrop<Arc<T>> {
        // SAFETY: By the type-parameter invariant, `self.inner` was produced by
        // `Arc::into_raw::<T>` and points to a live `Arc<T>` allocation. Wrapping the
        // reconstructed `Arc` in `ManuallyDrop` prevents it from decrementing the strong count
        // when this local goes out of scope.
        ManuallyDrop::new(unsafe { Arc::from_raw(self.inner.cast::<T>()) })
    }

    /// Takes ownership of the inner [`Arc<T>`], consuming `self`.
    ///
    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`.
    #[inline(always)]
    unsafe fn take_inner<T>(self) -> Arc<T> {
        // SAFETY: By the type-parameter invariant, `self.inner` was produced by
        // `Arc::into_raw::<T>`. `self` is consumed by value and `ArcK` has no `Drop` impl, so
        // the ownership of the strong reference transfers cleanly to the returned `Arc<T>`.
        unsafe { Arc::from_raw(self.inner.cast::<T>()) }
    }
}

unsafe impl SharedPointerKind for ArcK {
    #[inline(always)]
    fn new<T>(v: T) -> ArcK {
        ArcK::new_from_inner(Arc::new(v))
    }

    #[inline(always)]
    fn from_box<T>(v: Box<T>) -> ArcK {
        ArcK::new_from_inner::<T>(Arc::from(v))
    }

    #[inline(always)]
    unsafe fn as_ptr<T>(&self) -> *const T {
        // SAFETY: The type-parameter invariant is forwarded to `ErasedPtr::cast`.
        unsafe { self.inner.cast::<T>() }
    }

    #[inline(always)]
    unsafe fn deref<T>(&self) -> &T {
        // SAFETY: By the type-parameter invariant, `self.inner` was produced by
        // `Arc::into_raw::<T>`, so it points to a valid `T` inside an allocation that is kept
        // alive by `self`. The returned reference is tied to the lifetime of `&self`.
        unsafe { &*self.inner.cast::<T>() }
    }

    #[inline(always)]
    unsafe fn try_unwrap<T>(self) -> Result<T, ArcK> {
        // SAFETY: The type-parameter invariant is forwarded to `take_inner`.
        let arc: Arc<T> = unsafe { self.take_inner::<T>() };

        Arc::try_unwrap(arc).map_err(ArcK::new_from_inner)
    }

    #[inline(always)]
    unsafe fn get_mut<T>(&mut self) -> Option<&mut T> {
        // SAFETY: The type-parameter invariant is forwarded to `map_owned`; `Arc::from_raw` and
        // `Arc::into_raw` are an inverse pair for `Arc<T>`.
        let ret: Option<*mut T> = unsafe {
            self.inner.map_owned::<T, Arc<T>, _>(Arc::from_raw, Arc::as_ptr, |arc| {
                Arc::get_mut(arc).map(core::ptr::from_mut)
            })
        };

        // SAFETY: If `Arc::get_mut` returned `Some`, no other strong or weak reference existed
        // at the time of the call, so we have exclusive access to the `T`. The allocation is
        // kept alive for at least as long as `self` because `self.inner` still owns a strong
        // reference. The returned reference's lifetime is bound to `&mut self`, so no other
        // access through `self` can occur while it is live.
        ret.map(|p| unsafe { &mut *p })
    }

    #[inline(always)]
    unsafe fn make_mut<T: Clone>(&mut self) -> &mut T {
        // SAFETY: The type-parameter invariant is forwarded to `map_owned`; `Arc::from_raw` and
        // `Arc::into_raw` are an inverse pair for `Arc<T>`.
        let ret: *mut T = unsafe {
            self.inner.map_owned::<T, Arc<T>, _>(Arc::from_raw, Arc::as_ptr, |arc| {
                core::ptr::from_mut(Arc::make_mut(arc))
            })
        };

        // SAFETY: `Arc::make_mut` guarantees exclusive access to the (possibly freshly cloned)
        // `T`. The allocation is kept alive by `self.inner`. The returned reference's lifetime
        // is bound to `&mut self`, so no other access through `self` can occur while it is
        // live.
        unsafe { &mut *ret }
    }

    #[inline(always)]
    unsafe fn strong_count<T>(&self) -> usize {
        // SAFETY: The type-parameter invariant is forwarded to `as_inner`.
        let arc: ManuallyDrop<Arc<T>> = unsafe { self.as_inner::<T>() };

        Arc::strong_count(&*arc)
    }

    #[inline(always)]
    unsafe fn clone<T>(&self) -> ArcK {
        // SAFETY: The type-parameter invariant is forwarded to `as_inner`.
        let arc: ManuallyDrop<Arc<T>> = unsafe { self.as_inner::<T>() };

        ArcK::new_from_inner(Arc::clone(&*arc))
    }

    #[inline(always)]
    unsafe fn drop<T>(&mut self) {
        // SAFETY: By the type-parameter invariant, `self.inner` was produced by
        // `Arc::into_raw::<T>`. Reconstructing the `Arc<T>` and letting it drop decrements the
        // strong count matching the initial `Arc::into_raw`. The caller guarantees this is the
        // last use of `self`.
        drop(unsafe { Arc::from_raw(self.inner.cast::<T>()) });
    }
}

impl Debug for ArcK {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("ArcK")
    }
}

#[cfg(test)]
mod test;
