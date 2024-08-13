/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::shared_pointer::kind::SharedPointerKind;
use alloc::boxed::Box;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Formatter;
use core::mem;
use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr;
use triomphe::Arc;

type UntypedArc = Arc<()>;

/// [Type constructors](https://en.wikipedia.org/wiki/Type_constructor) for
/// [`triomphe::Arc`](triomphe::Arc) pointers.
pub struct ArcTK {
    /// We use [`ManuallyDrop`] here, so that we can drop it explicitly as
    /// [`Arc<T>`](triomphe::Arc).  Not sure if it can be dropped as [`UntypedArc`], but it
    /// seems to be playing with fire (even more than we already are).
    inner: ManuallyDrop<UntypedArc>,
}

impl ArcTK {
    #[inline(always)]
    fn new_from_inner<T>(arc: Arc<T>) -> ArcTK {
        ArcTK { inner: ManuallyDrop::new(unsafe { mem::transmute::<Arc<T>, UntypedArc>(arc) }) }
    }

    #[inline(always)]
    unsafe fn take_inner<T>(self) -> Arc<T> {
        let arc: UntypedArc = ManuallyDrop::into_inner(self.inner);

        mem::transmute(arc)
    }

    #[inline(always)]
    unsafe fn as_inner_ref<T>(&self) -> &Arc<T> {
        let arc_t: *const Arc<T> = (self.inner.deref() as *const UntypedArc).cast::<Arc<T>>();

        // Static check to make sure we are not messing up the sizes.
        // This could happen if we allowed for `T` to be unsized, because it would need to be
        // represented as a wide pointer inside `Arc`.
        // TODO Use static_assertion when https://github.com/nvzqz/static-assertions-rs/issues/21
        //      gets fixed
        let _ = mem::transmute::<UntypedArc, Arc<T>>;

        &*arc_t
    }

    #[inline(always)]
    unsafe fn as_inner_mut<T>(&mut self) -> &mut Arc<T> {
        let arc_t: *mut Arc<T> = (self.inner.deref_mut() as *mut UntypedArc).cast::<Arc<T>>();

        &mut *arc_t
    }
}

unsafe impl SharedPointerKind for ArcTK {
    #[inline(always)]
    fn new<T>(v: T) -> ArcTK {
        ArcTK::new_from_inner(Arc::new(v))
    }

    #[inline(always)]
    fn from_box<T>(v: Box<T>) -> ArcTK {
        ArcTK::new_from_inner::<T>(Arc::from(v))
    }

    #[inline(always)]
    unsafe fn as_ptr<T>(&self) -> *const T {
        Arc::as_ptr(self.as_inner_ref())
    }

    #[inline(always)]
    unsafe fn deref<T>(&self) -> &T {
        self.as_inner_ref::<T>().as_ref()
    }

    #[inline(always)]
    unsafe fn try_unwrap<T>(self) -> Result<T, ArcTK> {
        Arc::try_unwrap(self.take_inner()).map_err(ArcTK::new_from_inner)
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
        Arc::count(self.as_inner_ref::<T>())
    }

    #[inline(always)]
    unsafe fn clone<T>(&self) -> ArcTK {
        ArcTK { inner: ManuallyDrop::new(Arc::clone(self.as_inner_ref())) }
    }

    #[inline(always)]
    unsafe fn drop<T>(&mut self) {
        ptr::drop_in_place::<Arc<T>>(self.as_inner_mut());
    }
}

impl Debug for ArcTK {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("ArcTK")
    }
}

#[cfg(test)]
mod test;
