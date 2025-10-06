use crate::shared_pointer::kind::SharedPointerKind;
use alloc::boxed::Box;
use alloc::rc::Rc;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::mem;
use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr;

type UntypedRc = Rc<()>;

/// [Type constructors](https://en.wikipedia.org/wiki/Type_constructor) for
/// [`Rc`] pointers.
pub struct RcK {
    /// We use [`ManuallyDrop`] here, so that we can drop it explicitly as [`Rc<T>`](alloc::rc::Rc).
    /// Not sure if it can be dropped as [`UntypedRc`], but it seems to be playing with fire (even
    /// more than we already are).
    inner: ManuallyDrop<UntypedRc>,
}

impl RcK {
    #[inline(always)]
    fn new_from_inner<T>(rc: Rc<T>) -> RcK {
        RcK { inner: ManuallyDrop::new(unsafe { mem::transmute::<Rc<T>, UntypedRc>(rc) }) }
    }

    #[inline(always)]
    unsafe fn take_inner<T>(self) -> Rc<T> {
        unsafe {
            let rc: UntypedRc = ManuallyDrop::into_inner(self.inner);

            mem::transmute(rc)
        }
    }

    #[inline(always)]
    unsafe fn as_inner_ref<T>(&self) -> &Rc<T> {
        unsafe {
            let rc_t: *const Rc<T> = ptr::from_ref::<UntypedRc>(self.inner.deref()).cast::<Rc<T>>();

            // Static check to make sure we are not messing up the sizes.
            // This could happen if we allowed for `T` to be unsized, because it would need to be
            // represented as a wide pointer inside `Rc`.
            // TODO Use static_assertion when https://github.com/nvzqz/static-assertions-rs/issues/21
            //      gets fixed
            let _ = mem::transmute::<UntypedRc, Rc<T>>;

            &*rc_t
        }
    }

    #[inline(always)]
    unsafe fn as_inner_mut<T>(&mut self) -> &mut Rc<T> {
        unsafe {
            let rc_t: *mut Rc<T> =
                ptr::from_mut::<UntypedRc>(self.inner.deref_mut()).cast::<Rc<T>>();

            &mut *rc_t
        }
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
        unsafe { Rc::as_ptr(self.as_inner_ref()) }
    }

    #[inline(always)]
    unsafe fn deref<T>(&self) -> &T {
        unsafe { self.as_inner_ref::<T>().as_ref() }
    }

    #[inline(always)]
    unsafe fn try_unwrap<T>(self) -> Result<T, RcK> {
        unsafe { Rc::try_unwrap(self.take_inner()).map_err(RcK::new_from_inner) }
    }

    #[inline(always)]
    unsafe fn get_mut<T>(&mut self) -> Option<&mut T> {
        unsafe { Rc::get_mut(self.as_inner_mut()) }
    }

    #[inline(always)]
    unsafe fn make_mut<T: Clone>(&mut self) -> &mut T {
        unsafe { Rc::make_mut(self.as_inner_mut()) }
    }

    #[inline(always)]
    unsafe fn strong_count<T>(&self) -> usize {
        unsafe { Rc::strong_count(self.as_inner_ref::<T>()) }
    }

    #[inline(always)]
    unsafe fn clone<T>(&self) -> RcK {
        unsafe { RcK { inner: ManuallyDrop::new(Rc::clone(self.as_inner_ref())) } }
    }

    #[inline(always)]
    unsafe fn drop<T>(&mut self) {
        unsafe {
            ptr::drop_in_place::<Rc<T>>(self.as_inner_mut());
        }
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
