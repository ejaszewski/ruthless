/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use ::board::Board;

const PIECE_SQUARE_MASKS: [u64; 10] = [
    0x8100000000000081,
    0x4281000000008142,
    0x2400810000810024,
    0x1800008181000018,
    0x0042000000004200,
    0x0024420000422400,
    0x0018004242001800,
    0x0000240000240000,
    0x0000182424180000,
    0x0000001818000000
];

pub struct PieceSquareEvaluator {
    square_table: [i32; 10]
}

impl PieceSquareEvaluator {
    pub fn new() -> PieceSquareEvaluator {
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
    fn get_score(&self, board: &mut Board) -> i32 {
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
    use ::board::Board;
    use ::search::eval::Evaluator;
    use super::PieceSquareEvaluator;

    #[test]
    fn test_piece_square_evaluator() {
        let eval_1 = PieceSquareEvaluator::new();
        let eval_2 = PieceSquareEvaluator::from([1; 10]);

        let mut board = Board::from_pos(0xFFFFFFFF00000000, 0x00000000FFFFFFFF, true);
        assert_eq!(eval_1.get_score(&mut board), 0);
        assert_eq!(eval_2.get_score(&mut board), 0);
    }
}
