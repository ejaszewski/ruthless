use std::i32;
use std::io::{ self, Write };
use std::time::Instant;

use ::board::{ Board, Move };
use ::search::eval::Evaluator;

pub fn negamax<T: Evaluator>(board: &mut Board, depth: u8, evaluator: &T) -> (i32, Move) {
    let mut moves = board.get_moves();
    moves.sort_unstable_by_key(|&m| board.move_count_after(m)); // TODO: Better move ordering.

    let beta = i32::MAX;
    let mut best_score = -beta;
    let mut best_move = moves[0];

    for m in moves {
        print!("Evaluating: {}", m);
        io::stdout().flush().expect("Unable to flush stdout.");

        let start_time = Instant::now();

        let undo = board.make_move(m);
        let (mut result, nodes) = negamax_impl(board, -beta, best_score, depth - 1, evaluator);
        board.undo_move(undo, m);

        result = -result;

        let end_time = Instant::now();
        let duration = end_time - start_time;
        let time_taken = duration.as_secs() as u32 * 1000 + duration.subsec_millis();

        println!(" -- Score: {}, Nodes: {}, Time {} ms", result, nodes, time_taken);

        if result >= beta {
            best_move = m;
            best_score = beta;
            break;
        }

        if result > best_score {
            best_move = m;
            best_score = result;
        }
    }

    return (best_score, best_move);
}

pub fn negamax_impl<T: Evaluator>(board: &mut Board, mut alpha: i32, beta: i32, depth: u8, evaluator: &T) -> (i32, u64) {
    if board.is_game_over() || depth == 0 {
        return (evaluator.get_score(board), 1);
    }

    let mut moves = board.get_moves();
    moves.sort_unstable_by_key(|&m| {
        let undo = board.make_move(m);
        let score = evaluator.get_score(board);
        board.undo_move(undo, m);
        score
    });

    let mut total_nodes = 1;

    for m in moves {
        let undo = board.make_move(m);
        let (mut result, nodes) = negamax_impl(board, -beta, -alpha, depth - 1, evaluator);
        board.undo_move(undo, m);

        result = -result;
        total_nodes += nodes;

        if result > alpha {
            alpha = result;
        }

        if alpha >= beta {
            break;
        }
    }

    (alpha, total_nodes)
}
