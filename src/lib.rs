#![no_std]
#![cfg_attr(feature = "fatal-warnings", deny(warnings))]
// Note: If you change this remember to update `README.md`.  To do so run `cargo rdme`.
//! `archery` is a rust library that offers a way to abstraction over
//! [`Rc`](::alloc::rc::Rc) and
//! [`Arc`](::alloc::sync::Arc) smart pointers.
//! This allows you to create data structures where the pointer type is parameterizable, so you can
//! [avoid the overhead of `Arc`](::alloc::sync::Arc#thread-safety)
//! when you don’t need to share data across threads.
//!
//! In languages that supports
//! [higher-kinded polymorphism](https://en.wikipedia.org/wiki/Type_class#Higher-kinded_polymorphism)
//! this would be simple to achieve without any library, but
//! [rust does not support that yet](https://github.com/rust-lang/rfcs/issues/324).
//! To mimic higher-kinded polymorphism `archery` implements the approach suggested by
//! Joshua Liebow-Feeser in
//! “[Rust has higher kinded types already… sort of](https://joshlf.com/post/2018/10/18/rust-higher-kinded-types-already/)”.
//! While [other approaches](#alternative-approaches) exist, they seem to always offer poor
//! ergonomics for the user.
//!
//! # Setup
//!
//! To use `archery` add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! archery = "<version>"
//! ```
//!
//! # Using `archery`
//!
//! `archery` defines a [`SharedPointer`](crate::shared_pointer::SharedPointer)
//! that receives the [kind of pointer](crate::shared_pointer::kind::SharedPointerKind)
//! as a type parameter.  This gives you a convenient and ergonomic way to abstract the pointer
//! type away.
//!
//! ## Example
//!
//! Declare a data structure with the pointer kind as a type parameter bounded by
//! [`SharedPointerKind`](crate::shared_pointer::kind::SharedPointerKind):
//!
//! ```rust
//! use archery::*;
//!
//! struct KeyValuePair<K, V, P: SharedPointerKind> {
//!     pub key: SharedPointer<K, P>,
//!     pub value: SharedPointer<V, P>,
//! }
//!
//! impl<K, V, P: SharedPointerKind> KeyValuePair<K, V, P> {
//!     fn new(key: K, value: V) -> KeyValuePair<K, V, P> {
//!         KeyValuePair {
//!             key: SharedPointer::new(key),
//!             value: SharedPointer::new(value),
//!         }
//!     }
//! }
//! ```
//!
//! To use it just plug-in the kind of pointer you want:
//!
//! ```rust
//! # use archery::*;
//! #
//! # struct KeyValuePair<K, V, P: SharedPointerKind> {
//! #    pub key: SharedPointer<K, P>,
//! #    pub value: SharedPointer<V, P>,
//! # }
//! #
//! # impl<K, V, P: SharedPointerKind> KeyValuePair<K, V, P> {
//! #     fn new(key: K, value: V) -> KeyValuePair<K, V, P> {
//! #         KeyValuePair {
//! #             key: SharedPointer::new(key),
//! #             value: SharedPointer::new(value),
//! #         }
//! #     }
//! # }
//! #
//! let pair: KeyValuePair<_, _, RcK> =
//!     KeyValuePair::new("António Variações", 1944);
//!
//! assert_eq!(*pair.value, 1944);
//! ```
//!
//! ## `triomphe::Arc`
//!
//! You can also use [`triomphe::Arc`](https://docs.rs/triomphe/latest/triomphe/struct.Arc.html)
//! as the backing implementation of a [`SharedPointer`](crate::shared_pointer::SharedPointer).
//! This is generally faster than [`std::sync::Arc`](::alloc::sync::Arc).
//! Read [`triomphe`’s crate documentation](https://docs.rs/triomphe/latest/triomphe/) to learn more
//! about it.
//!
//! To use it you need to enable the `triomphe` feature in `archery`. Use `ArcTK` as the pointer
//! kind in [`SharedPointer`](crate::shared_pointer::SharedPointer).
//!
//! ## Serialization
//!
//! We support serialization through [serde](https://crates.io/crates/serde).  To use it
//! enable the `serde` feature.  To do so change the archery dependency in your `Cargo.toml` to
//!
//! ```toml
//! [dependencies]
//! archery = { version = "<version>", features = ["serde"] }
//! ```
//! # Limitations
//!
//! Currently it is not possible to have unsized types inside a
//! [`SharedPointer`](crate::shared_pointer::SharedPointer).  As a workaround you can put the
//! unsized type inside a [`Box`](::alloc::boxed::Box).
//!
//! # Alternative approaches
//!
//! An alternative to the approach taken by `archery` is to use traits with associated types to encode
//! type-level functions.  This has been suggested
//! [multiple](https://github.com/orium/rpds/issues/7#issuecomment-362635901)
//! [times](https://joshlf.com/post/2018/10/18/rust-higher-kinded-types-already/#comment-4160863400),
//! but offers ugly ergonomics (see
//! [here](https://github.com/Marwes/rpds/blob/e482d5abbaa6c876d7c624e497affe7299bbeece/src/sequence/vector/mod.rs#L153)
//! and [here](https://github.com/Marwes/rpds/blob/e482d5abbaa6c876d7c624e497affe7299bbeece/src/sequence/vector/mod.rs#L249)).

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod shared_pointer;

pub use shared_pointer::SharedPointer;

pub use shared_pointer::kind::SharedPointerKind;

#[doc(no_inline)]
pub use shared_pointer::kind::ArcK;
#[cfg(feature = "triomphe")]
#[doc(no_inline)]
pub use shared_pointer::kind::ArcTK;
#[doc(no_inline)]
pub use shared_pointer::kind::RcK;
