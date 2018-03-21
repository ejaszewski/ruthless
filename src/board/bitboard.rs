//! Contains a number of utility functions and constants for bitboard operations.

/// The 'A' file of the board.
pub const FILE_A: u64 = 0x80_80_80_80_80_80_80_80;
/// The 'B' file of the board.
pub const FILE_B: u64 = 0x40_40_40_40_40_40_40_40;
/// The 'C' file of the board.
pub const FILE_C: u64 = 0x20_20_20_20_20_20_20_20;
/// The 'D' file of the board.
pub const FILE_D: u64 = 0x10_10_10_10_10_10_10_10;
/// The 'E' file of the board.
pub const FILE_E: u64 = 0x08_08_08_08_08_08_08_08;
/// The 'F' file of the board.
pub const FILE_F: u64 = 0x04_04_04_04_04_04_04_04;
/// The 'G' file of the board.
pub const FILE_G: u64 = 0x02_02_02_02_02_02_02_02;
/// The 'H' file of the board.
pub const FILE_H: u64 = 0x01_01_01_01_01_01_01_01;
/// An array containing all 8 board file masks in order.
pub const FILES: [u64; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H
];

/// Rank 1 of the board.
pub const RANK_1: u64 = 0x00_00_00_00_00_00_00_FF;
/// Rank 2 of the board.
pub const RANK_2: u64 = 0x00_00_00_00_00_00_FF_00;
/// Rank 3 of the board.
pub const RANK_3: u64 = 0x00_00_00_00_00_FF_00_00;
/// Rank 4 of the board.
pub const RANK_4: u64 = 0x00_00_00_00_FF_00_00_00;
/// Rank 5 of the board.
pub const RANK_5: u64 = 0x00_00_00_FF_00_00_00_00;
/// Rank 6 of the board.
pub const RANK_6: u64 = 0x00_00_FF_00_00_00_00_00;
/// Rank 7 of the board.
pub const RANK_7: u64 = 0x00_FF_00_00_00_00_00_00;
/// Rank 8 of the board.
pub const RANK_8: u64 = 0xFF_00_00_00_00_00_00_00;
/// An array containing all 8 board rank masks in order.
pub const RANKS: [u64; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8
];

/// An array containing the 8 directions and their shifts.
/// The order is up, down, left, right, up-left, up-right, down-left, down-right.
pub const SHIFT_DIRS: [i8; 8] = [-8, 8, -1, 1, -9, -7, 7, 9];

/// An array containing the 8 directions and their shift masks, used to avoid overflow or wrapping.
/// The order is up, down, left, right, up-left, up-right, down-left, down-right.
pub const SHIFT_MASKS: [u64; 8] = [
    !RANK_8,
    !RANK_1,
    !FILE_A,
    !FILE_H,
    !(RANK_8 | FILE_A),
    !(RANK_8 | FILE_H),
    !(RANK_1 | FILE_A),
    !(RANK_1 | FILE_H),
];

/// A function which allows shifting a negative number of spots to the right.
/// # Arguments
/// * `x`: The number to shift.
/// * `shift`: The size of the shift, positive for right and negative for left.
pub fn directional_shift(x: u64, shift: i8) -> u64 {
    if shift < 0 {
        x << -shift
    } else {
        x >> shift
    }
}

/// A function which generates the moves in the specified direction using the given mask.
/// # Arguments:
/// * `player`: The bitboard representing the player's disks.
/// * `mask`: A bitmask containing the opponent's disks, excluding the wrapping locations.
/// * `dir`: The direction in which to shift (from SHIFT_DIRS).
pub fn directional_moves(player: u64, mask: u64, dir: i8) -> u64 {
    let mask_2 = mask & directional_shift(mask, dir);
    let mask_4 = mask_2 & directional_shift(mask_2, 2 * dir);

    let mut flip = player;
    flip |= mask & directional_shift(flip, dir);
    flip |= mask_2 & directional_shift(flip, dir * 2);
    flip |= mask_4 & directional_shift(flip, dir * 4);

    directional_shift(flip & mask, dir)
}

/// A function which generates all moves a given player can make against the given opponent.
/// # Arguments:
/// * `player`: The bitboard representing the player's disks.
/// * `opponent`: The bitboard representing the opponent's disks.
pub fn all_moves(player: u64, opponent: u64) -> u64 {
    let mut all_moves: u64 = 0;

    for i in 0..SHIFT_DIRS.len() {
        let shift = SHIFT_DIRS[i];
        let mask = SHIFT_MASKS[i];
        all_moves |= directional_moves(player, opponent & mask, shift);
    }

    all_moves & !(player | opponent)
}

mod test {
    #[test]
    fn test_directional_shift() {
        let x = 0x00_00_00_13_57_00_00_00;

        assert_eq!(super::directional_shift(x,  1), x >> 1);
        assert_eq!(super::directional_shift(x, -1), x << 1);
    }
}
