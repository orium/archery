/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::shared_pointer::kind::SharedPointerKind;
use static_assertions::assert_eq_size;
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

pub struct SharedPointerKindRc {
    // WIP! Explain that we use ManuallyDrop so we can drop it as `Rc<T>`
    inner: ManuallyDrop<UntypedRc>,
}

impl SharedPointerKindRc {
    #[inline(always)]
    unsafe fn as_inner_ref<T>(&self) -> &Rc<T> {
        let rc_t: *const Rc<T> = self.inner.deref() as *const UntypedRc as *const Rc<T>;

        // Static check to make sure we are not messing up the sizes.
        // This could happen if we allowed for `T` to be unsized, because it would need to be
        // represented as a wide pointer inside `Rc`.
        assert_eq_size!(UntypedRc, Rc<T>);

        &*rc_t
    }

    #[inline(always)]
    unsafe fn as_inner_mut<T>(&mut self) -> &mut Rc<T> {
        let rc_t: *mut Rc<T> = self.inner.deref_mut() as *mut UntypedRc as *mut Rc<T>;

        &mut *rc_t
    }
}

impl SharedPointerKind for SharedPointerKindRc {
    #[inline(always)]
    fn new<T>(v: T) -> SharedPointerKindRc {
        let rc: Rc<T> = Rc::new(v);

        SharedPointerKindRc {
            inner: ManuallyDrop::new(unsafe { mem::transmute(rc) }),
        }
    }

    #[inline(always)]
    unsafe fn clone<T>(&self) -> SharedPointerKindRc {
        SharedPointerKindRc {
            inner: ManuallyDrop::new(Rc::clone(self.as_inner_ref())),
        }
    }

    #[inline(always)]
    unsafe fn make_mut<T>(&mut self) -> &mut T
    where
        T: Clone,
    {
        Rc::make_mut(self.as_inner_mut())
    }

    #[inline(always)]
    unsafe fn deref<T>(&self) -> &T {
        self.as_inner_ref::<T>().as_ref()
    }

    #[inline(always)]
    unsafe fn drop<T>(&mut self) {
        ptr::drop_in_place::<Rc<T>>(self.as_inner_mut());
    }
}

impl Debug for SharedPointerKindRc {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("SharedPointerKindRc")
    }
}

#[cfg(test)]
mod test;
