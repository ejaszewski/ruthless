extern crate time;

pub mod search;
pub mod properties;

use std::collections::HashMap;
use std::f32;
use board;
use board::util;

// A B C D D C B A
// B B E F F E B B
// C E G F F G E C
// D F F E E F F D
const SCORE_FUNC: [(u64, f32); 7] = [
    (0x81_00_00_00_00_00_00_81, 20.0), // A
    (0x42_C3_00_00_00_00_C3_42, -5.0), // B
    (0x24_00_81_00_00_81_00_24, 5.0),  // C
    (0x18_00_00_81_81_00_00_18, 5.0),  // D
    (0x00_24_42_00_00_42_24_00, 2.0),  // E
    (0x00_18_18_66_66_18_18_00, 2.0),  // F
    (0x00_00_24_00_00_24_00_00, 2.0),  // G
];

const MATERIAL_WEIGHT: f32 = 0.2;
const MOBILITY_WEIGHT: f32 = 1.0;

fn disk_count(x: u64, mask: u64) -> f32 {
    return (x & mask).count_ones() as f32;
}

pub fn get_score(board: &mut board::Board) -> f32 {
    let mut eval: f32 = 0.0;
    for sc in SCORE_FUNC.iter() {
        eval += (disk_count(board.dark_disks, sc.0) - disk_count(board.light_disks, sc.0)) * sc.1;
    }
    eval *= MATERIAL_WEIGHT;
    eval += board.move_count() as f32 * MOBILITY_WEIGHT;
    if board.dark_move {
        eval
    } else {
        -eval
    }
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

pub fn get_score_endgame_solve(board: &board::Board) -> i8 {
    let modifier = if board.dark_move { 1 } else { -1 };
    (board.dark_disks.count_ones() as i8 - board.light_disks.count_ones() as i8).signum() * modifier
}

pub fn get_move_map(
    board: &mut board::Board,
    moves: &mut Vec<Option<u8>>,
    heuristic: &properties::Heuristic,
    depth: u8,
) -> HashMap<Option<u8>, f32> {
    let mut move_map: HashMap<Option<u8>, f32> = HashMap::new();

    for m in moves {
        let undo = board.make_move(*m);
        let (mut score, _leaves) = search::negamax(board, heuristic, -10000., 10000., depth);
        board.undo_move(undo, *m);

        score = -score;

        move_map.insert(*m, score);
    }

    move_map
}

pub fn do_search(board: &mut board::Board, props: &properties::Properties) -> Option<u8> {

    let mut moves: Vec<Option<u8>> = board.get_moves();

    let mut searched = 0;
    let start_time = time::now();

    eprintln!("{} moves", board.all_disks().count_ones());

    let heuristic = props.get_heuristic(board.all_disks().count_ones());
    let depth = heuristic.depth;

    eprintln!(
        "Current board score: {}",
        get_score_heuristic(board, heuristic)
    );
    eprintln!("Evaluating moves with depth {}.", depth);

    let move_map = get_move_map(board, &mut moves, heuristic, 3);
    moves.sort_unstable_by(|a, b| move_map.get(b).partial_cmp(&move_map.get(a)).unwrap());

    eprintln!("Current move ordering: {:?}", moves);

    let beta = f32::INFINITY;
    let mut best_score = f32::NEG_INFINITY;
    let mut best_move = moves[0];

    for m in &moves {
        let undo = board.make_move(*m);
        let (mut score, leaves) = search::negamax(board, heuristic, -beta, -best_score, depth);
        board.undo_move(undo, *m);

        score = -score;
        searched += leaves;

        if score >= beta {
            break;
        }
        if score > best_score {
            best_move = *m;
            best_score = score;
        }
    }

    let end_time = time::now();
    let time_taken = (end_time - start_time).num_milliseconds();
    let nps = searched as f32 / time_taken as f32;

    eprintln!(
        "Searched {} nodes in {} millis. ({} knodes/sec)",
        searched, time_taken, nps
    );
    eprintln!(
        "Found best move {} with score {}.",
        util::move_string(best_move),
        best_score
    );
    return best_move;
}

pub fn endgame_solve(board: &mut board::Board) -> Option<u8> {
    let moves: Vec<Option<u8>> = board.get_moves();

    let mut searched = 0;
    let start_time = time::now();

    eprintln!("Running endgame solve.");

    let beta = 2;
    let mut best_score = -2;
    let mut best_move = moves[0];

    for m in &moves {
        let undo = board.make_move(*m);
        let (mut score, leaves) = search::negamax_endgame(board, -beta, -best_score);
        board.undo_move(undo, *m);

        score = -score;
        searched += leaves;

        if score >= beta {
            break;
        }
        if score > best_score {
            best_move = *m;
            best_score = score;
        }
    }

    let result_str = ["LOSS", "DRAW", "WIN"][(best_score + 1) as usize];
    eprintln!("Guaranteed {}.", result_str);

    let end_time = time::now();
    let time_taken = (end_time - start_time).num_milliseconds();
    let nps = searched as f32 / time_taken as f32;

    eprintln!(
        "Searched {} nodes in {} millis. ({} knodes/sec)",
        searched, time_taken, nps
    );

    return best_move;
}
