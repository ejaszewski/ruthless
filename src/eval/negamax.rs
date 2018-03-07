use std::collections::HashMap;
use board::{Board, Position, NodeType};
use eval::properties::Heuristic;
use eval::{score};

pub fn negamax(board: &mut Board, heuristic: &Heuristic, mut alpha: f32, beta: f32, depth: u8) -> (f32, u64) {
    if depth == 0 {
        return (score::get_score_heuristic(board, heuristic), 1);
    }
    let mut count = 0;

    let mut moves = board.get_moves();

    if depth > 3 {
        let move_map = score::get_move_map(board, &mut moves, heuristic, 1);
        moves.sort_unstable_by(|a, b| move_map.get(b).partial_cmp(&move_map.get(a)).unwrap());
    }

    for m in &moves {
        let undo = board.make_move(*m);
        let result = negamax(board, heuristic, -beta, -alpha, depth - 1);
        let score = -result.0;
        count += result.1;
        board.undo_move(undo, *m);

        if score >= beta {
            return (beta, count);
        }
        if score > alpha {
            alpha = score;
        }
    }
    return (alpha, count);
}

pub fn negamax_tpt(board: &mut Board, heuristic: &Heuristic, tpt: &mut HashMap<Position, NodeType>, mut alpha: f32, beta: f32, depth: u8) -> (f32, u64, u64) {
    let node = board.get_position();
    let mut table_hits = 0;

    if tpt.contains_key(&node) {
        match *tpt.get(&node).unwrap() {
            NodeType::AllNode(lower_bound) => {
                if lower_bound > beta {
                    return (lower_bound, 0, 1);
                }
            },
            NodeType::CutNode(upper_bound) => {
                if upper_bound < alpha {
                    return (upper_bound, 0, 1);
                }
            },
            NodeType::ScoreNode(score) => {
                return (score, 0, 1);
            }
        }
    }

    if depth == 0 {
        let score = score::get_score_heuristic(board, heuristic);
        return (score, 1, 0);
    }
    let mut count = 0;

    let mut moves = board.get_moves();

    if depth > 3 {
        let move_map = score::get_move_map(board, &mut moves, heuristic, 1);
        moves.sort_unstable_by(|a, b| move_map.get(b).partial_cmp(&move_map.get(a)).unwrap());
    }

    for m in &moves {
        let undo = board.make_move(*m);
        let result = negamax_tpt(board, heuristic, tpt, -beta, -alpha, depth - 1);
        let sub_node = board.get_position();
        let score = -result.0;
        count += result.1;
        table_hits += result.2;
        board.undo_move(undo, *m);

        if depth == 1 {
            // tpt.insert(sub_node, NodeType::ScoreNode(score));
        } else if depth > 2 {
            if score < alpha {
                tpt.insert(sub_node, NodeType::CutNode(score));
            } else if score >= beta {
                tpt.insert(sub_node, NodeType::AllNode(score));
            }
        }

        if score >= beta {
            return (beta, count, table_hits);
        }
        if score > alpha {
            alpha = score;
        }
    }
    return (alpha, count, table_hits);
}

pub fn negamax_endgame(board: &mut Board, mut alpha: i8, beta: i8) -> (i8, u64) {
    if board.is_game_over() {
        return (score::get_endgame_score(board), 1);
    }

    let mut moves = board.get_moves();

    if board.move_count() > 1 && board.all_disks().count_zeros() > 3 {
        let ff = score::get_fastest_first_arr(board, &mut moves);
        moves.sort_unstable_by(|a, b| ff[a.unwrap() as usize].cmp(&ff[b.unwrap() as usize]));
    }

    let mut count = 0;
    for m in moves {
        let undo = board.make_move(m);
        let result = negamax_endgame(board, -beta, -alpha);
        let score = -result.0;
        count += result.1;
        board.undo_move(undo, m);

        if score >= beta {
            return (beta, count);
        }
        if score > alpha {
            alpha = score;
        }
    }
    return (alpha, count);
}

pub fn negamax_endgame_full(board: &mut Board, mut alpha: i8, beta: i8) -> (i8, u64) {
    if board.is_game_over() {
        return (score::get_parity(board), 1);
    }

    let mut moves = board.get_moves();

    if board.move_count() > 1 && (board.move_count() > 4 || board.all_disks().count_zeros() > 3) {
        let ff = score::get_fastest_first_arr(board, &mut moves);
        moves.sort_unstable_by(|a, b| ff[a.unwrap() as usize].cmp(&ff[b.unwrap() as usize]));
    }

    let mut count = 0;
    for m in moves {
        let undo = board.make_move(m);
        let result = negamax_endgame_full(board, -beta, -alpha);
        let score = -result.0;
        count += result.1;
        board.undo_move(undo, m);

        if score >= beta {
            return (beta, count);
        }
        if score > alpha {
            alpha = score;
        }
    }
    return (alpha, count);
}
