use core::fmt::Debug;

/// Trait for [type constructors](https://en.wikipedia.org/wiki/Type_constructor) of
/// reference-counting pointers.
///
/// # Type-parameter invariant
///
/// Each instance of a `SharedPointerKind` implementer is logically associated with a fixed inner
/// type `T`, chosen when the instance is constructed via [`Self::new`], [`Self::from_box`], or
/// [`Self::clone`]. All subsequent calls of the `unsafe` methods on that instance must be called
/// with the same `T`. Callers of the `unsafe` methods are responsible for upholding this
/// invariant.
///
/// # Wrapping in a safe API
///
/// Implementers of this trait do not know the `T` they hold, so their [`Send`]/[`Sync`] impls
/// (if any) are unconditional. For example, [`ArcK`] is always [`Send`] + [`Sync`], but it is
/// only actually safe to send or share across threads when the inner `T` is itself [`Send`] +
/// [`Sync`].
///
/// A safe wrapper around a `SharedPointerKind` implementer must therefore gate its own
/// [`Send`]/[`Sync`] impls on `T: Send + Sync`. [`SharedPointer<T, P>`][SharedPointer] achieves
/// this by including a [`PhantomData<T>`][PhantomData] field, so the compiler only derives
/// [`Send`]/[`Sync`] for `SharedPointer<T, P>` when both `T` and `P` are appropriate.
///
/// # Safety
///
/// `T` may be `!`[`Unpin`], and [`SharedPointer`] may be held in a pinned
/// form ([`Pin`]`<SharedPointer<T, Self>>`).
/// As such, the implementation of this trait must uphold the pinning invariants
/// for `T` while it's held in `Self`. Specifically, this necessitates the
/// following:
///
/// - `&mut T` is only exposed through the trait methods returning `&mut T`.
///
/// - The implementor must not move out the contained `T` unless the semantics
///   of trait methods demands that.
///
/// - [`Self::drop`] drops `T` in place.
///
/// [SharedPointer]: crate::shared_pointer::SharedPointer
/// [`SharedPointer`]: crate::shared_pointer::SharedPointer
/// [`Pin`]: core::pin::Pin
/// [PhantomData]: core::marker::PhantomData
pub unsafe trait SharedPointerKind: Sized + Debug {
    fn new<T>(v: T) -> Self;
    fn from_box<T>(v: Box<T>) -> Self;

    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`. See the
    /// [type-parameter invariant](SharedPointerKind#type-parameter-invariant).
    unsafe fn as_ptr<T>(&self) -> *const T;

    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`. See the
    /// [type-parameter invariant](SharedPointerKind#type-parameter-invariant).
    unsafe fn deref<T>(&self) -> &T;

    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`. See the
    /// [type-parameter invariant](SharedPointerKind#type-parameter-invariant).
    unsafe fn try_unwrap<T>(self) -> Result<T, Self>;

    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`. See the
    /// [type-parameter invariant](SharedPointerKind#type-parameter-invariant).
    unsafe fn get_mut<T>(&mut self) -> Option<&mut T>;

    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`. See the
    /// [type-parameter invariant](SharedPointerKind#type-parameter-invariant).
    unsafe fn make_mut<T: Clone>(&mut self) -> &mut T;

    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`. See the
    /// [type-parameter invariant](SharedPointerKind#type-parameter-invariant).
    unsafe fn strong_count<T>(&self) -> usize;

    /// The returned `Self` inherits the same type-parameter `T` as `self`.
    ///
    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`. See the
    /// [type-parameter invariant](SharedPointerKind#type-parameter-invariant).
    #[must_use]
    unsafe fn clone<T>(&self) -> Self;

    /// # Safety
    ///
    /// `Self` must have been constructed with the same `T`. See the
    /// [type-parameter invariant](SharedPointerKind#type-parameter-invariant).
    ///
    /// This method must be called at most once per instance, when `Self` is being disposed of.
    /// After the call, `Self` must not be used again.
    unsafe fn drop<T>(&mut self);
}

mod arc;
#[cfg(feature = "triomphe")]
mod arct;
mod erased_ptr;
mod rc;

use alloc::boxed::Box;
#[doc(inline)]
pub use arc::ArcK;
#[cfg(feature = "triomphe")]
#[doc(inline)]
pub use arct::ArcTK;
#[doc(inline)]
pub use rc::RcK;
