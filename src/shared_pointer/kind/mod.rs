/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fmt::Debug;

/// Trait for [type constructors](https://en.wikipedia.org/wiki/Type_constructor) of
/// reference-counting pointers.
pub trait SharedPointerKind: Sized + Debug {
    fn new<T>(v: T) -> Self;
    fn from_box<T>(v: Box<T>) -> Self;
    unsafe fn deref<T>(&self) -> &T;
    unsafe fn try_unwrap<T>(self) -> Result<T, Self>;
    unsafe fn get_mut<T>(&mut self) -> Option<&mut T>;
    unsafe fn make_mut<T: Clone>(&mut self) -> &mut T;
    unsafe fn strong_count<T>(&self) -> usize;
    unsafe fn clone<T>(&self) -> Self;
    unsafe fn drop<T>(&mut self);
}

mod arc;
mod rc;

#[doc(inline)]
pub use arc::SharedPointerKindArc;
#[doc(inline)]
pub use rc::SharedPointerKindRc;
