/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate clap;
extern crate ruthless;

use std::io::{ self, BufRead, Write };
use std::time::Instant;

use clap::App;
use ruthless::board::{ self, Move };
use ruthless::search::{ endgame, negamax, bns, eval::PieceSquareEvaluator };

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(perft_matches) = matches.subcommand_matches("perft") {
        let depth_str = perft_matches.value_of("DEPTH").unwrap();
        if let Ok(depth) = depth_str.parse::<u8>() {
            perft(depth);
        } else {
            panic!("DEPTH must be a positive integer less than 256.");
        }
    }

    if let Some(_play_matches) = matches.subcommand_matches("play") {
        play();
    }
}

fn play() {
    let mut board = board::Board::new();
    let stdin = io::stdin();
    let mut undo_stack: Vec<(u64, Move)> = Vec::new();

    let print_info = |b: &mut board::Board| {
        println!("\n{}", b);
        print!("Playable Moves:\n\t");
        for m in b.get_moves() {
            print!("{} ", m);
        }
        println!("\n");
        print!("Enter a command: ");
        io::stdout().flush().expect("Unable to flush stdout.");
    };

    print_info(&mut board);

    for line in stdin.lock().lines() {
        // Get input from the player.
        if let Ok(text) = line {
            let split: Vec<&str> = text.split(' ').collect();

            if split[0] == "exit" {
                break;
            } else if split[0] == "play" {
                let m = Move::from_coord(split[1]);

                if board.get_moves().contains(&m) {
                    println!("Playing move: {}", m);
                    // Make the move and save for undo.
                    let undo_info = board.make_move(m);
                    undo_stack.push((undo_info, m));
                } else {
                    println!("Invalid move. Try another.");
                }
            } else if split[0] == "undo" {
                // Pop a move off the undo stack (if possible) and undo it.
                if let Some((undo, m)) = undo_stack.pop() {
                    board.undo_move(undo, m);
                    println!("Undoing move {}", m);
                } else {
                    println!("No moves to undo!");
                }
            } else if split[0] == "go" {
                // Get the search depth
                let depth = match split.get(1) {
                    Some(dep) => match dep.parse::<u8>() { Ok(x) => x, _ => 8 },
                    None => 8
                };

                // Get the best move.
                let (score, best_move) = if depth < board.all_disks().count_zeros() as u8 {
                    // If the search is not full depth, then run a normal search.
                    if let Some(&alg) = split.get(2) {
                        match alg {
                            "nm" => negamax::negamax(&mut board, depth, &PieceSquareEvaluator::new()),
                            "bns" => bns::best_node_search(&mut board, depth, &PieceSquareEvaluator::new()),
                            _ => negamax::negamax(&mut board, depth, &PieceSquareEvaluator::new())
                        }
                    } else {
                        negamax::negamax(&mut board, depth, &PieceSquareEvaluator::new())
                    }
                } else {
                    // If the search will be full-depth, then just endgame solve.
                    endgame::endgame_solve(&mut board, false)
                };

                println!("Computer is playing {}, which had score {}.", best_move, score);
                // Make move and save undo information.
                let undo_info = board.make_move(best_move);
                undo_stack.push((undo_info, best_move));
            } else {
                println!("Invalid action. Must be one of:");
                let actions = vec![ "play <coord>", "go [depth] [algorithm]", "undo", "exit" ];
                for action in actions {
                    println!("  - {}", action);
                }
            }

            // Handle a game over.
            if board.is_game_over() {
                let black_disks = board.black_disks.count_ones();
                let white_disks = board.white_disks.count_ones();

                println!("Game Over! Score {} to {}", black_disks, white_disks);

                if black_disks > white_disks {
                    println!("BLACK wins!");
                } else if black_disks < white_disks {
                    println!("WHITE wins!");
                } else {
                    println!("TIE!");
                }
                break;
            }

            // Print info.
            print_info(&mut board);
        } else {
            break;
        }
    }
}

fn perft(depth: u8) {
    println!("Running perft test at depth {}.", depth);
    let mut board = board::Board::new();

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

fn perft_impl(depth: u8, board: &mut board::Board) -> u64 {
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
