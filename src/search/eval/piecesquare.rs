use ::board::Board;

const PIECE_SQUARE_MASKS: [u64; 10] = [
    0x8100000000000081,
    0x4200000000000042,
    0x2400000000000024,
    0x1800000000000018,
    0x0042000000004200,
    0x0024000000002400,
    0x0018000000001800,
    0x0000240000240000,
    0x0000180000180000,
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
}

impl super::Evaluator for PieceSquareEvaluator {
    fn get_score(&self, board: &mut Board) -> i32 {
        let mut score = 0;
        for (index, mask) in PIECE_SQUARE_MASKS.iter().enumerate() {
            score += (board.black_disks & mask).count_ones() as i32 * self.square_table[index];
            score -= (board.white_disks & mask).count_ones() as i32 * self.square_table[index];
        }
        score
    }
}
