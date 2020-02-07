/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Contains an implementation of the Negamax search algorithm with Alpha-Beta pruning.
//! # Implementation:
//! The algorithm used is a standard implementation of Negamax, which returns a score relative to
//! the side playing (positive for winning, negative for losing). Both the root negamax (`negamax`)
//! and implementation (`negamax_impl`) also have alpha-beta puning.
//!
//! The root method begins with upper and lower bounds of `i32::MAX` and `-i32::MAX` for the score,
//! respectively, and runs a negamax search on each move in the position, to determine the score
//! for each move. The lower bound is tweaked after each search to minimize the search window, so
//! an accurate score is guaranteed only for the best move, but all moves will be searched.
//!
//! The implementation Negamax uses the provided upper and lower bounds, tweaking appropriately, so
//! is not guaranteed to search every move in the position. Scores must be in the range [`alpha`,
//! `beta`], so alpha-cutoffs will return alpha, while beta-cutoffs will return beta.

use std::collections::HashMap;
use std::i32;
use std::io::{ self, Write };
use std::time::Instant;

use crate::board::{ Board, Move };
use crate::search::{ SearchData, eval::Evaluator };

const MIN_SEARCH_DEPTH: u8 = 8;

/// A Negamax implementation which returns the best move for a curent position, along with score.
/// This function should be called only if the best move is what is desired. Prints information
/// about the search to stdout.
/// # Arguments:
/// * `board`: Board to search.
/// * `depth`: Depth to search to.
/// * `evaluator`: Evaluator to use for position evaluation at a leaf.
/// # Returns:
/// * A tuple containing the score of the best move and the best move.
pub fn negamax<T: Evaluator>(board: &mut Board, depth: u8, evaluator: &T, print: bool) -> (i32, Move, SearchData) {
    let mut moves = board.get_moves();
    moves.sort_by(|&m| -evaluator.move_order_score(board, m));

    let beta = i32::MAX;
    let mut best_score = -beta;
    let mut best_move = moves[0];

    let mut total_nodes = 0;
    let mut total_millis = 0;

    for m in &moves {
        if print {
            print!("Evaluating: {}", m);
            io::stdout().flush().expect("Unable to flush stdout.");
        }

        let start_time = Instant::now();

        let undo = board.make_move(m);
        let (mut result, nodes) = negamax_impl(board, -beta, -best_score, depth - 1, evaluator);
        board.undo_move(undo, m);

        result = -result;

        let end_time = Instant::now();
        let duration = end_time - start_time;
        let time_taken = duration.as_secs() as u32 * 1000 + duration.subsec_millis();

        if result > best_score {
            if print {
                println!(" -- Score: {:3}, Nodes: {}, Time {} ms", result, nodes, time_taken);
            }
            best_move = m;
            best_score = result;
        } else if print {
            println!(" --             Nodes: {}, Time {} ms", nodes, time_taken);
        }

        total_nodes += nodes;
        total_millis += time_taken;
    }
    
    if print {
        println!("Searched {} nodes in {} ms ({:.2} kn/s)", total_nodes, total_millis, total_nodes as f32 / total_millis as f32);
    }

    (best_score, best_move, SearchData { nodes: total_nodes, time: total_millis, depth })
}

/// A Negamax implementation which returns the best move for a curent position, along with score.
/// This function should be called only if the best move is what is desired. Prints information
/// about the search to stdout.
/// # Arguments:
/// * `board`: Board to search.
/// * `depth`: Depth to search to.
/// * `evaluator`: Evaluator to use for position evaluation at a leaf.
/// # Returns:
/// * A tuple containing the score of the best move and the best move.
pub fn negamax_id<T: Evaluator>(board: &mut Board, time: u32, evaluator: &T, print: bool) -> (i32, Move, SearchData) {
    let mut moves = board.get_moves();
    moves.sort_by(|&m| -evaluator.move_order_score(board, m));

    let mut scores: HashMap<Move, i32> = HashMap::new();

    let mut total_nodes = 0;
    let mut total_millis = 0;

    let mut depth = MIN_SEARCH_DEPTH;
    let mut time_prediction = 0;

    let mut branching_factor;

    while time_prediction + total_millis < time {
        let beta = i32::MAX;

        let mut best_score = -beta;

        let mut iter_nodes = 0;
        let mut iter_time = 0;

        for m in &moves {
            if print {
                print!("Evaluating: {}", m);
                io::stdout().flush().expect("Unable to flush stdout.");
            }

            let start_time = Instant::now();

            let undo = board.make_move(m);
            let (mut result, nodes) = negamax_impl(board, -beta, -best_score, depth - 1, evaluator);
            board.undo_move(undo, m);

            result = -result;

            scores.insert(m, result);

            let end_time = Instant::now();
            let duration = end_time - start_time;
            let time_taken = duration.as_secs() as u32 * 1000 + duration.subsec_millis();

            if result > best_score {
                if print {
                    println!(" -- Score: {:3}, Nodes: {}, Time {} ms", result, nodes, time_taken);
                }
                best_score = result;
            } else if print {
                println!(" --             Nodes: {}, Time {} ms", nodes, time_taken);
            }
            
            iter_nodes += nodes;
            iter_time += time_taken;
            total_nodes += nodes;
            total_millis += time_taken;
        }

        moves.sort_by(|&m| -scores.get(&m).unwrap());

        depth += 1;

        branching_factor = (iter_nodes as f32).powf(1.0 / depth as f32);
        time_prediction = (iter_time as f32 * (branching_factor + 1.0)) as u32;

        eprintln!("Took {} ms", iter_time);
        eprintln!("Predicted next time is {} ms", time_prediction);
    }

    depth -= 1;
    
    if print {
        println!("Searched {} nodes in {} ms ({:.2} kn/s)", total_nodes, total_millis, total_nodes as f32 / total_millis as f32);
        println!("Final search was depth {}", depth);
    }

    (*scores.get(&moves[0]).unwrap(), moves[0], SearchData { nodes: total_nodes, time: total_millis, depth })
}

/// A Negamax implementation which returns the score of the best move in current position for the
/// current player. Evaluates score whenever the depth limit is hit or the game is over. Scores
/// returned are always in the range [`alpha`, `beta`].
/// # Arguments:
/// * `board`: Board to search.
/// * `depth`: Depth to search to.
/// * `alpha`: Alpha-cutoff.
/// * `beta`: Beta-cutoff.
/// * `evaluator`: Evaluator to use for position evaluation at a leaf.
/// # Returns:
/// * A tuple containing the score of the best move and the number of nodes searched.
pub fn negamax_impl<T: Evaluator>(board: &mut Board, mut alpha: i32, beta: i32, depth: u8, evaluator: &T) -> (i32, u64) {
    if board.is_game_over() || depth == 0 {
        return (evaluator.get_score(board), 1);
    }

    let mut moves = board.get_moves();
    if depth > 3 {    
        moves.sort_by(|&m| -evaluator.move_order_score(board, m));
    }

    let mut total_nodes = 1;

    for m in &moves {
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

#[cfg(test)]
mod test {
    use crate::board::{ Board, Move };
    use crate::search::{ negamax, eval::PieceSquareEvaluator };

    #[test]
    fn test_negamax() {
        let mut board = Board::from_pos(0x000040BC00000000, 0x0000004000000000, false);
        let eval = PieceSquareEvaluator::from([1; 10]);

        let (_score, m, _) = negamax::negamax(&mut board, 2, &eval, true);

        assert_eq!(m, Move::Play(9));
    }
}
