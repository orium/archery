[![Build Status](https://github.com/orium/archery/workflows/CI/badge.svg)](https://github.com/orium/archery/actions?query=workflow%3ACI)
[![Code Coverage](https://codecov.io/gh/orium/archery/branch/master/graph/badge.svg)](https://codecov.io/gh/orium/archery)
[![Dependency status](https://deps.rs/repo/github/orium/archery/status.svg)](https://deps.rs/repo/github/orium/archery)
[![crates.io](https://img.shields.io/crates/v/archery.svg)](https://crates.io/crates/archery)
[![Downloads](https://img.shields.io/crates/d/archery.svg)](https://crates.io/crates/archery)
[![Github stars](https://img.shields.io/github/stars/orium/archery.svg?logo=github)](https://github.com/orium/archery/stargazers)
[![Documentation](https://docs.rs/archery/badge.svg)](https://docs.rs/archery/)
[![License](https://img.shields.io/crates/l/archery.svg)](./LICENSE.md)
<img src="https://raw.githubusercontent.com/orium/archery/master/images/archery.svg?sanitize=true" width="240" align="right">

<!-- cargo-rdme start -->

# Archery

Archery is a rust library that offers a way to abstraction over
[`Rc`](https://doc.rust-lang.org/stable/alloc/rc/struct.Rc.html) and
[`Arc`](https://doc.rust-lang.org/stable/alloc/sync/struct.Arc.html) smart pointers.
This allows you to create data structures where the pointer type is parameterizable, so you can
[avoid the overhead of `Arc`](https://doc.rust-lang.org/stable/alloc/sync/struct.Arc.html#thread-safety)
when you don’t need to share data across threads.

In languages that supports
[higher-kinded polymorphism](https://en.wikipedia.org/wiki/Type_class#Higher-kinded_polymorphism)
this would be simple to achieve without any library, but
[rust does not support that yet](https://github.com/rust-lang/rfcs/issues/324).
To mimic higher-kinded polymorphism Archery implements the approach suggested by
Joshua Liebow-Feeser in
“[Rust has higher kinded types already… sort of](https://joshlf.com/post/2018/10/18/rust-higher-kinded-types-already/)”.
While [other approaches](#alternative-approaches) exist, they seem to always offer poor
ergonomics for the user.

## Setup

To use Archery add the following to your `Cargo.toml`:

```toml
[dependencies]
archery = "<version>"
```

## Using Archery

Archery defines a [`SharedPointer`](https://docs.rs/archery/latest/archery/shared_pointer/struct.SharedPointer.html)
that receives the [kind of pointer](https://docs.rs/archery/latest/archery/shared_pointer/kind/trait.SharedPointerKind.html)
as a type parameter.  This gives you a convenient and ergonomic way to abstract the pointer
type away.

### Example

Declare a data structure with the pointer kind as a type parameter bounded by
[`SharedPointerKind`](https://docs.rs/archery/latest/archery/shared_pointer/kind/trait.SharedPointerKind.html):

```rust
use archery::*;

struct KeyValuePair<K, V, P: SharedPointerKind> {
    pub key: SharedPointer<K, P>,
    pub value: SharedPointer<V, P>,
}

impl<K, V, P: SharedPointerKind> KeyValuePair<K, V, P> {
    fn new(key: K, value: V) -> KeyValuePair<K, V, P> {
        KeyValuePair {
            key: SharedPointer::new(key),
            value: SharedPointer::new(value),
        }
    }
}
```

To use it just plug-in the kind of pointer you want:

```rust
let pair: KeyValuePair<_, _, RcK> =
    KeyValuePair::new("António Variações", 1944);

assert_eq!(*pair.value, 1944);
```

## Limitations

Currently it is not possible to have unsized types inside a
[`SharedPointer`](https://docs.rs/archery/latest/archery/shared_pointer/struct.SharedPointer.html).  As a workaround you can put the
unsized type inside a [`Box`](https://doc.rust-lang.org/stable/alloc/boxed/struct.Box.html).

# Alternative approaches

An alternative to the approach taken by Archery is to use traits with associated types to encode
type-level functions.  This has been suggested
[multiple](https://github.com/orium/rpds/issues/7#issuecomment-362635901)
[times](https://joshlf.com/post/2018/10/18/rust-higher-kinded-types-already/#comment-4160863400),
but offers ugly ergonomics (see
[here](https://github.com/Marwes/rpds/blob/e482d5abbaa6c876d7c624e497affe7299bbeece/src/sequence/vector/mod.rs#L153)
and [here](https://github.com/Marwes/rpds/blob/e482d5abbaa6c876d7c624e497affe7299bbeece/src/sequence/vector/mod.rs#L249)).

<!-- cargo-rdme end -->
