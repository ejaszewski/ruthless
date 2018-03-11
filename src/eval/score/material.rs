use board;
use eval::{properties, score};

pub fn get_material(board: &board::Board) -> i8 {
    if board.dark_move {
        (board.dark_disks.count_ones() as i8 - board.light_disks.count_ones() as i8)
    } else {
        (board.light_disks.count_ones() as i8 - board.dark_disks.count_ones() as i8)
    }
}

pub fn get_material_weighted(board: &board::Board, heuristic: &properties::Heuristic) -> f32 {
    let mut material_score = 0.0;
    let mut index = 0;
    for &mask in score::EVAL_MASKS.iter() {
        material_score += (
            score::disk_count(board.dark_disks, mask) -
            score::disk_count(board.light_disks, mask)
        ) * heuristic.square_values[index];
        index += 1;
    }
    return material_score;
}
