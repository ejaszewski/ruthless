extern crate time;

pub mod negamax;
pub mod properties;
pub mod search;
pub mod score;

use std::f32;
use board;
use board::util;

pub fn do_search(board: &mut board::Board, props: &properties::Properties) -> Option<u8> {
    let start_time = time::now();

    eprintln!("{} moves", board.all_disks().count_ones());

    let heuristic = props.get_heuristic(board.all_disks().count_ones());
    let depth = heuristic.depth;

    eprintln!(
        "Current board score: {}",
        score::get_score_heuristic(board, heuristic)
    );
    eprintln!("Evaluating moves with depth {}.", depth);

    // Standard negamax search.
    let (best_move, best_score, searched) = search::negamax(board, heuristic);
    eprintln!("Avg. Branching Factor: {}", (searched as f32).powf(1. / depth as f32));

    // board.clear_moves();
    //
    // // Best Node Search
    // let (best_move, searched) = search::best_node_search(board, heuristic);
    // let best_score = "?";
    // eprintln!("Avg. Branching Factor: {}", (searched as f32).powf(1. / depth as f32));

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
