use archery::*;
use std::rc::Rc;
use static_assertions::assert_impl_all;

assert_impl_all!(SharedPointer<Rc<i32>, ArcK>: Send);
//~^ ERROR cannot be sent between threads safely
//~^^ ERROR cannot be shared between threads safely

assert_impl_all!(SharedPointer<Rc<i32>, ArcK>: Sync);
//~^ ERROR cannot be sent between threads safely
//~^^ ERROR cannot be shared between threads safely

fn main() {}
