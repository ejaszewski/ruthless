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

    println!("");

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line_split: Vec<&str> = line.split(" ").collect();

        let x: i8 = str::parse::<i8>(line_split[0]).unwrap();
        let y: i8 = str::parse::<i8>(line_split[1]).unwrap();
        if x >= 0 && y >= 0 {
            let coord: u8 = (y * 8 + x) as u8;
            board.make_move(coord);
        }

        // eprintln!("{}", board);

        let moves = board.get_moves();
        eprintln!("{:?}", moves);
        let x: i32;
        let y: i32;

        if moves.len() > 0 {
            x = (moves[0] % 8) as i32;
            y = (moves[0] / 8) as i32;
        } else {
            x = -1;
            y = -1;
        }

        if x >= 0 && y >= 0 {
            board.make_move(moves[0]);
        }
        // eprintln!("{}", board);

        println!("{} {}", x, y);
    }
}
