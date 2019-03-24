/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::board::Board;

const PIECE_SQUARE_MASKS: [u64; 10] = [
    0x81_00_00_00_00_00_00_81,
    0x42_81_00_00_00_00_81_42,
    0x24_00_81_00_00_81_00_24,
    0x18_00_00_81_81_00_00_18,
    0x00_42_00_00_00_00_42_00,
    0x00_24_42_00_00_42_24_00,
    0x00_18_00_42_42_00_18_00,
    0x00_00_24_00_00_24_00_00,
    0x00_00_18_24_24_18_00_00,
    0x00_00_00_18_18_00_00_00
];

#[derive(Default)]
pub struct PieceSquareEvaluator {
    square_table: [i32; 10]
}

impl PieceSquareEvaluator {
    pub fn new() -> Self {
        PieceSquareEvaluator {
            square_table: [64, -30, 10, 5, -40, 2, 2, 5, 1, 1]
        }
    }

    pub fn from(square_table: [i32; 10]) -> PieceSquareEvaluator {
        PieceSquareEvaluator {
            square_table
        }
    }
}

impl super::Evaluator for PieceSquareEvaluator {
    fn get_score(&self, board: &Board) -> i32 {
        let mut black_score = 0;
        let mut white_score = 0;

        for (index, mask) in PIECE_SQUARE_MASKS.iter().enumerate() {
            black_score += (board.black_disks & mask).count_ones() as i32 * self.square_table[index];
            white_score += (board.white_disks & mask).count_ones() as i32 * self.square_table[index];
        }

        if board.black_move {
            black_score - white_score
        } else {
            white_score - black_score
        }
    }
}

#[cfg(test)]
mod test {
    use crate::board::Board;
    use crate::search::eval::Evaluator;
    use super::PieceSquareEvaluator;

    #[test]
    fn test_piece_square_evaluator() {
        let eval_1 = PieceSquareEvaluator::new();
        let eval_2 = PieceSquareEvaluator::from([1; 10]);

        let mut board = Board::from_pos(0xFF_FF_FF_FF_00_00_00_00, 0x00_00_00_00_FF_FF_FF_FF, true);
        assert_eq!(eval_1.get_score(&board), 0);
        assert_eq!(eval_2.get_score(&board), 0);
    }
}
