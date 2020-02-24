/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub mod eval;

pub mod endgame;
pub mod negamax;
pub mod bns;
pub mod nm_new;
pub mod iterative;
pub mod hashtable;

#[cfg(test)]
mod ffo_test;

pub struct SearchData {
    pub nodes: u64,
    pub time: u32,
    pub depth: u8
}
