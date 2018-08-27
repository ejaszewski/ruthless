use std::i32;
use std::io::{ self, Write };
use std::time::Instant;

use ::board::{ Board, Move };
use ::search::eval::Evaluator;

pub use ::search::negamax::negamax_impl;

pub fn best_node_search<T: Evaluator>(board: &mut Board, depth: u8, evaluator: &T) -> (i32, Move) {
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

    println!("Running BNS:");

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
        }).map(|m| *m).collect();

        total_nodes += iter_nodes;

        if filtered.len() >= 1 {
            alpha = guess;
            better = filtered.len() as u32;
            moves = filtered;
        } else {
            beta = guess;
        }

        println!(" -- Time: {} ms, Nodes: {} -- Better: {}", time_ms(iter_time_start, Instant::now()), iter_nodes, better);
    }

    println!("BNS Finished. Time: {} ms, Nodes: {}, Best Move: {}", time_ms(total_time_start, Instant::now()), total_nodes, moves[0]);

    (alpha, moves[0])
}
