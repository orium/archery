/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::shared_pointer::kind::SharedPointerKind;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::ops::Deref;

pub struct SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    ptr: P,
    _phantom_t: PhantomData<T>,
}

impl<T, P> SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    #[inline(always)]
    pub fn new(v: T) -> SharedPointer<T, P> {
        SharedPointer {
            ptr: P::new::<T>(v),
            _phantom_t: PhantomData,
        }
    }
}

impl<T, P> SharedPointer<T, P>
where
    P: SharedPointerKind,
    T: Clone,
{
    #[inline(always)]
    pub fn make_mut(this: &mut SharedPointer<T, P>) -> &mut T {
        unsafe { this.ptr.make_mut::<T>() }
    }
}

impl<T, P> Deref for SharedPointer<T, P>
where
    P: SharedPointerKind,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        unsafe { self.ptr.deref() }
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
        SharedPointer {
            ptr: unsafe { self.ptr.clone::<T>() },
            _phantom_t: PhantomData,
        }
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

impl<T, P: SharedPointerKind> Display for SharedPointer<T, P>
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
