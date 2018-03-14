extern crate clap;
extern crate rand;
extern crate time;
extern crate serde_json;
extern crate ruthless;

use std::io;
use std::io::BufRead;
use std::str;
use clap::App;
use ruthless::{board};
use ruthless::eval::properties;

static PROPERTIES: &'static str = include_str!("../eval_new_new.json");

fn main() {
    let matches = App::new("Rusty")
                          .version("1.0")
                          .author("Ethan Jaszewski")
                          .about("A stripped down version of ruthless.")
                          .args_from_usage(
                              "<COLOR> 'Tells Rusty what color to play.'")
                          .get_matches();

    let board = board::Board::new();
    let black = matches.value_of("COLOR").unwrap() == "Black";
    let props = properties::Properties::from_json(PROPERTIES).expect("Invalid JSON file.");
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

        let time_allocated = 2. / (52. - board.all_disks().count_ones() as f32).max(3.0) * ms_left as f32;

        if board.all_disks().count_zeros() > 18 && !(board.all_disks().count_zeros() < 24 && last_bf < 3.5) {
            let (best, bf) = ruthless::eval::do_search(&mut board, &properties, time_allocated);
            best_move = best;
            last_bf = bf;
        } else if board.all_disks().count_zeros() > 12 {
            let (best, score) = ruthless::eval::search::endgame_solve_fast(&mut board);
            if score == -1 {
                board.clear_moves();
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
