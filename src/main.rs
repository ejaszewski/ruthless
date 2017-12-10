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

        let x: u8 = str::parse::<u8>(line_split[0]).unwrap();
        let y: u8 = str::parse::<u8>(line_split[1]).unwrap();
        let coord = y * 8 + x;
        board.make_move(coord);

        eprintln!("{}", board);

        let moves = board.get_moves();
        let x: u8 = (moves[0] % 8) as u8;
        let y: u8 = (moves[0] / 8) as u8;

        board.make_move(moves[0]);
        eprintln!("{}", board);

        println!("{} {}", x, y);
    }
}
