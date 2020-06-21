/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::shared_pointer::kind::SharedPointerKind;
use alloc::boxed::Box;
use core::borrow::Borrow;
use core::cmp::Ordering;
use core::fmt;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::hash::Hash;
use core::hash::Hasher;
use core::marker::PhantomData;
use core::mem;
use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::ptr;

/// Pointer to shared data with reference-counting.
///
/// The type parameter `P` is a [type constructor](https://en.wikipedia.org/wiki/Type_constructor)
/// of the underlying pointer type, offering a way to abstraction over
/// [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) and
/// [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) smart pointers.
/// This allows you to create data structures where the pointer type is parameterizable, so you can
/// [avoid the overhead of `Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html#thread-safety)
/// when you don’t need to share data across threads.
///
/// # Example
///
/// Declare a data structure with the pointer kind as a type parameter bounded by
/// `SharedPointerKind`:
///
/// ```rust
/// use archery::*;
///
/// struct KeyValuePair<K, V, P: SharedPointerKind> {
///     pub key: SharedPointer<K, P>,
///     pub value: SharedPointer<V, P>,
/// }
///
/// impl<K, V, P: SharedPointerKind> KeyValuePair<K, V, P> {
///     fn new(key: K, value: V) -> KeyValuePair<K, V, P> {
///         KeyValuePair {
///             key: SharedPointer::new(key),
///             value: SharedPointer::new(value),
///         }
///     }
/// }
/// ```
///
/// To use it just plug-in the kind of pointer you want:
///
/// ```rust
/// # use archery::*;
/// #
/// # struct KeyValuePair<K, V, P: SharedPointerKind> {
/// #    pub key: SharedPointer<K, P>,
/// #    pub value: SharedPointer<V, P>,
/// # }
/// #
/// # impl<K, V, P: SharedPointerKind> KeyValuePair<K, V, P> {
/// #     fn new(key: K, value: V) -> KeyValuePair<K, V, P> {
/// #         KeyValuePair {
/// #             key: SharedPointer::new(key),
/// #             value: SharedPointer::new(value),
/// #         }
/// #     }
/// # }
/// #
/// let pair: KeyValuePair<_, _, RcK> =
///     KeyValuePair::new("António Variações", 1944);
///
/// assert_eq!(*pair.value, 1944);
/// ```
pub struct SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    ptr: ManuallyDrop<P>,
    _phantom_t: PhantomData<T>,
}

impl<T, P> SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    #[inline(always)]
    fn new_from_inner(ptr: P) -> SharedPointer<T, P> {
        SharedPointer { ptr: ManuallyDrop::new(ptr), _phantom_t: PhantomData }
    }

    #[inline(always)]
    pub fn new(v: T) -> SharedPointer<T, P> {
        SharedPointer::new_from_inner(P::new::<T>(v))
    }

    #[inline(always)]
    pub fn try_unwrap(mut this: SharedPointer<T, P>) -> Result<T, SharedPointer<T, P>> {
        let ptr: P = unsafe { ManuallyDrop::take(&mut this.ptr) };

        mem::forget(this);

        unsafe { ptr.try_unwrap::<T>() }.map_err(SharedPointer::new_from_inner)
    }

    #[inline(always)]
    pub fn get_mut(this: &mut SharedPointer<T, P>) -> Option<&mut T> {
        unsafe { this.ptr.get_mut::<T>() }
    }

    #[inline(always)]
    pub fn strong_count(this: &Self) -> usize {
        unsafe { this.ptr.strong_count::<T>() }
    }

    #[inline(always)]
    pub fn ptr_eq<PO: SharedPointerKind>(
        this: &SharedPointer<T, P>,
        other: &SharedPointer<T, PO>,
    ) -> bool {
        ptr::eq(this.deref(), other.deref())
    }
}

impl<T, P> SharedPointer<T, P>
where
    T: Clone,
    P: SharedPointerKind,
{
    #[inline(always)]
    pub fn make_mut(this: &mut SharedPointer<T, P>) -> &mut T {
        unsafe { this.ptr.make_mut::<T>() }
    }
}

impl<T, P> Default for SharedPointer<T, P>
where
    T: Default,
    P: SharedPointerKind,
{
    #[inline(always)]
    fn default() -> SharedPointer<T, P> {
        SharedPointer::new(Default::default())
    }
}

impl<T, P> Deref for SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        unsafe { self.ptr.deref().deref() }
    }
}

impl<T, P> Borrow<T> for SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    #[inline(always)]
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T, P> AsRef<T> for SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T, P> Clone for SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    #[inline(always)]
    fn clone(&self) -> SharedPointer<T, P> {
        SharedPointer::new_from_inner(unsafe { self.ptr.deref().clone::<T>() })
    }
}

impl<T, P> Hash for SharedPointer<T, P>
where
    T: Hash,
    P: SharedPointerKind,
{
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

impl<T, P, PO> PartialEq<SharedPointer<T, PO>> for SharedPointer<T, P>
where
    T: PartialEq,
    P: SharedPointerKind,
    PO: SharedPointerKind,
{
    #[inline(always)]
    fn eq(&self, other: &SharedPointer<T, PO>) -> bool {
        self.deref().eq(other.deref())
    }

    #[inline(always)]
    fn ne(&self, other: &SharedPointer<T, PO>) -> bool {
        self.deref().ne(other.deref())
    }
}

impl<T, P> Eq for SharedPointer<T, P>
where
    T: Eq,
    P: SharedPointerKind,
{
}

impl<T, P, PO> PartialOrd<SharedPointer<T, PO>> for SharedPointer<T, P>
where
    T: PartialOrd,
    P: SharedPointerKind,
    PO: SharedPointerKind,
{
    #[inline(always)]
    fn partial_cmp(&self, other: &SharedPointer<T, PO>) -> Option<Ordering> {
        self.deref().partial_cmp(other.deref())
    }

    #[inline(always)]
    fn lt(&self, other: &SharedPointer<T, PO>) -> bool {
        self.deref().lt(other.deref())
    }

    #[inline(always)]
    fn le(&self, other: &SharedPointer<T, PO>) -> bool {
        self.deref().le(other.deref())
    }

    #[inline(always)]
    fn gt(&self, other: &SharedPointer<T, PO>) -> bool {
        self.deref().gt(other.deref())
    }

    #[inline(always)]
    fn ge(&self, other: &SharedPointer<T, PO>) -> bool {
        self.deref().ge(other.deref())
    }
}

impl<T, P> Ord for SharedPointer<T, P>
where
    T: Ord,
    P: SharedPointerKind,
{
    #[inline(always)]
    fn cmp(&self, other: &SharedPointer<T, P>) -> Ordering {
        self.deref().cmp(other.deref())
    }
}

impl<T, P> From<T> for SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    #[inline(always)]
    fn from(other: T) -> SharedPointer<T, P> {
        SharedPointer::new(other)
    }
}

impl<T, P> From<Box<T>> for SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    #[inline(always)]
    fn from(v: Box<T>) -> SharedPointer<T, P> {
        SharedPointer::new_from_inner(P::from_box(v))
    }
}

impl<T, P> Debug for SharedPointer<T, P>
where
    T: Debug,
    P: SharedPointerKind,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Debug::fmt(self.deref(), f)
    }
}

impl<T, P> fmt::Pointer for SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&(&**self as *const T), f)
    }
}

impl<T, P> Display for SharedPointer<T, P>
where
    T: Display,
    P: SharedPointerKind,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        Display::fmt(self.deref(), f)
    }
}

impl<T, P> Drop for SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            self.ptr.drop::<T>();
        }
    }
}

pub mod kind;

#[cfg(test)]
mod test;
