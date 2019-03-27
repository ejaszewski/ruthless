/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::board::Board;
use crate::board::Move;

mod piecesquare;
pub use self::piecesquare::PieceSquareEvaluator;
mod pattern;
pub use self::pattern::PatternEvaluator;

pub trait Evaluator {
    fn get_score(&self, _: &Board) -> i32;
    fn move_order_score(&self, board: &mut Board, mv: Move) -> i32 {
        let undo = board.make_move(mv);
        let score = -self.get_score(board);
        board.undo_move(undo, mv);
        
        score
    }
}
