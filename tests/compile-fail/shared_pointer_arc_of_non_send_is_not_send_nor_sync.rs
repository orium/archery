extern crate archery;
extern crate static_assertions;

use archery::*;
use static_assertions::assert_impl_all;
use std::sync::MutexGuard;

assert_impl_all!(SharedPointer<MutexGuard<'static, ()>, ArcK>: Send);
//~^ ERROR cannot be sent between threads safely

assert_impl_all!(SharedPointer<MutexGuard<'static, ()>, ArcK>: Sync);
//~^ ERROR cannot be sent between threads safely

fn main() {}
