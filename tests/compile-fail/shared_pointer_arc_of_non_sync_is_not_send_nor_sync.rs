use archery::*;
use static_assertions::assert_impl_all;
use std::cell::Cell;

assert_impl_all!(SharedPointer<Cell<()>, ArcK>: Send);
//~^ ERROR cannot be shared between threads safely

assert_impl_all!(SharedPointer<Cell<()>, ArcK>: Sync);
//~^ ERROR cannot be shared between threads safely

fn main() {}
