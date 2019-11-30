/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Contains an implementation of Best Node Search, a fuzzy search algorithm, which uses multiple
//! zero-window Negamax calls to find the best move.
//!
//! # Algorithm:
//! Best Node Search is an algorithm detailed in 2011 which uses several zero-window Negamax
//! searches to find the best move. It is a "fuzzified" algorithm, meaning that it doesn't
//! necessarily find the exact score for any of the moves, but instead determines which move is
//! best by finding the score which only one move exceeds. This is done by setting search bounds,
//! guessing what score will separate one move, then performing zero-window searches on each move.
//! The bounds and guess are tweaked, and the process is repeated until the best move is found.
//!
//! This algorithm is a theoretical improvement over standard Negamax and NegaScout because with a
//! relatively accurate guess, multiple moves can be discarded with a relatively fast zero-window
//! search. The algorithm benefits from accurate upper and lower bounds, and statistical tuning of
//! guessing from the bounds.
//!
//! # Implementation:
//! The Best Node Search implemented in this module is a slight modification of the one presented
//! in the original paper, with some small improvements. It uses the negamax implementation from
//! the `negamax` package for its zero-window searches. The main modification made to the
//! implementation described in the paper is that moves which are below a cutoff are discarded as
//! long as at least one move is greater than the cutoff. This means that once a move is known to
//! not be the best, it is never searched again.

// TODO: Get link to paper on BNS

use std::i32;
use std::io::{ self, Write };
use std::time::Instant;

use crate::board::{ Board, Move };
use crate::search::{ SearchData, eval::Evaluator };

pub use crate::search::negamax::negamax_impl;

/// A BNS implementation which returns the best move for a curent position, along with score.
/// This function should be called only if the best move is what is desired. Prints information
/// about the search to stdout.
/// # Arguments:
/// * `board`: Board to search.
/// * `depth`: Depth to search to.
/// * `evaluator`: Evaluator to use for position evaluation at a leaf.
/// # Returns:
/// * A tuple containing the score of the best move and the best move.
pub fn best_node_search<T: Evaluator>(board: &mut Board, depth: u8, evaluator: &T) -> (i32, Move, SearchData) {
    let next_guess = | a: i32, b: i32, count: u32 | {
        a + ((b - a) as f32 * ((count as f32 - 1.0) / count as f32)) as i32
    };

    let time_ms = | start: Instant, end: Instant | {
        let duration = end - start;
        let secs = duration.as_secs();
        let millis = duration.subsec_millis();

        (secs as u32 * 1000) + millis
    };

    let (initial, _) = negamax_impl(board, -i32::MAX, i32::MAX, depth / 2, evaluator);
    let mut alpha = initial - 20;
    let mut beta = initial + 20;
    let mut better = board.move_count();

    println!("Running BNS depth {}:", depth);

    let total_time_start = Instant::now();
    let mut total_nodes = 0;

    let mut moves = board.get_moves();

    while beta - alpha >= 2 && better != 1 {
        let iter_time_start = Instant::now();
        let mut iter_nodes = 0;

        let guess = next_guess(alpha, beta, better);

        print!("  - α: {}, β: {}, G: {}", alpha, beta, guess);
        io::stdout().flush().expect("asdf");

        let filtered: Vec<Move> = moves.iter().filter(| &m | {
            let undo = board.make_move(*m);
            let (mut result, nodes) = negamax_impl(board, -guess, -(guess - 1), depth - 1, evaluator);
            board.undo_move(undo, *m);

            result = -result;
            iter_nodes += nodes;

            result >= guess
        }).cloned().collect();

        total_nodes += iter_nodes;

        if !filtered.is_empty() {
            alpha = guess;
            better = filtered.len() as u32;
            moves = filtered;
        } else {
            beta = guess;
        }

        println!(" -- Time: {} ms, Nodes: {} -- Better: {}", time_ms(iter_time_start, Instant::now()), iter_nodes, better);
    }

    let total_time = time_ms(total_time_start, Instant::now());

    println!("BNS Finished. Time: {} ms, Nodes: {}, Best Move: {}", total_time, total_nodes, moves[0]);

    (alpha, moves[0], SearchData { nodes: total_nodes, time: total_time, depth })
}

#[cfg(test)]
mod test {
    use crate::board::{ Board, Move };
    use crate::search::{ bns, eval::PieceSquareEvaluator };

    #[test]
    fn test_best_node_search() {
        let mut board = Board::from_pos(0x000040BC00000000, 0x0000004000000000, false);
        let eval = PieceSquareEvaluator::from([1; 10]);

        let (_score, m) = bns::best_node_search(&mut board, 2, &eval);

        assert_eq!(m, Move::Play(9));
    }
}
