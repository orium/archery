extern crate archery;
extern crate static_assertions;

use std::rc::Rc;
use static_assertions::assert_impl;
use archery::*;

fn main() {
    assert_impl!(
        SharedPointer<Rc<i32>, SharedPointerKindArc>,
        Send
    );
    //~^^^^ ERROR cannot be sent between threads safely

    assert_impl!(
        SharedPointer<Rc<i32>, SharedPointerKindArc>,
        Sync
    );
    //~^^^^ ERROR cannot be shared between threads safely
}
