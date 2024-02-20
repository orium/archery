# Release notes

## 1.2.0

* Added support for serde. This gated behind the `serde` feature.

## 1.1.0

* Added support for `triomphe::Arc` as a pointer kind. This gated behind the `triomphe` feature.

## 1.0.0

* No changes.  It’s just time to commit to a stable release :).

## 0.5.0

* Fix `Send`/`Sync` unsoundness in `SharedPointer`.  See issue [#18](https://github.com/orium/archery/issues/18).
* Added `SharedPointer::pin()`.
* Added `SharedPointer::as_ptr()`.
* Updated dependencies.
* Updated to 2021 edition.

## 0.4.0

* Added support for `no_std`.

## 0.3.0

* Renamed `SharedPointerKindRc` and `SharedPointerKindArc` to `RcK` and `ArcK`, respectively.

## 0.2.1

* Minor fix in README.

## 0.2.0

* Added some functionality to `SharedPointer` that you would expect from `Rc`/`Arc`.
  * Functions:
    * `SharedPointer::try_unwrap()`.
    * `SharedPointer::get_mut()`.
    * `SharedPointer::strong_count()`.
    * `SharedPointer::ptr_eq()`.
  * Traits:
    * `Default`.
    * `From<T>`.
    * `From<Box<T>>`.
    * `std::fmt::Pointer`.

## 0.1.0

* Initial release with `SharedPointer`, `SharedPointerKind`, `SharedPointerKindRc`, and `SharedPointerKindArc`.
  * Functionality exposed from the underlying pointers: `deref()`, `make_mut()`, `clone()`.
