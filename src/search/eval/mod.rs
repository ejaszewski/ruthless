/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::board::Board;

mod piecesquare;
pub use self::piecesquare::PieceSquareEvaluator;

pub trait Evaluator {
    fn get_score(&self, _: &Board) -> i32;
}
