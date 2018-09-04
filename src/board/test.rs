/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::{coord_to_bitmask, Move, Board};

#[test]
fn test_coord_to_bitmask() {
    // Testing each file.
    assert_eq!(coord_to_bitmask(String::from("a1")), Some(0x80_00_00_00_00_00_00_00));
    assert_eq!(coord_to_bitmask(String::from("b2")), Some(0x00_40_00_00_00_00_00_00));
    assert_eq!(coord_to_bitmask(String::from("c3")), Some(0x00_00_20_00_00_00_00_00));
    assert_eq!(coord_to_bitmask(String::from("d4")), Some(0x00_00_00_10_00_00_00_00));
    assert_eq!(coord_to_bitmask(String::from("e5")), Some(0x00_00_00_00_08_00_00_00));
    assert_eq!(coord_to_bitmask(String::from("f6")), Some(0x00_00_00_00_00_04_00_00));
    assert_eq!(coord_to_bitmask(String::from("g7")), Some(0x00_00_00_00_00_00_02_00));
    assert_eq!(coord_to_bitmask(String::from("h8")), Some(0x00_00_00_00_00_00_00_01));

    // Testing all possible failure modes.
    assert_eq!(coord_to_bitmask(String::from("")), None);
    assert_eq!(coord_to_bitmask(String::from("j1")), None);
    assert_eq!(coord_to_bitmask(String::from("a9")), None);
    assert_eq!(coord_to_bitmask(String::from("ab")), None);
    assert_eq!(coord_to_bitmask(String::from("a")), None);
}

#[test]
fn test_coord_to_move() {
    // Testing each file.
    assert_eq!(Move::from_coord(String::from("a1")), Move::Play(0));
    assert_eq!(Move::from_coord(String::from("b2")), Move::Play(9));
    assert_eq!(Move::from_coord(String::from("c3")), Move::Play(18));
    assert_eq!(Move::from_coord(String::from("d4")), Move::Play(27));
    assert_eq!(Move::from_coord(String::from("e5")), Move::Play(36));
    assert_eq!(Move::from_coord(String::from("f6")), Move::Play(45));
    assert_eq!(Move::from_coord(String::from("g7")), Move::Play(54));
    assert_eq!(Move::from_coord(String::from("h8")), Move::Play(63));

    // Testing all possible failure modes.
    assert_eq!(Move::from_coord(String::from("")), Move::Pass);
    assert_eq!(Move::from_coord(String::from("j1")), Move::Pass);
    assert_eq!(Move::from_coord(String::from("a9")), Move::Pass);
    assert_eq!(Move::from_coord(String::from("ab")), Move::Pass);
    assert_eq!(Move::from_coord(String::from("a")), Move::Pass);
}

#[test]
fn test_move_count_after() {
    // This is a very minimal test case, since this gets called a lot by the FFO tests.
    let mut board = Board::from_pos(0x00FF000000000000, 0xFF00000000000000, true);
    assert_eq!(board.move_count_after(Move::Pass), 8);

    // This is a very minimal test case, since this gets called a lot by the FFO tests.
    let mut board = Board::from_pos(0xFF00000000000000, 0x00FF000000000000, false);
    assert_eq!(board.move_count_after(Move::Pass), 8);
}

#[test]
fn test_board_derives() {
    let mut board = Board::new();
    let clone = board.clone();

    assert_eq!(board, board);
    assert_eq!(board, clone);
    assert_eq!(clone, board);

    board.make_move(Move::from_coord(String::from("d3")));
    assert!(board != clone);
    assert!(board == board);
}

#[test]
fn test_move_derives() {
    // Testing derive(Debug)
    assert_eq!(format!("{:?}", Move::Play(0)), "Play(0)");
    assert_eq!(format!("{:?}", Move::Pass), "Pass");

    // Testing derive(PartialEq, Eq)
    assert!(Move::Play(0) == Move::Play(0));
    assert!(Move::Play(0) != Move::Play(1));
    assert!(Move::Pass == Move::Pass);
    assert!(Move::Pass != Move::Play(1));
}

#[test]
fn test_move_display() {
    // Testing a few possible moves and a pass, and making sure their Display is correct.
    assert_eq!(format!("{}", Move::Play(0)), "a1");
    assert_eq!(format!("{}", Move::Play(13)), "f2");
    assert_eq!(format!("{}", Move::Play(43)), "d6");
    assert_eq!(format!("{}", Move::Play(63)), "h8");
    assert_eq!(format!("{}", Move::Pass), "PASS");
}

#[test]
fn test_board_new() {
    let mut board = Board::new();

    // Make sure the starting disks are correct.
    assert_eq!(board.black_disks, 0x00_00_00_08_10_00_00_00);
    assert_eq!(board.white_disks, 0x00_00_00_10_08_00_00_00);

    // Make sure black's moves were generated correctly.
    let moves = vec![Move::Play(19), Move::Play(26), Move::Play(37), Move::Play(44)];
    assert_eq!(board.get_moves(), moves);
    assert_eq!(board.get_black_moves(), 0x00_00_10_20_04_08_00_00);
    assert_eq!(board.move_count(), 4);

    // Some additional sanity checking.
    assert!(board.moves_exist());
    assert!(!board.is_game_over());
}

#[test]
fn test_board_1() {
    let mut board = Board::new();

    // Make a move and ensure it applied correctly.
    let m = board.make_move(Move::Play(19));
    assert_eq!(board.black_disks, 0x00_00_10_18_10_00_00_00);
    assert_eq!(board.white_disks, 0x00_00_00_00_08_00_00_00);

    // Make sure that white's moves were generated correctly.
    let moves = vec![Move::Play(18), Move::Play(20), Move::Play(34)];
    assert_eq!(board.get_moves(), moves);
    assert_eq!(board.get_white_moves(), 0x00_00_28_00_20_00_00_00);
    assert_eq!(board.move_count(), 3);

    // Make sure that we can generate black's moves correctly.
    assert_eq!(board.get_black_moves(), 0x00_00_00_00_04_0C_00_00);

    // Some additional sanity checking.
    assert!(board.moves_exist());
    assert!(!board.is_game_over());

    // Undo the move and check everything.
    board.undo_move(m, Move::Play(19));
    assert_eq!(board.black_disks, 0x00_00_00_08_10_00_00_00);
    assert_eq!(board.white_disks, 0x00_00_00_10_08_00_00_00);
    assert_eq!(board.all_disks(), 0x00_00_00_18_18_00_00_00);
    assert_eq!(board.get_black_moves(), 0x00_00_10_20_04_08_00_00);
    assert_eq!(board.get_white_moves(), 0x00_00_08_04_20_10_00_00);
}

#[test]
fn test_board_2() {
    let mut board = Board::new();

    // Make a move and ensure it applied correctly.
    board.make_move(Move::Play(19));
    let m = board.make_move(Move::Play(18));
    assert_eq!(board.black_disks, 0x00_00_10_08_10_00_00_00);
    assert_eq!(board.white_disks, 0x00_00_20_10_08_00_00_00);
    assert_eq!(board.all_disks(), 0x00_00_30_18_18_00_00_00);

    board.undo_move(m, Move::Play(18));

    assert_eq!(board.black_disks, 0x00_00_10_18_10_00_00_00);
    assert_eq!(board.white_disks, 0x00_00_00_00_08_00_00_00);
    assert_eq!(board.get_black_moves(), 0x00_00_00_00_04_0C_00_00);
    assert_eq!(board.get_white_moves(), 0x00_00_28_00_20_00_00_00);

    // Some additional sanity checking.
    assert!(board.moves_exist());
    assert!(!board.is_game_over());
}

#[test]
fn test_board_3() {
    let mut board = Board::new();

    // Make a move and ensure it applied correctly.
    board.make_move(Move::Play(26));
    board.make_move(Move::Play(18));
    board.make_move(Move::Play(10));
    board.make_move(Move::Play(34));
    board.make_move(Move::Play(42));
    board.make_move(Move::Play(43));
    board.make_move(Move::Play(44));
    board.make_move(Move::Play(29));
    board.make_move(Move::Play(30));
    assert_eq!(board.black_disks, 0x00_20_20_3E_38_38_00_00);
    assert_eq!(board.white_disks, 0x00_00_00_00_00_00_00_00);
    assert_eq!(board.all_disks(), 0x00_20_20_3E_38_38_00_00);
    assert_eq!(board.get_black_moves(), 0x00_00_00_00_00_00_00_00);
    assert_eq!(board.get_white_moves(), 0x00_00_00_00_00_00_00_00);

    // Make sure we're at a loss condition & have no moves.
    assert_eq!(board.get_moves(), vec![Move::Pass]);
    assert!(!board.moves_exist());
    assert!(board.is_game_over());

    // Make & undo a Pass. State should remain the same.
    board.make_move(Move::Pass);
    assert_eq!(board.black_disks, 0x00_20_20_3E_38_38_00_00);
    assert_eq!(board.white_disks, 0x00_00_00_00_00_00_00_00);
    assert_eq!(board.get_black_moves(), 0x00_00_00_00_00_00_00_00);
    assert_eq!(board.get_white_moves(), 0x00_00_00_00_00_00_00_00);

    board.undo_move(0, Move::Pass);
    assert_eq!(board.black_disks, 0x00_20_20_3E_38_38_00_00);
    assert_eq!(board.white_disks, 0x00_00_00_00_00_00_00_00);
    assert_eq!(board.get_black_moves(), 0x00_00_00_00_00_00_00_00);
    assert_eq!(board.get_white_moves(), 0x00_00_00_00_00_00_00_00);
}

#[test]
fn test_board_display() {
    let board = Board::new();

    let display = "    A   B   C   D   E   F   G   H  \
\n  ╔═══╤═══╤═══╤═══╤═══╤═══╤═══╤═══╗
1 ║   │   │   │   │   │   │   │   ║ 1
  ╟───┼───┼───┼───┼───┼───┼───┼───╢
2 ║   │   │   │   │   │   │   │   ║ 2
  ╟───┼───┼───┼───┼───┼───┼───┼───╢
3 ║   │   │   │   │   │   │   │   ║ 3
  ╟───┼───┼───┼───┼───┼───┼───┼───╢
4 ║   │   │   │ ○ │ ● │   │   │   ║ 4
  ╟───┼───┼───┼───┼───┼───┼───┼───╢
5 ║   │   │   │ ● │ ○ │   │   │   ║ 5
  ╟───┼───┼───┼───┼───┼───┼───┼───╢
6 ║   │   │   │   │   │   │   │   ║ 6
  ╟───┼───┼───┼───┼───┼───┼───┼───╢
7 ║   │   │   │   │   │   │   │   ║ 7
  ╟───┼───┼───┼───┼───┼───┼───┼───╢
8 ║   │   │   │   │   │   │   │   ║ 8
  ╚═══╧═══╧═══╧═══╧═══╧═══╧═══╧═══╝
    A   B   C   D   E   F   G   H  ";

    // Make sure the starting disks are correct.
    assert_eq!(format!("{}", board), display);
}

#[test]
fn test_board_debug() {
    let board = Board::new();

    let debug = "  A B C D E F G H 	  A B C D E F G H \
\n1 - - - - - - - -	1 - - - - - - - -
2 - - - - - - - -	2 - - - - - - - -
3 - - - - - - - -	3 - - - - - - - -
4 - - - - # - - -	4 - - - # - - - -
5 - - - # - - - -	5 - - - - # - - -
6 - - - - - - - -	6 - - - - - - - -
7 - - - - - - - -	7 - - - - - - - -
8 - - - - - - - -	8 - - - - - - - -
       BLACK      	       WHITE      ";

    assert_eq!(format!("{:?}", board), debug);
}
