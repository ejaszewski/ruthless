#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate time;
extern crate serde_json;
extern crate ruthless;

use std::fs::File;
use std::io;
use std::io::Read;
use std::io::BufRead;
use std::io::prelude::*;
use std::str;
use clap::App;
use ruthless::{board, eval};
use ruthless::eval::properties;
use serde_json::Error;

#[derive(Serialize, Deserialize)]
struct PosgenOut {
    pub num_positions: usize,
    pub positions: Vec<board::TrainPosition>
}

impl PosgenOut {
    pub fn from_json(json: &str) -> Option<PosgenOut> {
        let props: Result<PosgenOut, Error> = serde_json::from_str(json);
        match props {
            Ok(properties) => Some(properties),
            Err(_) => None
        }
    }
}

fn main() {
    let cli_yaml = load_yaml!("cli_spec.yml");
    let matches = App::from_yaml(cli_yaml).get_matches();

    if matches.is_present("perft") {
        let depth = str::parse::<u64>(matches.value_of("depth").unwrap()).unwrap_or(1);
        run_perft(depth);
    } else if matches.is_present("color") {
        let props_file = matches.value_of("props").unwrap();
        let mut props_file = File::open(props_file).unwrap();
        let mut props_json = String::new();
        let pr = props_file.read_to_string(&mut props_json);
        match pr {
            Ok(_) => {}
            Err(_) => {}
        }

        let board = board::Board::new();
        let black = matches.value_of("color").unwrap() == "Black";
        let props = properties::Properties::from_json(props_json.as_str()).expect("Invalid JSON file.");
        eprintln!("{:?}", props);
        play_stdin(board, props, black);
    } else if matches.is_present("posgen") {
        match matches.values_of("posgen") {
           Some(mut posgen) => {
                let num_positions = str::parse::<usize>(posgen.next().unwrap()).unwrap_or(0);
                let num_random = str::parse::<u8>(posgen.next().unwrap()).unwrap_or(0);
                let output_file = posgen.next().unwrap_or("out.json");

                let mut output = PosgenOut {
                    num_positions,
                    positions: Vec::new()
                };

                for i in 0..num_positions {
                    output.positions.push(get_random_position(num_random));
                    if i % 1000 == 0 && i > 0 {
                        println!("Generated {} random positions.", i);
                    }
                }

                println!("Serializing position data...");
                let json_out = serde_json::to_string(&output).unwrap_or(String::new());
                println!("Writing output to file...");
                let mut file = File::create(output_file).unwrap();

                file.write_all(json_out.as_bytes()).expect("Unable to write to output file.");
                println!("Done.");
           },
           None => {}
       }
   } else if matches.is_present("fullsolve") {
        match matches.values_of("fullsolve") {
            Some(mut fullsolve) => {
                let pos_file = fullsolve.next().unwrap();
                let mut pos_file = File::open(pos_file).unwrap();
                let mut pos_json = String::new();

                let pr = pos_file.read_to_string(&mut pos_json);
                match pr {
                    Ok(_) => {}
                    Err(_) => {}
                }

                let mut pgo = PosgenOut::from_json(pos_json.as_str()).unwrap();

                for i in 0 .. pgo.num_positions {
                    let mut pos = &mut pgo.positions[i];
                    let mut board = board::Board::from_train_pos(pos);
                    let mut score = eval::negamax::negamax_endgame_full(&mut board, -64, 64).0 as f32;
                    if !board.dark_move {
                        score = -score;
                    }
                    pos.score = score;
                    if i % 1000 == 0 && i > 0 {
                        println!("Solved {} random positions.", i);
                    }
                }

                let output_file = fullsolve.next().unwrap_or("out.json");

                println!("Serializing position data...");
                let json_out = serde_json::to_string(&pgo).unwrap_or(String::new());
                println!("Writing output to file...");
                let mut file = File::create(output_file).unwrap();

                file.write_all(json_out.as_bytes()).expect("Unable to write to output file.");
                println!("Done.");
            },
            None => {}
        }
    } else {
        App::from_yaml(cli_yaml).print_help().unwrap();
        println!();
    }
}

fn play_stdin(mut board: board::Board, properties: properties::Properties, black: bool) {
    let stdin = io::stdin();
    let mut first_move = true;
    let mut last_bf = 10.0;

    eprintln!("Initialized...");
    println!("");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        eprintln!("Line: {}", line);

        eprintln!(
            "\nRuthless: Making {} Move",
            if !board.dark_move { "Dark" } else { "Light" }
        );

        let line_split: Vec<&str> = line.split(" ").collect();

        let x: i8 = str::parse::<i8>(line_split[0]).unwrap();
        let y: i8 = str::parse::<i8>(line_split[1]).unwrap();
        let ms_left: i64 = str::parse::<i64>(line_split[2]).unwrap();
        if x >= 0 && y >= 0 {
            let coord: u8 = (y * 8 + x) as u8;
            board.make_move(Some(coord));
        } else if black && first_move {
            eprintln!("First move & black.");
        } else {
            board.make_move(None);
        }

        let x: i32;
        let y: i32;
        let best_move;

        let time_allocated = 1. / (64. - board.all_disks().count_ones() as f32) * ms_left as f32;

        if board.all_disks().count_zeros() > 18 && !(board.all_disks().count_zeros() < 24 && last_bf < 3.5) {
            let (best, bf) = ruthless::eval::do_search(&mut board, &properties, time_allocated);
            best_move = best;
            last_bf = bf;
        } else if board.all_disks().count_zeros() > 12 {
            let (best, score) = ruthless::eval::search::endgame_solve_fast(&mut board);
            if score == -1 {
                let (best, bf) = ruthless::eval::do_search(&mut board, &properties, time_allocated);
                best_move = best;
                last_bf = bf;
            } else {
                best_move = best;
                last_bf = 0.0;
            }
        } else {
            last_bf = 0.0;
            best_move = ruthless::eval::search::endgame_solve_full(&mut board).0;
        }

        match best_move {
            Some(m) => {
                x = (m % 8) as i32;
                y = (m / 8) as i32;
            }
            None => {
                x = -1;
                y = -1;
            }
        }

        board.make_move(best_move);
        first_move = false;

        eprintln!("");
        println!("{} {}", x, y);
    }
}

fn get_random_position(num_random: u8) -> board::TrainPosition {
    let mut board = board::Board::new();

    for _ in 0..num_random {
        let moves = board.get_moves();
        board.make_move(moves[rand::random::<usize>() % moves.len()]);
    }

    return board.get_train_position();
}

fn run_perft(depth: u64) {
    println!("Running perft test at depth {}.", depth);
    let mut board = board::Board::new();

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

fn perft_impl(depth: u64, board: &mut board::Board) -> u64 {
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
