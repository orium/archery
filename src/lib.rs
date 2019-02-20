/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::correctness))]
#![cfg_attr(feature = "cargo-clippy", warn(clippy::pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::match_bool))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::if_not_else))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::stutter))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::similar_names))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::use_self))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::single_match_else))]
// Note: If you change this remember to update `README.md`.  To do so run `./tools/update-readme.sh`.
//! # Archery
//!
//! WIP! README

pub fn test() -> u32 {
    println!("hello");
    println!("from");
    println!("archery");

    42
}

#[cfg(test)]
mod test;
