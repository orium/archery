/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use super::*;
use kind::SharedPointerKindArc;
use static_assertions::assert_impl;

assert_impl!(
    shared_pointer_arc_impls_send_sync;
    SharedPointer<i32, SharedPointerKindArc>,
    Send, Sync
);
