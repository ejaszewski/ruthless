/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{ self, BufRead, BufReader, Write };
use std::time::Instant;

use clap::App;
use rand::Rng;
use rayon::prelude::*;
use ruthless::board::{ self, Move, Board, Position };
use ruthless::search::{ endgame, negamax, bns, iterative, eval::{ PatternEvaluator } };
use ruthless::search::endgame::EndgameSearcher;
use serde::Deserialize;
use serde_json::from_reader;

#[derive(Deserialize)]
struct PatternFile {
    masks: Vec<u64>,
    weights: Vec<Vec<f32>>
}

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

    if let Some(gtd) = matches.subcommand_matches("gen-training-data") {
        let empties_str = gtd.value_of("EMPTIES").unwrap();
        let num_pos_str = gtd.value_of("NUM_POSITIONS").unwrap();
        let output_file = gtd.value_of("FILE").unwrap();
        let depth_maybe_str = gtd.value_of("DEPTH").unwrap_or("");

        if let Ok(empties) = empties_str.parse::<u8>() {
            if let Ok(num_pos) = num_pos_str.parse::<usize>() {
                if let Ok(depth) = depth_maybe_str.parse::<u8>() {
                    let positions = training_data_heuristic(empties, depth, num_pos);
                    println!("Serializing position data...");
                    let json_out = serde_json::to_string(&positions).unwrap_or(String::new());
                    println!("Writing output to file...");
                    let mut file = File::create(output_file).unwrap();
                    file.write_all(json_out.as_bytes()).expect("Unable to write to output file.");
                    println!("Done.");
                } else {
                    let positions = training_data_solve(empties, num_pos);
                    println!("Serializing position data...");
                    let json_out = serde_json::to_string(&positions).unwrap_or(String::new());
                    println!("Writing output to file...");
                    let mut file = File::create(output_file).unwrap();
                    file.write_all(json_out.as_bytes()).expect("Unable to write to output file.");
                    println!("Done.");
                }
            } else {
                panic!("NUM_POSITIONS must be a positive integer.");
            }
        } else {
            panic!("EMPTIES must be a positive integer less than 256.");
        }
    }

    if let Some(_play_matches) = matches.subcommand_matches("play") {
        play();
    }

    if let Some(cs2_matches) = matches.subcommand_matches("cs2l") {
        let board = board::Board::new();
        let black = cs2_matches.value_of("COLOR").unwrap() == "Black";

        cs2_play(board, black);
    }
}

fn play() {
    let mut board = board::Board::new();
    let stdin = io::stdin();
    let mut undo_stack: Vec<(u64, Move)> = Vec::new();

    let print_info = |b: &mut board::Board| {
        println!("\n{}", b);
        print!("Playable Moves:\n\t");
        for m in &b.get_moves() {
            print!("{} ", m);
        }
        println!("\n");
        print!("Enter a command: ");
        io::stdout().flush().expect("Unable to flush stdout.");
    };

    let file = File::open("pat9-12-fp.json").expect("File read error.");
    let reader = BufReader::new(file);
    let pat_file: PatternFile = from_reader(reader).expect("Unable to parse json");

    let pat_eval = PatternEvaluator::from(pat_file.masks, pat_file.weights);

    print_info(&mut board);

    for line in stdin.lock().lines() {
        // Get input from the player.
        if let Ok(text) = line {
            let split: Vec<&str> = text.split(' ').collect();

            if split[0] == "exit" {
                break;
            } else if split[0] == "play" {
                let m = Move::from_coord(split[1]);

                if board.get_moves().contains(m) {
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
                let (score, best_move, _) = if depth <= board.all_disks().count_zeros() as u8 {
                    // If the search is not full depth, then run a normal search.
                    if let Some(&alg) = split.get(2) {
                        match alg {
                            "nm" => negamax::negamax(&mut board, depth, &pat_eval, true),
                            "bns" => bns::best_node_search(&mut board, depth, &pat_eval),
                            _ => negamax::negamax(&mut board, depth, &pat_eval, true)
                        }
                    } else {
                        negamax::negamax(&mut board, depth, &pat_eval, true)
                    }
                } else {
                    // If the search will be full-depth, then just endgame solve.
                    endgame::endgame_solve(&mut board, false, true)
                };

                println!("Computer is playing {}, which had score {}.", best_move, score);
                // Make move and save undo information.
                let undo_info = board.make_move(best_move);
                undo_stack.push((undo_info, best_move));
            } else if split[0] == "gt" {
                // Get the search depth
                let time = match split.get(1) {
                    Some(dep) => match dep.parse::<u32>() { Ok(x) => x, _ => 1000 },
                    None => 1000
                };

                // Get the best move.
                let (score, best_move, _) = if let Some(&alg) = split.get(2) {
                    match alg {
                        "nm" => negamax::negamax_id(&mut board, time, &pat_eval, false),
                        "bns" => iterative::bns_iter_deep(&mut board, time, &pat_eval),
                        _ => negamax::negamax_id(&mut board, time, &pat_eval, true)
                    }
                } else {
                    negamax::negamax_id(&mut board, time, &pat_eval, true)
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
                println!("\n{}", board);
                break;
            }

            // Print info.
            print_info(&mut board);
        } else {
            break;
        }
    }
}

fn cs2_play(mut board: Board, black: bool) {
    let stdin = io::stdin();
    let mut first_move = true;
    let mut last_bf = 10.0;

    let file = File::open("pat9-12-fp.json").expect("File read error.");
    let reader = BufReader::new(file);
    let pat_file: PatternFile = from_reader(reader).expect("Unable to parse json");

    let pat_eval = PatternEvaluator::from(pat_file.masks, pat_file.weights);

    eprintln!("Initialized...");
    println!("");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        eprintln!("Line: {}", line);

        eprintln!(
            "\nRuthless: Making {} Move",
            if !board.black_move { "Dark" } else { "Light" }
        );

        let line_split: Vec<&str> = line.split(" ").collect();

        let x: i8 = str::parse::<i8>(line_split[0]).unwrap();
        let y: i8 = str::parse::<i8>(line_split[1]).unwrap();
        let ms_left: i64 = str::parse::<i64>(line_split[2]).unwrap();
        if x >= 0 && y >= 0 {
            let coord: u8 = (y * 8 + x) as u8;
            board.make_move(Move::Play(coord));
        } else if black && first_move {
            eprintln!("First move & black.");
        } else {
            board.make_move(Move::Pass);
        }

        let x: i32;
        let y: i32;
        let best_move;
        let best_score;
        let srch_data;

        let time_allocated = (2. / (52. - board.all_disks().count_ones() as f32).max(3.0) * ms_left as f32) as u32;

        eprintln!("Allocating {:.2} s to search.", time_allocated as f32 / 1000.0);

        if board.all_disks().count_zeros() > 18 && !(board.all_disks().count_zeros() < 20 && last_bf < 3.5) {
            let (score, best, data) = negamax::negamax_id(&mut board, time_allocated, &pat_eval, false);
            best_move = best;
            best_score = score;
            last_bf = (data.nodes as f32).powf(1.0 / data.depth as f32);
            srch_data = data;
        } else if board.all_disks().count_zeros() > 12 {
            let (score, best, data) = endgame::endgame_solve(&mut board, true, false);
            if score == -1 { // TODO: This is bad.
                let (score, best, data) = negamax::negamax_id(&mut board, time_allocated, &pat_eval, false);
                best_move = best;
                best_score = score;
                last_bf = (data.nodes as f32).powf(1.0 / data.depth as f32);
            } else {
                best_move = best;
                best_score = score;
                last_bf = 0.0;
            }
            srch_data = data;
        } else {
            last_bf = 0.0;
            let (score, best, data) = endgame::endgame_solve(&mut board, false, false);
            best_move = best;
            best_score = score;
            srch_data = data;
        }

        eprintln!("\nBest move was {} with score {}", best_move, best_score);

        match best_move {
            Move::Play(m) => {
                x = (m % 8) as i32;
                y = (m / 8) as i32;
            }
            Move::Pass => {
                x = -1;
                y = -1;
            }
        }

        board.make_move(best_move);
        first_move = false;

        eprintln!("");
        eprintln!("Searched {} nodes in {} ms ({:.2} kn/s). Final depth was {}.", srch_data.nodes, srch_data.time, srch_data.nodes as f32 / srch_data.time as f32, srch_data.depth);
        eprintln!("");
        println!("{} {}", x, y);
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
        let undo = board.make_move(m);
        nodes += perft_impl(depth - 1, board);
        board.undo_move(undo, m);
    }

    nodes
}

fn training_data_solve(empties: u8, num_pos: usize) -> Vec<Position> {
    // TODO: Make this a lot cleaner.
    let idxs: Vec<usize> = (0..num_pos).collect();
    let searcher: EndgameSearcher = EndgameSearcher::new(false);
    idxs.par_iter().map(|&_| random_solved(empties, &searcher)).collect()
}

fn random_solved(empties: u8, solver: &EndgameSearcher) -> Position {
    let mut rng = rand::thread_rng();
    'new_pos: loop {
        let mut board = Board::new();
        while board.all_disks().count_zeros() > empties.into() {
            let moves = board.get_moves();
            board.make_move(moves[rng.gen_range(0, moves.len())]);
            if board.is_game_over() {
                continue 'new_pos;
            }
        }
        let mut score = solver.endgame_solve(&mut board, false).0;
        if !board.black_move {
            score = -score;
        }
        let mut pos = board.get_position();
        pos.score = Some(score);
        return pos;
    }
}

fn training_data_heuristic(empties: u8, depth: u8, num_pos: usize) -> Vec<Position> {
    let idxs: Vec<usize> = (0..num_pos).collect();

    let file = File::open("pat9-12.json").expect("File read error.");
    let reader = BufReader::new(file);
    let pat_file: PatternFile = from_reader(reader).expect("Unable to parse json");

    let pat_eval = PatternEvaluator::from(pat_file.masks, pat_file.weights);

    idxs.par_iter().map(|&_| random_heuristic(empties, depth, &pat_eval)).collect()
}

fn random_heuristic(empties: u8, depth: u8, heuristic: &PatternEvaluator) -> Position {
    let mut rng = rand::thread_rng();
    'new_pos: loop {
        let mut board = Board::new();
        while board.all_disks().count_zeros() > empties.into() {
            let moves = board.get_moves();
            board.make_move(moves[rng.gen_range(0, moves.len())]);
            if board.is_game_over() {
                continue 'new_pos;
            }
        }
        let mut score = negamax::negamax(&mut board, depth, heuristic, false).0;
        if !board.black_move {
            score = -score;
        }
        let mut pos = board.get_position();
        pos.score = Some(score);
        return pos;
    }
}
