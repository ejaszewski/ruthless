extern crate ruthless;
extern crate time;

use ruthless::board::*;

fn main() {
    let depth = 11;

    println!("Running perft test at depth {}.", depth);
    let mut board = Board::new();

    let start_time = time::now();
    let nodes = perft_impl(depth, &mut board);
    let end_time = time::now();
    let time_taken = (end_time - start_time).num_milliseconds();
    let nps = nodes as f32 / time_taken as f32;

    println!("Perft Finished:");
    println!("  Nodes      : {} nodes", nodes);
    println!("  Time Taken : {} millis", time_taken);
    println!("  Nodes/Sec  : {} knodes/sec", nps);
}

fn perft_impl(depth: u64, board: &mut Board) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    for m in &board.get_moves() {
        let undo = board.make_move(*m);
        nodes += perft_impl(depth - 1, board);
        board.undo_move(undo, *m);
    }

    nodes
}
