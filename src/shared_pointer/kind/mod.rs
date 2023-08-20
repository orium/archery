/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use core::fmt::Debug;

/// Trait for [type constructors](https://en.wikipedia.org/wiki/Type_constructor) of
/// reference-counting pointers.
///
/// # Safety
///
/// `T` may be `!`[`Unpin`], and [`SharedPointer`][1] may be held in a pinned
/// form ([`Pin`][2]`<SharedPointer<T, Self>>`).
/// As such, the implementation of this trait must uphold the pinning invariants
/// for `T` while it's held in `Self`. Specifically, this necessitates the
/// following:
///
/// - `&mut T` is only exposed through the trait methods returning `&mut T`.
///
/// - The implementor must not move out the contained `T` unless the semantics
/// of trait methods demands that.
///
/// - [`Self::drop`] drops `T` in place.
///
/// [1]: crate::shared_pointer::SharedPointer
/// [2]: core::pin::Pin
//
// There are two conditions for types implementing this trait to be used in a safe way:
//
// 1. Always use the correct type `T`.
// 2. Make sure that you use it wrapped in something that derives the correct auto-traits taking
//    into account the type of `T`.
//
// To elaborate on point 2: a `ArcK` will always implement `Send + Sync`, but that
// is only safe if the actually type that `ArcK` holds is in fact `Send + Sync`.
// This means that a safe wrapper around this type must make sure it does not implement
// `Send + Sync` unless `T: Send + Sync`.  This is holds true for `SharedPointer` since it has a
// phantom field with `T`, thus the compiler will only make `SharedPointer<T>` implement
// `Send + Sync` if `T: Send + Sync`.
pub unsafe trait SharedPointerKind: Sized + Debug {
    fn new<T>(v: T) -> Self;
    fn from_box<T>(v: Box<T>) -> Self;
    unsafe fn as_ptr<T>(&self) -> *const T;
    unsafe fn deref<T>(&self) -> &T;
    unsafe fn try_unwrap<T>(self) -> Result<T, Self>;
    unsafe fn get_mut<T>(&mut self) -> Option<&mut T>;
    unsafe fn make_mut<T: Clone>(&mut self) -> &mut T;
    unsafe fn strong_count<T>(&self) -> usize;
    #[must_use]
    unsafe fn clone<T>(&self) -> Self;
    unsafe fn drop<T>(&mut self);
}

mod arc;
#[cfg(feature = "triomphe")]
mod arct;
mod rc;

use alloc::boxed::Box;
#[doc(inline)]
pub use arc::ArcK;
#[cfg(feature = "triomphe")]
#[doc(inline)]
pub use arct::ArcTK;
#[doc(inline)]
pub use rc::RcK;
