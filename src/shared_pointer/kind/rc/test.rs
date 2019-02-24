/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use super::*;

#[test]
fn test_deref() {
    let mut ptr_42 = SharedPointerKindRc::new::<i32>(42);
    let mut ptr_box_dyn_hello = SharedPointerKindRc::new::<Box<dyn ToString>>(Box::new("hello"));

    unsafe {
        assert_eq!(ptr_42.deref::<i32>(), &42);
        assert_eq!(
            ptr_box_dyn_hello.deref::<Box<dyn ToString>>().to_string(),
            "hello"
        );

        ptr_42.drop::<i32>();
        ptr_box_dyn_hello.drop::<Box<dyn ToString>>();
    }
}
