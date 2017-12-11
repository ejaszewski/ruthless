pub mod util;
pub mod constants;

use std::fmt;

pub struct Board {
    pub light_disks: u64,
    pub dark_disks: u64,
    pub dark_move: bool
}

impl Board {
    pub fn new() -> Board {
        let mut light_disks = 0;
        light_disks |= util::coord_to_bitmask(String::from("e5")).unwrap();
        light_disks |= util::coord_to_bitmask(String::from("d4")).unwrap();

        let mut dark_disks = 0;
        dark_disks |= util::coord_to_bitmask(String::from("e4")).unwrap();
        dark_disks |= util::coord_to_bitmask(String::from("d5")).unwrap();

        // dark_disks |= util::coord_to_bitmask(String::from("e5")).unwrap();
        // dark_disks |= util::coord_to_bitmask(String::from("e6")).unwrap();
        // dark_disks |= util::coord_to_bitmask(String::from("f5")).unwrap();
        // light_disks |= util::coord_to_bitmask(String::from("d5")).unwrap();
        // light_disks |= util::coord_to_bitmask(String::from("d6")).unwrap();

        let dark_move = true;
        Board {
            light_disks, dark_disks, dark_move
        }
    }

    #[inline]
    pub fn get_moves(&self) -> Vec<u8> {
        let mut moves: Vec<u8>;
        if self.dark_move {
            moves = self.get_dark_moves();
        } else {
            moves = self.get_light_moves();
        }
        moves.sort();
        moves.dedup();
        moves
    }

    fn get_light_moves(&self) -> Vec<u8> {
        let mut mask = 0x80_00_00_00_00_00_00_00;
        let mut moves: Vec<u8> = vec![];

        for i in 0..64 {
            if mask & self.light_disks != 0 {
                let num_directions = constants::SHIFT_DIRS.len();
                for j in 0..num_directions {
                    let shift = constants::SHIFT_DIRS[j];
                    let prop = self.dark_disks & constants::SHIFT_MASKS[j] & constants::MASKS[i][j];
                    let mut gen = util::directional_shift(mask, shift) & prop;
                    let mut next = util::directional_shift(gen, shift);
                    while next & prop != 0 {
                        gen = next & prop;
                        next = util::directional_shift(gen, shift);
                    }
                    if next != 0 && next & self.all_disks() == 0 {
                        moves.push(next.leading_zeros() as u8);
                    }
                }
            }
            mask >>= 1;
        }

        moves
    }

    fn get_dark_moves(&self) -> Vec<u8> {
        let mut mask = 0x80_00_00_00_00_00_00_00;
        let mut moves: Vec<u8> = vec![];

        for i in 0..64 {
            if mask & self.dark_disks != 0 {
                let num_directions = constants::SHIFT_DIRS.len();
                for j in 0..num_directions {
                    let shift = constants::SHIFT_DIRS[j];
                    let prop = self.light_disks & constants::SHIFT_MASKS[j] & constants::MASKS[i][j];
                    let mut gen = util::directional_shift(mask, shift) & prop;
                    let mut next = util::directional_shift(gen, shift);
                    while next & prop != 0 {
                        gen = next & prop;
                        next = util::directional_shift(gen, shift);
                    }
                    if next != 0 && next & self.all_disks() == 0 {
                        moves.push(next.leading_zeros() as u8);
                    }
                }
            }
            mask >>= 1;
        }

        moves
    }

    pub fn make_move(&mut self, move_option: Option<u8>) -> u64 {
        match move_option {
            Some(m) => {
                let num_directions = constants::SHIFT_DIRS.len();
                let disk = 0x80_00_00_00_00_00_00_00 >> m;
                if self.dark_move {
                    self.dark_disks |= disk;
                } else {
                    self.light_disks |= disk;
                }

                let mut flood = 0;
                for i in 0..num_directions {
                    let shift = constants::SHIFT_DIRS[i];
                    let prop = if self.dark_move { self.light_disks } else { self.dark_disks } & constants::SHIFT_MASKS[i] & constants::MASKS[m as usize][i];
                    let mut temp_flood = 0;

                    let mut gen = disk;
                    let mut next = gen;
                    while gen != 0 {
                        temp_flood |= gen;
                        next = util::directional_shift(gen, shift);
                        gen = next & prop;
                    }

                    if self.dark_move {
                        if next & self.dark_disks != 0 {
                            flood |= temp_flood ^ disk;
                        }
                    } else {
                        if next & self.light_disks != 0 {
                            flood |= temp_flood ^ disk;
                        }
                    }
                }

                self.light_disks ^= flood;
                self.dark_disks ^= flood;
                self.dark_move = !self.dark_move;

                flood
            },
            None => {
                self.dark_move = !self.dark_move;
                0
            }
        }
    }

    pub fn undo_move(&mut self, undo: u64, m: u8) {
        self.light_disks ^= undo;
        self.dark_disks ^= undo;
        self.dark_move = !self.dark_move;

        let disk = 0x80_00_00_00_00_00_00_00 >> m;
        if self.dark_move {
            self.dark_disks &= !disk;
        } else {
            self.light_disks &= !disk;
        }
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
