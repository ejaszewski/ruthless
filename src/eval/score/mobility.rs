use board;
use eval::{properties, score};

pub fn get_mobility(board: &mut board::Board) -> f32 {
    return board.move_count() as f32;
}

pub fn get_mobility_weighted(board: &mut board::Board, heuristic: &properties::Heuristic) -> f32 {
    let mut mobility_score = 0.0;
    let mut index = 0;
    let dark_moves = board.get_dark_moves();
    let light_moves = board.get_light_moves();
    for &mask in score::EVAL_MASKS.iter() {
        mobility_score += (
            score::disk_count(dark_moves, mask) -
            score::disk_count(light_moves, mask)
        ) * heuristic.mobility_values[index];
        index += 1;
    }
    return mobility_score;
}
