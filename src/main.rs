#[macro_use]
extern crate clap;
extern crate time;

extern crate ruthless;

use std::io;
use std::io::BufRead;
use std::str;
use clap::App;
use ruthless::board;
use ruthless::eval::properties;

fn main() {
    let cli_yaml = load_yaml!("cli_spec.yml");
    let matches = App::from_yaml(cli_yaml).get_matches();
    let eval_properties = properties::Properties::from_args(&matches);

    if matches.is_present("perft") {
        let depth = str::parse::<u64>(matches.value_of("depth").unwrap()).unwrap_or(1);
        run_perft(depth);
    } else {
        let board = board::Board::new();
        let black = matches.value_of("color").unwrap() == "Black";
        play_stdin(board, eval_properties, black);
    }
}

fn play_stdin(mut board: board::Board, properties: properties::Properties, black: bool) {
    let stdin = io::stdin();
    let mut first_move = true;

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
        if board.all_disks().count_zeros() > 18 {
            best_move = ruthless::eval::do_search(&mut board, &properties);
        } else {
            best_move = ruthless::eval::endgame_solve(&mut board);
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
