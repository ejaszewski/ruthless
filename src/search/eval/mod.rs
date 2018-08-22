use board::Board;

mod piecesquare;
pub use self::piecesquare::PieceSquareEvaluator;

pub trait Evaluator {
    fn get_score(&self, &mut Board) -> i32;
}
