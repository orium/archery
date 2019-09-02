extern crate archery;
extern crate static_assertions;

use std::rc::Rc;
use static_assertions::assert_impl_all;
use archery::*;

fn main() {
    assert_impl_all!(
        SharedPointer<Rc<i32>, ArcK>,
        Send
    );
    //~^^^^ ERROR cannot be sent between threads safely

    assert_impl_all!(
        SharedPointer<Rc<i32>, ArcK>,
        Sync
    );
    //~^^^^ ERROR cannot be shared between threads safely
}
