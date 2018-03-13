use board;
use eval::{properties, negamax};
use std::collections::HashMap;
use std::f32;

mod material;
mod mobility;
mod stability;

/*
Layout of these constants:
    A B C D D C B A
    B E F G G F E B
    C F H I I H F C
    D G I J J I G D
    D G I J J I G D
    C F H I I H F C
    B E F G G F E B
    A B C D D C B A
*/
pub const EVAL_MASKS: [u64; 10] = [
    0x81_00_00_00_00_00_00_81, // A
    0x42_81_00_00_00_00_81_42, // B
    0x24_00_81_00_00_81_00_24, // C
    0x18_00_00_81_81_00_00_18, // D
    0x00_42_00_00_00_00_42_00, // E
    0x00_24_42_00_00_42_24_00, // F
    0x00_18_00_42_42_00_18_00, // G
    0x00_00_24_00_00_24_00_00, // H
    0x00_00_18_24_24_18_00_00, // I
    0x00_00_00_18_18_00_00_00, // J
];

fn disk_count(x: u64, mask: u64) -> f32 {
    return (x & mask).count_ones() as f32;
}

pub fn get_endgame_score(board: &board::Board) -> i8 {
    return material::get_material(board).signum();
}

pub fn get_endgame_score_full(board: &board::Board) -> i8 {
    return material::get_material(board);
}

pub fn get_score(board: &mut board::Board, heuristic: &properties::Heuristic) -> f32 {
    let score;
    if board.is_game_over() {
        if board.dark_disks.count_ones() > board.light_disks.count_ones() {
            score = f32::INFINITY;
        } else {
            score = f32::NEG_INFINITY;
        }
    } else {
        let material_score = material::get_material_weighted(board, heuristic);
        let mobility_score = mobility::get_mobility_weighted(board, heuristic);
        score = material_score * heuristic.material_weight +
                mobility_score * heuristic.mobility_weight
    }

    if board.dark_move {
        score + heuristic.bias
    } else {
        -score + heuristic.bias
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
