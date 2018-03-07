use board;
use eval::{properties, negamax};
use std::collections::HashMap;
use std::f32;

fn disk_count(x: u64, mask: u64) -> f32 {
    return (x & mask).count_ones() as f32;
}

pub fn get_score_heuristic(board: &mut board::Board, heuristic: &properties::Heuristic) -> f32 {
    let score;
    if board.is_game_over() {
        if board.dark_disks.count_ones() > board.light_disks.count_ones() {
            score = f32::INFINITY;
        } else {
            score = f32::NEG_INFINITY;
        }
    } else {
        let mut material_score = 0.0;
        let mut index = 0;
        for &mask in properties::MATERIAL_MASKS.iter() {
            material_score +=
            (disk_count(board.dark_disks, mask) - disk_count(board.light_disks, mask)) * heuristic.square_values[index];
            index += 1;
        }
        let mobility_score = board.move_count() as f32;
        score = material_score * heuristic.material_weight * (1.0 + mobility_score * heuristic.mobility_weight).log10();
    }

    if board.dark_move {
        score + heuristic.bias
    } else {
        -score + heuristic.bias
    }
}

pub fn get_parity(board: &board::Board) -> i8 {
    if board.dark_move {
        (board.dark_disks.count_ones() as i8 - board.light_disks.count_ones() as i8)
    } else {
        (board.light_disks.count_ones() as i8 - board.dark_disks.count_ones() as i8)
    }
}

pub fn get_endgame_score(board: &board::Board) -> i8 {
    if board.dark_move {
        (board.dark_disks.count_ones() as i8 - board.light_disks.count_ones() as i8).signum()
    } else {
        (board.light_disks.count_ones() as i8 - board.dark_disks.count_ones() as i8).signum()
    }
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

pub fn get_fastest_first_arr(board: &mut board::Board, moves: &mut Vec<Option<u8>>) -> [u32; 64] {
    let mut ret = [0; 64];

    for m in moves {
        let undo = board.make_move(*m);
        let score = board.move_count();
        board.undo_move(undo, *m);

        match *m {
            Some(sq) => ret[sq as usize] = score,
            _ => {}
        }
    }

    ret
}
