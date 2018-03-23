use super::*;

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
