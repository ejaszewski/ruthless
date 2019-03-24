/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::time::Instant;

use crate::board::{ Board, Move };

pub fn endgame_solve(board: &mut Board, wld: bool, print: bool) -> (i32, Move) {
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

    if print {
        println!("[{}] Searched {} nodes in {} ms.", if wld { "WLD" } else { "FULL" }, total_nodes, time_taken);
    }

    (best_score, best_move)
}

fn endgame_negamax(board: &mut Board, mut alpha: i32, beta: i32, wld: bool) -> (i32, u64) {
    if board.is_game_over() {
        let score = if board.black_move { board.get_score() } else { -board.get_score() };
        if wld {
            return (score.signum(), 1);
        } else {
            return (score, 1);
        }
    }

    let mut moves = board.get_moves();
    let move_count = board.move_count();
    if move_count > 4 || move_count > 1 && board.all_disks().count_zeros() > 3 {
        moves.sort_unstable_by_key(|&m| board.move_count_after(m));
    }

    let mut total_nodes = 0;

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

#[cfg(test)]
mod test {
    use crate::board::{ Board, Move };

    #[test]
    fn ffo_simplified_40() {
        let mut board = Board::from_pos(0x0101312303010100, 0x9E7ECEDCFC1E0800, true);

        board.make_move(Move::Play(8));
        board.make_move(Move::Play(1));

        assert_eq!(super::endgame_solve(&mut board, true, true).0, 1);

        let (score, m) = super::endgame_solve(&mut board, false, true);
        assert_eq!(score, 38);
        assert_eq!(m, Move::Play(2));
    }
}
