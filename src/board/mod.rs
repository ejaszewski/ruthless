pub mod util;
pub mod constants;

use std::fmt;

pub struct Board {
    light_disks: u64,
    dark_disks: u64,
    dark_move: bool
}

impl Board {
    pub fn new() -> Board {
        let mut light_disks = 0;
        // light_disks |= util::coord_to_bitmask(String::from("e5")).unwrap();
        light_disks |= util::coord_to_bitmask(String::from("d4")).unwrap();

        let mut dark_disks = 0;
        dark_disks |= util::coord_to_bitmask(String::from("e4")).unwrap();
        // dark_disks |= util::coord_to_bitmask(String::from("d5")).unwrap();

        dark_disks |= util::coord_to_bitmask(String::from("e5")).unwrap();
        dark_disks |= util::coord_to_bitmask(String::from("f5")).unwrap();
        light_disks |= util::coord_to_bitmask(String::from("d5")).unwrap();
        light_disks |= util::coord_to_bitmask(String::from("d6")).unwrap();

        let dark_move = false;
        Board {
            light_disks, dark_disks, dark_move
        }
    }

    pub fn get_light_moves(self) -> Vec<u8> {
        let mut mask = 0x80_00_00_00_00_00_00_00;
        let mut moves: Vec<u8> = vec![];

        for i in 0..64 {
            if mask & self.light_disks != 0 {
                let num_directions = constants::SHIFT_DIRS.len();
                for j in 0..num_directions {
                    let shift = constants::SHIFT_DIRS[j];
                    let mut dark_adjacent = self.dark_disks & constants::SHIFT_MASKS[j];
                    if shift < 0 {
                        dark_adjacent <<= -shift;
                    } else {
                        dark_adjacent >>= shift;
                    }
                    let light_ray = constants::MASKS[i][j];
                    let new_move = light_ray & dark_adjacent & !self.dark_disks;
                    if new_move != 0 {
                        moves.push(new_move.leading_zeros() as u8);
                    }
                }
            }
            mask >>= 1;
        }

        moves
    }

    pub fn get_dark_moves(&mut self) -> Vec<u8> {
        let mut mask = 0x80_00_00_00_00_00_00_00;
        let mut moves: Vec<u8> = vec![];

        let mut all_moves = 0;

        for i in 0..64 {
            if mask & self.dark_disks != 0 {
                let num_directions = constants::SHIFT_DIRS.len();
                for j in 0..num_directions {
                    let shift = constants::SHIFT_DIRS[j];
                    let mut light_adjacent = self.light_disks & constants::SHIFT_MASKS[j];
                    if shift < 0 {
                        light_adjacent <<= -shift;
                    } else {
                        light_adjacent >>= shift;
                    }
                    let dark_ray = constants::MASKS[i][j];
                    let new_move = dark_ray & light_adjacent & !self.light_disks;
                    if new_move != 0 {
                        moves.push(new_move.leading_zeros() as u8);
                    }
                }
            }
            mask >>= 1;
        }

        moves
    }

    #[inline]
    pub fn all_disks(&self) -> u64 {
        self.light_disks | self.dark_disks
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  │ A │ B │ C │ D │ E │ F │ G │ H │\n").unwrap();
        write!(f, "──┼───┼───┼───┼───┼───┼───┼───┼───┼──\n").unwrap();
        for shift in 0..64 {
            let row = shift / 8 + 1;
            if shift % 8 == 0 {
                write!(f, "{} │", row).unwrap();
            }
            let mask: u64 = 0x8000000000000000 >> shift;
            if mask & self.light_disks != 0 {
                write!(f, " 0 │").unwrap();
            } else if mask & self.dark_disks != 0 {
                write!(f, " # │").unwrap();
            } else {
                write!(f, "   │").unwrap();
            }
            if shift % 8 == 7 {
                write!(f, " {}\n──┼───┼───┼───┼───┼───┼───┼───┼───┼──\n", row).unwrap();
            }
        }
        write!(f, "  │ A │ B │ C │ D │ E │ F │ G │ H │")
    }
}
