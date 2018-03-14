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
    let stable_dark = score::stability::get_stable_squares(board.dark_disks);
    let stable_light = score::stability::get_stable_squares(board.dark_disks);
    let unstable_dark = board.dark_disks & !stable_dark;
    let unstable_light = board.light_disks & !stable_light;
    for &mask in score::EVAL_MASKS.iter() {
        material_score += score::disk_count(stable_dark, mask) * heuristic.stable_material_values[index];
        material_score -= score::disk_count(stable_light, mask) * heuristic.stable_material_values[index];

        material_score += score::disk_count(unstable_dark, mask) * heuristic.unstable_material_values[index];
        material_score -= score::disk_count(unstable_light, mask) * heuristic.unstable_material_values[index];

        index += 1;
    }
    return material_score;
}
