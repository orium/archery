/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fmt::Debug;

pub trait SharedPointerKind: Debug {
    fn new<T>(v: T) -> Self;
    unsafe fn deref<T>(&self) -> &T;
    unsafe fn make_mut<T>(&mut self) -> &mut T
    where
        T: Clone;
    unsafe fn clone<T>(&self) -> Self;
    unsafe fn drop<T>(&mut self);
}

mod arc;
mod rc;

#[doc(inline)]
pub use arc::SharedPointerKindArc;
#[doc(inline)]
pub use rc::SharedPointerKindRc;
