use board::{ Board, Move };
use search::endgame::endgame_solve;

fn move_in_list(moves: Vec<u8>, m: Move) -> bool {
    moves.iter().any(|&x| Move::Play(x) == m)
}

#[test]
fn ffo_pos_40_wld() {
    let mut board = Board::from_pos(0x0101312303010100, 0x9E7ECEDCFC1E0800, true);

    assert_eq!(endgame_solve(&mut board, true).0, 1);
}

#[test]
fn ffo_pos_40_exact() {
    let mut board = Board::from_pos(0x0101312303010100, 0x9E7ECEDCFC1E0800, true);

    let (score, m) = endgame_solve(&mut board, false);
    assert_eq!(score, 38);
    assert!(move_in_list(vec![ 8 ], m));
}

#[test]
fn ffo_pos_41_wld() {
    let mut board = Board::from_pos(0x000200F8642C1800, 0x7C3C7E0618D02472, true);

    assert_eq!(endgame_solve(&mut board, true).0, 0);
}

#[test]
#[ignore]
fn ffo_pos_41_exact() {
    let mut board = Board::from_pos(0x000200F8642C1800, 0x7C3C7E0618D02472, true);

    let (score, m) = endgame_solve(&mut board, false);
    assert_eq!(score, 0);
    assert!(move_in_list(vec![ 31 ], m));
}

#[test]
fn ffo_pos_42_wld() {
    let mut board = Board::from_pos(0x000C040486040200, 0x3801FB7B391B1D3C, true);

    assert_eq!(endgame_solve(&mut board, true).0, 1);
}

#[test]
#[ignore]
fn ffo_pos_42_exact() {
    let mut board = Board::from_pos(0x000C040486040200, 0x3801FB7B391B1D3C, true);

    let (score, m) = endgame_solve(&mut board, false);
    assert_eq!(score, 6);
    assert!(move_in_list(vec![ 14 ], m));
}

#[test]
fn ffo_pos_43_wld() {
    let mut board = Board::from_pos(0x3E3C0C1E1C08143E, 0x0000706062F60800, false);

    assert_eq!(endgame_solve(&mut board, true).0, -1);
}

#[test]
#[ignore]
fn ffo_pos_43_exact() {
    let mut board = Board::from_pos(0x3E3C0C1E1C08143E, 0x0000706062F60800, false);

    let (score, m) = endgame_solve(&mut board, false);
    assert_eq!(score, -12);
    assert!(move_in_list(vec![ 50, 22 ], m));
}

#[test]
fn ffo_pos_44_wld() {
    let mut board = Board::from_pos(0x08081C0E0CC83C1C, 0x222563F1F0340000, false);

    assert_eq!(endgame_solve(&mut board, true).0, -1);
}

#[test]
#[ignore]
fn ffo_pos_44_exact() {
    let mut board = Board::from_pos(0x08081C0E0CC83C1C, 0x222563F1F0340000, false);

    let (score, m) = endgame_solve(&mut board, false);
    assert_eq!(score, -14);
    assert!(move_in_list(vec![ 57, 11 ], m));
}

#[test]
#[ignore]
fn ffo_pos_46_wld() {
    let mut board = Board::from_pos(0x1C04060703173078, 0x003838783C280C02, true);

    assert_eq!(endgame_solve(&mut board, true).0, -1);
}

#[test]
#[ignore]
fn ffo_pos_46_exact() {
    let mut board = Board::from_pos(0x1C04060703173078, 0x003838783C280C02, true);

    let (score, m) = endgame_solve(&mut board, false);
    assert_eq!(score, -8);
    assert!(move_in_list(vec![ 17 ], m));
}
