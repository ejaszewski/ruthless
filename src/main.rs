extern crate ruthless;
use ruthless::board;

fn main() {
    let mut b = board::Board::new();
    println!("{}", b);
    println!("{:?}", b.get_dark_moves());
}
