extern crate ruthless;
use ruthless::board;

fn main() {
    let mut b = board::Board::new();
    println!("{}", b);

    let moves: Vec<u8> = b.get_moves();
    println!("{:?}", moves);

    b.make_move(*moves.get(0).unwrap());
    println!("{}", b);
}
