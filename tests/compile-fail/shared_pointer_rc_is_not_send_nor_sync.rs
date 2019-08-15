extern crate archery;
extern crate static_assertions;

use static_assertions::assert_impl_all;
use archery::*;

fn main() {
    assert_impl_all!(i32, Send, Sync);

    assert_impl_all!(
        SharedPointer<i32, SharedPointerKindRc>,
        Send
    );
    //~^^^^ ERROR cannot be sent between threads safely

    assert_impl_all!(
        SharedPointer<i32, SharedPointerKindRc>,
        Sync
    );
    //~^^^^ ERROR cannot be shared between threads safely
}
