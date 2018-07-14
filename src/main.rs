#[macro_use]
extern crate clap;
extern crate ruthless;

use std::time::Instant;

use clap::App;
use ruthless::board::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("perft") {
        let depth_str = matches.value_of("DEPTH").unwrap();
        if let Ok(depth) = depth_str.parse::<u64>() {
            perft(depth);
        } else {
            panic!("DEPTH must be a positive integer.");
        }
    }
}

fn perft(depth: u64) {
    println!("Running perft test at depth {}.", depth);
    let mut board = Board::new();

    let start_time = Instant::now();
    let nodes = perft_impl(depth, &mut board);
    let end_time = Instant::now();

    let duration = end_time - start_time;
    let time_taken = duration.as_secs() as u32 * 1000 + duration.subsec_millis();

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
