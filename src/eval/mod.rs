extern crate time;

pub mod search;
pub mod properties;

use std::collections::HashMap;
use std::f32;
use board;
use board::util;

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
    let start_time = time::now();

    eprintln!("{} moves", board.all_disks().count_ones());

    let heuristic = props.get_heuristic(board.all_disks().count_ones());
    let depth = heuristic.depth;

    eprintln!(
        "Current board score: {}",
        get_score_heuristic(board, heuristic)
    );
    eprintln!("Evaluating moves with depth {}.", depth);

    // Standard negamax search.
    let (best_move, best_score, searched) = do_negamax(board, heuristic);

    // Best Node Search
    // let (best_move, searched) = do_best_node_search(board, heuristic);
    // let best_score = -1;

    eprintln!("Avg. Branching Factor: {}", (searched as f32).powf(1. / depth as f32));

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

fn do_negamax(board: &mut board::Board, heuristic: &properties::Heuristic) -> (Option<u8>, f32, u64) {
    let mut moves: Vec<Option<u8>> = board.get_moves();
    let mut searched = 0;
    let depth = heuristic.depth;

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

    (best_move, best_score, searched)
}

fn do_best_node_search(board: &mut board::Board, heuristic: &properties::Heuristic) -> (Option<u8>, u64) {
    let mut moves: Vec<Option<u8>> = board.get_moves();
    let mut searched = 0;
    let depth = heuristic.depth;

    let move_map = get_move_map(board, &mut moves, heuristic, 3);
    moves.sort_unstable_by(|a, b| move_map.get(b).partial_cmp(&move_map.get(a)).unwrap());

    eprintln!("Current move ordering: {:?}", moves);

    let mut alpha = get_score_heuristic(board, heuristic) - 20.;
    let mut beta = get_score_heuristic(board, heuristic) + 20.;
    let mut better_count = 0;
    let mut subtree_count = moves.len() as u32;

    // let mut best_score = f32::NEG_INFINITY;
    let mut best_move = moves[0];

    while !((beta - alpha < 2.0) || (better_count == 1)) {

        better_count = 0;
        let guess = alpha + (beta - alpha) * (subtree_count as f32 - 1.) / subtree_count as f32;

        for m in &moves {
            let undo = board.make_move(*m);
            let (mut score, leaves) = search::negamax(board, heuristic, -guess, -(guess - 1.), depth);
            board.undo_move(undo, *m);

            score = -score;
            searched += leaves;

            if score >= guess {
                better_count += 1;
                best_move = *m;
            }
        }

        if better_count > 0 {
            alpha = guess;
            subtree_count = better_count;
        } else {
            beta -= 5.;
        }
    }

    (best_move, searched)
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
