extern crate time;

pub mod negamax;
pub mod properties;
pub mod search;
pub mod score;

use std::f32;
use board;
use board::util;

pub fn do_search(board: &mut board::Board, props: &properties::Properties, time_allocated: f32) -> (Option<u8>, f32) {
    let start_time = time::now();

    let heuristic = props.get_heuristic(board.all_disks().count_ones());
    let depth = heuristic.depth;

    let max_depth = board.all_disks().count_zeros() as u8;

    eprintln!(
        "Current board score: {}",
        score::get_score(board, heuristic)
    );
    eprintln!("{} ms allocated for search.", time_allocated);

    // Standard negamax search.
    let (best_move, best_score, searched, branching_factor) = search::iterative_deepening(board, props, 7, max_depth, time_allocated);
    eprintln!("Avg. Branching Factor ABP : {}", branching_factor);

    // board.clear_moves();

    // Best Node Search
    // let (best_move, searched) = search::best_node_search(board, heuristic);
    // let best_score = "?";

    // let branching_factor = (searched as f32).powf(1. / depth as f32);
    // eprintln!("Avg. Branching Factor BNS : {}", branching_factor);

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
    return (best_move, branching_factor);
}
