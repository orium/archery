/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::shared_pointer::kind::SharedPointerKind;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::mem;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr;
use std::rc::Rc;

type UntypedRc = Rc<()>;

/// [Type constructors](https://en.wikipedia.org/wiki/Type_constructor) for
/// [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) pointers.
pub struct RcK {
    /// We use `ManuallyDrop` here, so that we can drop it explicitly as `Rc<T>`.  Not sure if it
    /// can be dropped as `UntypedRc`, but it seems to be playing with fire (even more than we
    /// already are).
    inner: ManuallyDrop<UntypedRc>,
}

impl RcK {
    #[inline(always)]
    fn new_from_inner<T>(rc: Rc<T>) -> RcK {
        RcK { inner: ManuallyDrop::new(unsafe { mem::transmute(rc) }) }
    }

    #[inline(always)]
    unsafe fn take_inner<T>(self) -> Rc<T> {
        let rc: UntypedRc = ManuallyDrop::into_inner(self.inner);

        mem::transmute(rc)
    }

    #[inline(always)]
    unsafe fn as_inner_ref<T>(&self) -> &Rc<T> {
        let rc_t: *const Rc<T> = self.inner.deref() as *const UntypedRc as *const Rc<T>;

        // Static check to make sure we are not messing up the sizes.
        // This could happen if we allowed for `T` to be unsized, because it would need to be
        // represented as a wide pointer inside `Rc`.
        // TODO Use static_assertion when https://github.com/nvzqz/static-assertions-rs/issues/21
        //      gets fixed
        let _ = mem::transmute::<UntypedRc, Rc<T>>;

        &*rc_t
    }

    #[inline(always)]
    unsafe fn as_inner_mut<T>(&mut self) -> &mut Rc<T> {
        let rc_t: *mut Rc<T> = self.inner.deref_mut() as *mut UntypedRc as *mut Rc<T>;

        &mut *rc_t
    }
}

impl SharedPointerKind for RcK {
    #[inline(always)]
    fn new<T>(v: T) -> RcK {
        RcK::new_from_inner(Rc::new(v))
    }

    #[inline(always)]
    fn from_box<T>(v: Box<T>) -> RcK {
        RcK::new_from_inner::<T>(Rc::from(v))
    }

    #[inline(always)]
    unsafe fn deref<T>(&self) -> &T {
        self.as_inner_ref::<T>().as_ref()
    }

    #[inline(always)]
    unsafe fn try_unwrap<T>(self) -> Result<T, RcK> {
        Rc::try_unwrap(self.take_inner()).map_err(RcK::new_from_inner)
    }

    #[inline(always)]
    unsafe fn get_mut<T>(&mut self) -> Option<&mut T> {
        Rc::get_mut(self.as_inner_mut())
    }

    #[inline(always)]
    unsafe fn make_mut<T: Clone>(&mut self) -> &mut T {
        Rc::make_mut(self.as_inner_mut())
    }

    #[inline(always)]
    unsafe fn strong_count<T>(&self) -> usize {
        Rc::strong_count(self.as_inner_ref::<T>())
    }

    #[inline(always)]
    unsafe fn clone<T>(&self) -> RcK {
        RcK { inner: ManuallyDrop::new(Rc::clone(self.as_inner_ref())) }
    }

    #[inline(always)]
    unsafe fn drop<T>(&mut self) {
        ptr::drop_in_place::<Rc<T>>(self.as_inner_mut());
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
