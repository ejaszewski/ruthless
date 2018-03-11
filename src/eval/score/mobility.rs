use board;
use eval::properties;

pub fn get_mobility(board: &mut board::Board) -> f32 {
    return board.move_count() as f32;
}

pub fn get_mobility_weighted(board: &mut board::Board, heuristic: &properties::Heuristic) -> f32 {
    return board.move_count() as f32;
}
