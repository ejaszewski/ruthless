use std::time::Instant;

use ::board::{ Board, Move };

pub fn endgame_solve(board: &mut Board, wld: bool) -> (i8, Move) {
    let start_time = Instant::now();
    let mut total_nodes = 0;

    let mut moves = board.get_moves();
    moves.sort_unstable_by_key(|&m| board.move_count_after(m));

    let beta = if wld { 1 } else { 64 };
    let mut best_score = -beta;
    let mut best_move = moves[0];

    for m in moves {
        let undo = board.make_move(m);
        let (mut result, nodes) = endgame_negamax(board, -beta, -best_score, wld);
        board.undo_move(undo, m);

        total_nodes += nodes;

        result = -result;

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

    let end_time = Instant::now();
    let duration = end_time - start_time;
    let time_taken = duration.as_secs() as u32 * 1000 + duration.subsec_millis();

    println!("[{}] Searched {} nodes in {} ms.", if wld { "WLD" } else { "FULL" }, total_nodes, time_taken);

    return (best_score, best_move);
}

fn endgame_negamax(board: &mut Board, mut alpha: i8, beta: i8, wld: bool) -> (i8, u64) {
    if board.is_game_over() {
        let score = if board.black_move { board.get_score() } else { -board.get_score() };
        if wld {
            return (score.signum(), 1);
        } else {
            return (score, 1);
        }
    }

    let mut moves = board.get_moves();
    if board.move_count() > 1 && board.all_disks().count_zeros() > 3 {
        moves.sort_unstable_by_key(|&m| board.move_count_after(m));
    }

    let mut total_nodes = 1;

    for m in moves {
        let undo = board.make_move(m);
        let (mut result, nodes) = endgame_negamax(board, -beta, -alpha, wld);
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
