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

    let (initial, _) = negamax_impl(board, -i32::MAX, i32::MAX, depth / 2, evaluator);
    let mut alpha = initial - 20;
    let mut beta = initial + 20;
    let mut better = board.move_count();

    println!("Running BNS:");

    let moves = board.get_moves();
    let mut best_move = moves[0];

    while beta - alpha >= 2 && better != 1 {
        let guess = next_guess(alpha, beta, better);
        let mut better_count = 0;

        print!("  - α: {}, β: {}, G: {}", alpha, beta, guess);
        io::stdout().flush().expect("asdf");

        for m in &moves {
            let undo = board.make_move(*m);
            let (mut result, nodes) = negamax_impl(board, -guess, -(guess - 1), depth - 1, evaluator);
            board.undo_move(undo, *m);

            result = -result;

            if result >= guess {
                better_count += 1;
                best_move = *m;
            }
        }

        if better_count >= 1 {
            alpha = guess;
            better = better_count;
        } else {
            beta = guess;
        }

        println!(" -- Better: {}", better);
    }

    (alpha, best_move)
}
