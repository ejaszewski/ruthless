extern crate ruthless;

use std::io;
use std::io::BufRead;
use std::str;
use ruthless::board;

fn main() {
    let board = board::Board::new();
    play_stdin(board);
}

fn play_stdin(mut board: board::Board) {
    let stdin = io::stdin();
    let mut first = true;

    eprintln!("Initialized...");
    println!("");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        eprintln!("{}", line);

        let line_split: Vec<&str> = line.split(" ").collect();

        let x: i8 = str::parse::<i8>(line_split[0]).unwrap();
        let y: i8 = str::parse::<i8>(line_split[1]).unwrap();
        if x >= 0 && y >= 0 {
            let coord: u8 = (y * 8 + x) as u8;
            board.make_move(Some(coord));
        } else if first {
            first = false;
        } else {
            board.make_move(None);
        }

        let moves = board.get_moves();
        eprintln!("{:?}", moves);
        let x: i32;
        let y: i32;
        let best_move = ruthless::eval::do_search(&mut board);

        match best_move {
            Some(m) => {
                x = (m % 8) as i32;
                y = (m / 8) as i32;
            },
            None => {
                x = -1;
                y = -1;
            }
        }

        board.make_move(best_move);

        println!("{} {}", x, y);
    }
}
