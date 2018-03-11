extern crate time;

use board;
use eval::{negamax, properties, score};
use std::collections::HashMap;
use std::f32;

pub fn negamax(board: &mut board::Board, heuristic: &properties::Heuristic) -> (Option<u8>, f32, u64) {
    let mut moves: Vec<Option<u8>> = board.get_moves();
    let mut searched = 0;
    let depth = heuristic.depth;

    let move_map = score::get_move_map(board, &mut moves, heuristic, 3);
    moves.sort_unstable_by(|a, b| move_map.get(b).partial_cmp(&move_map.get(a)).unwrap());

    eprintln!("Current move ordering: {:?}", moves);

    let beta = f32::INFINITY;
    let mut best_score = f32::NEG_INFINITY;
    let mut best_move = moves[0];

    for m in &moves {
        let undo = board.make_move(*m);
        let (mut score, leaves) = negamax::negamax(board, heuristic, -beta, -best_score, depth);
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

pub fn endgame_solve_fast(board: &mut board::Board) -> (Option<u8>, i8) {
    let mut moves: Vec<Option<u8>> = board.get_moves();

    eprintln!("{} moves on board.", moves.len());

    let mut searched = 0;
    let start_time = time::now();

    eprintln!("Running endgame solve.");

    let ff = score::get_fastest_first_arr(board, &mut moves);
    moves.sort_unstable_by(|a, b| ff[a.unwrap() as usize].cmp(&ff[b.unwrap() as usize]));

    let beta = 1;
    let mut best_score = -1;
    let mut best_move = moves[0];

    for m in moves {
        let undo = board.make_move(m);
        let (mut score, leaves) = negamax::negamax_endgame(board, -beta, -best_score);
        board.undo_move(undo, m);

        score = -score;
        searched += leaves;

        if score >= beta {
            best_move = m;
            best_score = beta;
            break;
        }
        if score > best_score {
            best_move = m;
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

    return (best_move, best_score);
}

pub fn endgame_solve_full(board: &mut board::Board) -> (Option<u8>, i8) {
    let mut moves: Vec<Option<u8>> = board.get_moves();

    let mut searched = 0;
    let start_time = time::now();

    eprintln!("Running endgame solve.");

    let ff = score::get_fastest_first_arr(board, &mut moves);
    moves.sort_unstable_by(|a, b| ff[a.unwrap() as usize].cmp(&ff[b.unwrap() as usize]));

    let beta = 64;
    let mut best_score = -64;
    let mut best_move = moves[0];

    for m in moves {
        let undo = board.make_move(m);
        let (mut score, leaves) = negamax::negamax_endgame_full(board, -beta, -best_score);
        board.undo_move(undo, m);

        eprintln!("Evaluated: {:?}, α: {}, β: {}", m, best_score, beta);

        score = -score;
        searched += leaves;

        if score >= beta {
            best_move = m;
            best_score = beta;
            break;
        }
        if score > best_score {
            best_move = m;
            best_score = score;
        }
    }

    eprintln!("Best score: {}", best_score);

    let dark_disks = 32 + if board.dark_move { best_score / 2 } else { -best_score / 2 };

    let result_str = ["LOSS", "DRAW", "WIN"][(best_score.signum() + 1) as usize];
    eprintln!("Guaranteed {} with disk count {}/{}", result_str, dark_disks, 64 - dark_disks);

    let end_time = time::now();
    let time_taken = (end_time - start_time).num_milliseconds();
    let nps = searched as f32 / time_taken as f32;

    eprintln!(
        "Searched {} nodes in {} millis. ({} knodes/sec)",
        searched, time_taken, nps
    );

    return (best_move, best_score);
}

pub fn best_node_search(board: &mut board::Board, heuristic: &properties::Heuristic) -> (Option<u8>, u64) {
    let mut moves: Vec<Option<u8>> = board.get_moves();
    let mut searched = 0;
    let mut table_hits = 0;
    let depth = heuristic.depth;

    let move_map = score::get_move_map(board, &mut moves, heuristic, 3);
    moves.sort_unstable_by(|a, b| move_map.get(b).partial_cmp(&move_map.get(a)).unwrap());

    eprintln!("Current move ordering: {:?}", moves);

    let mut alpha = score::get_score(board, heuristic) - 5.;
    let mut beta = score::get_score(board, heuristic) + 5.;
    let mut better_count = 0;
    let mut subtree_count = moves.len() as u32;

    let mut hash_table: HashMap<board::Position, board::NodeType> = HashMap::new();

    // let mut best_score = f32::NEG_INFINITY;
    let mut best_move = moves[0];

    while !((beta - alpha < 1.) || (better_count == 1)) {
        better_count = 0;
        let guess = alpha + (beta - alpha) * (subtree_count as f32 - 1.) / subtree_count as f32;

        eprintln!("BNS: α: {}, β: {}, G: {}", alpha, beta, guess);

        for m in &moves {
            let undo = board.make_move(*m);
            let (mut score, leaves, hits) = negamax::negamax_tpt(board, heuristic, &mut hash_table, -guess, -(guess - 0.1), depth);
            board.undo_move(undo, *m);

            score = -score;
            searched += leaves;
            table_hits += hits;

            if score >= guess {
                better_count += 1;
                best_move = *m;
            }
        }

        eprintln!("Results: {} of {} above", better_count, subtree_count);


        if better_count > 0 {
            alpha = guess;
            subtree_count = better_count;
        } else {
            beta = guess;
        }

        let trim_before = hash_table.len();
        hash_table.retain(|_, nt| {
            match *nt {
                board::NodeType::AllNode(lower) => { lower < alpha },
                _ => { true }
            }
        });
        let trim_after = hash_table.len();
        eprintln!("Trimmed {} entries from HT.", trim_before - trim_after);
    }

    eprintln!("{} Nodes Searched\n{} Table Hits\n{} Table Entries", searched, table_hits, hash_table.len());

    (best_move, searched)
}
