use core::mem::ManuallyDrop;

/// A type-erased raw pointer to a reference-counted allocation.
///
/// This is the storage shared between all [`SharedPointerKind`](super::SharedPointerKind)
/// implementations. It centralizes the [`*const ()`](pointer) representation used to erase the
/// element type `T` from the pointer, and offers the small set of `unsafe` primitives needed to
/// recover a typed pointer from it.
///
/// The pointer itself must have been produced by a smart pointer's `into_raw()` (for example
/// [`Rc::into_raw()`](alloc::rc::Rc::into_raw)) for some concrete `T`.
///
/// # Type-parameter invariant
///
/// Each instance is logically associated with a fixed `T`, chosen when the instance is
/// constructed. All `unsafe` methods must be called with that same `T`.
pub(crate) struct ErasedPtr {
    ptr: *const (),
}

impl ErasedPtr {
    #[inline(always)]
    pub fn new<T>(ptr: *const T) -> ErasedPtr {
        ErasedPtr { ptr: ptr.cast::<()>() }
    }

    /// Recovers the raw pointer typed as `*const T`.
    ///
    /// # Safety
    ///
    /// `T` must be the type this instance was constructed with.
    #[inline(always)]
    pub unsafe fn cast<T>(&self) -> *const T {
        self.ptr.cast::<T>()
    }

    /// Temporarily reconstructs an owned smart pointer from the stored raw pointer, hands it to
    /// `f` for mutation, then stores the (possibly updated) pointer back.
    ///
    /// `from_raw` and `as_ptr` must correspond to the smart pointer type this instance was
    /// constructed with, typically `P::from_raw` and `P::as_ptr` for some reference-counted
    /// pointer type `P` (for example [`Rc<T>`](alloc::rc::Rc)).
    ///
    /// This pairs the pointer consumption and the pointer replacement in a single primitive,
    /// preventing the leak that would occur if a caller updated the stored pointer without
    /// first consuming the previous one.
    ///
    /// # Panic safety
    ///
    /// The write-back happens through a RAII guard, so `self.ptr` is restored on both the
    /// normal return and unwind paths. If `f` panics, the smart pointer reconstructed from the
    /// stored raw pointer is [`ManuallyDrop`]-wrapped, so the strong reference it represents is
    /// preserved: `self.ptr` will point to the same allocation it did before the call.
    ///
    /// # Safety
    ///
    /// `T` must be the type this instance was constructed with, and `P` must be the smart
    /// pointer type it was constructed with.
    #[inline(always)]
    pub unsafe fn map_owned<T, P, R>(
        &mut self,
        from_raw: unsafe fn(*const T) -> P,
        as_ptr: fn(&P) -> *const T,
        f: impl FnOnce(&mut P) -> R,
    ) -> R {
        /// RAII guard that snapshots the current raw pointer from `owned` back into `slot` on
        /// drop, without consuming `owned`. Runs on both normal-return and unwind paths.
        struct WriteBack<'a, T, P> {
            slot: &'a mut *const (),
            owned: ManuallyDrop<P>,
            as_ptr: fn(&P) -> *const T,
        }

        impl<T, P> Drop for WriteBack<'_, T, P> {
            #[inline(always)]
            fn drop(&mut self) {
                *self.slot = (self.as_ptr)(&self.owned).cast::<()>();
                // `owned` is a `ManuallyDrop<P>` with no `Drop` glue: the strong reference the
                // raw pointer represents is preserved as `self.ptr` transitions from the local
                // to the slot.
            }
        }

        // SAFETY: by the caller's guarantee, `from_raw` matches the smart pointer type used at
        // construction and `T` matches the type used at construction.
        let owned: P = unsafe { from_raw(self.ptr.cast::<T>()) };
        let mut guard =
            WriteBack::<T, P> { slot: &mut self.ptr, owned: ManuallyDrop::new(owned), as_ptr };

        f(&mut guard.owned)
    }
}
