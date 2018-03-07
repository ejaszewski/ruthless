extern crate clap;
extern crate rand;
extern crate time;
extern crate serde_json;
extern crate ruthless;

use std::fs::File;
use std::io;
use std::io::Read;
use std::io::BufRead;
use std::str;
use clap::App;
use ruthless::{board};
use ruthless::eval::properties;

fn main() {
    let matches = App::new("Rusty")
                          .version("1.0")
                          .author("Ethan Jaszewski")
                          .about("A stripped down version of ruthless.")
                          .args_from_usage(
                              "<COLOR> 'Tells Rusty what color to play.'")
                          .get_matches();

    let mut props_file = File::open("eval.json").unwrap();
    let mut props_json = String::new();
    let pr = props_file.read_to_string(&mut props_json);
    match pr {
        Ok(_) => {}
        Err(_) => {}
    }

    let board = board::Board::new();
    let black = matches.value_of("COLOR").unwrap() == "Black";
    let props = properties::Properties::from_json(props_json.as_str()).expect("Invalid JSON file.");
    eprintln!("{:?}", props);

    play_stdin(board, props, black);
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

        if board.all_disks().count_zeros() > 18 && !(board.all_disks().count_zeros() < 24 && last_bf < 4.0) {
            let (best, bf) = ruthless::eval::do_search(&mut board, &properties);
            best_move = best;
            last_bf = bf;
        } else if board.all_disks().count_zeros() > 12 {
            let (best, score) = ruthless::eval::search::endgame_solve_fast(&mut board);
            if score == -1 {
                board.clear_moves();
                let (best, bf) = ruthless::eval::do_search(&mut board, &properties);
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
