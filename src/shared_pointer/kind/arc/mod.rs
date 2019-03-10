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
use std::sync::Arc;

type UntypedArc = Arc<()>;

/// [Type constructors](https://en.wikipedia.org/wiki/Type_constructor) for
/// [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) pointers.
pub struct SharedPointerKindArc {
    /// We use `ManuallyDrop` here, so that we can drop it explicitly as `Arc<T>`.  Not sure if it
    /// can be dropped as `UntypedArc`, but it seems to be playing with fire (even more than we
    /// already are).
    inner: ManuallyDrop<UntypedArc>,
}

impl SharedPointerKindArc {
    #[inline(always)]
    fn new_from_inner<T>(arc: Arc<T>) -> SharedPointerKindArc {
        SharedPointerKindArc {
            inner: ManuallyDrop::new(unsafe { mem::transmute(arc) }),
        }
    }

    #[inline(always)]
    unsafe fn take_inner<T>(self) -> Arc<T> {
        let arc: UntypedArc = ManuallyDrop::into_inner(self.inner);

        mem::transmute(arc)
    }

    #[inline(always)]
    unsafe fn as_inner_ref<T>(&self) -> &Arc<T> {
        let arc_t: *const Arc<T> = self.inner.deref() as *const UntypedArc as *const Arc<T>;

        // Static check to make sure we are not messing up the sizes.
        // This could happen if we allowed for `T` to be unsized, because it would need to be
        // represented as a wide pointer inside `Arc`.
        static_assertions::assert_eq_size!(UntypedArc, Arc<T>);

        &*arc_t
    }

    #[inline(always)]
    unsafe fn as_inner_mut<T>(&mut self) -> &mut Arc<T> {
        let arc_t: *mut Arc<T> = self.inner.deref_mut() as *mut UntypedArc as *mut Arc<T>;

        &mut *arc_t
    }
}

impl SharedPointerKind for SharedPointerKindArc {
    #[inline(always)]
    fn new<T>(v: T) -> SharedPointerKindArc {
        SharedPointerKindArc::new_from_inner(Arc::new(v))
    }

    #[inline(always)]
    fn from_box<T>(v: Box<T>) -> SharedPointerKindArc {
        SharedPointerKindArc::new_from_inner::<T>(Arc::from(v))
    }

    #[inline(always)]
    unsafe fn deref<T>(&self) -> &T {
        self.as_inner_ref::<T>().as_ref()
    }

    #[inline(always)]
    unsafe fn try_unwrap<T>(self) -> Result<T, SharedPointerKindArc> {
        Arc::try_unwrap(self.take_inner())
            .map_err(|inner| SharedPointerKindArc::new_from_inner(inner))
    }

    #[inline(always)]
    unsafe fn get_mut<T>(&mut self) -> Option<&mut T> {
        Arc::get_mut(self.as_inner_mut())
    }

    #[inline(always)]
    unsafe fn make_mut<T: Clone>(&mut self) -> &mut T {
        Arc::make_mut(self.as_inner_mut())
    }

    #[inline(always)]
    unsafe fn strong_count<T>(&self) -> usize {
        Arc::strong_count(self.as_inner_ref::<T>())
    }

    #[inline(always)]
    unsafe fn clone<T>(&self) -> SharedPointerKindArc {
        SharedPointerKindArc {
            inner: ManuallyDrop::new(Arc::clone(self.as_inner_ref())),
        }
    }

    #[inline(always)]
    unsafe fn drop<T>(&mut self) {
        ptr::drop_in_place::<Arc<T>>(self.as_inner_mut());
    }
}

impl Debug for SharedPointerKindArc {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("SharedPointerKindArc")
    }
}

#[cfg(test)]
mod test;
