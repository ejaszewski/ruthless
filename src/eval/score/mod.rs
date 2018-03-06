use board;
use eval::{properties, negamax};
use std::collections::HashMap;

fn disk_count(x: u64, mask: u64) -> f32 {
    return (x & mask).count_ones() as f32;
}

pub fn get_score_heuristic(board: &mut board::Board, heuristic: &properties::Heuristic) -> f32 {
    let mut material_score = 0.0;
    let mut index = 0;
    for &mask in properties::MATERIAL_MASKS.iter() {
        material_score +=
            (disk_count(board.dark_disks, mask) - disk_count(board.light_disks, mask)) * heuristic.square_values[index];
        index += 1;
    }
    let mobility_score = board.move_count() as f32;
    let score =
        material_score * heuristic.material_weight + mobility_score * heuristic.mobility_weight;
    if board.dark_move {
        score + heuristic.bias
    } else {
        -score + heuristic.bias
    }
}

pub fn get_parity(board: &board::Board) -> i8 {
    let modifier = if board.dark_move { 1 } else { -1 };
    (board.dark_disks.count_ones() as i8 - board.light_disks.count_ones() as i8) * modifier
}

pub fn get_score_endgame_solve(board: &board::Board) -> i8 {
    get_parity(board).signum()
}

pub fn get_move_map(board: &mut board::Board, moves: &mut Vec<Option<u8>>,
                    heuristic: &properties::Heuristic, depth: u8) -> HashMap<Option<u8>, f32> {
    let mut move_map: HashMap<Option<u8>, f32> = HashMap::new();

    for m in moves {
        let undo = board.make_move(*m);
        let (mut score, _leaves) = negamax::negamax(board, heuristic, -10000., 10000., depth);
        board.undo_move(undo, *m);

        score = -score;

        move_map.insert(*m, score);
    }

    move_map
}

pub fn get_fastest_first_map(board: &mut board::Board, moves: &mut Vec<Option<u8>>) -> HashMap<Option<u8>, u32> {
    let mut move_map: HashMap<Option<u8>, u32> = HashMap::new();

    for m in moves {
        let undo = board.make_move(*m);
        let score = board.move_count();
        board.undo_move(undo, *m);
        move_map.insert(*m, score);
    }

    move_map
}
