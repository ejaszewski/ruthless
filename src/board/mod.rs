pub mod util;
pub mod constants;

use std::fmt;
use std::collections::HashSet;

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

        let dark_move = true;
        Board {
            light_disks, dark_disks, dark_move
        }
    }

    pub fn get_moves(&self) -> HashSet<Option<u8>> {
        let (player, opponent) =
            if self.dark_move {
                (self.dark_disks, self.light_disks)
            } else {
                (self.light_disks, self.dark_disks)
            };

        let mask = opponent & 0x7E_7E_7E_7E_7E_7E_7E_7E;

        let mut all_moves: u64 = 0;
        for shift in &constants::SHIFT_DIRS {
            let shift = *shift;
            if shift == 8 || shift == -8 {
                all_moves |= util::directional_moves(player, opponent, shift);
            } else {
                all_moves |= util::directional_moves(player, mask, shift);
            }
        }

        all_moves &= !self.all_disks();

        let mut moves: HashSet<Option<u8>> = HashSet::new();
        
        if all_moves == 0 {
            moves.insert(None);
        } else {
            let mut mask = 0x80_00_00_00_00_00_00_00;
            for i in 0..64 {
                if mask & all_moves != 0 {
                    moves.insert(Some(i));
                }
                mask >>= 1;
            }
        }

        moves
    }

    pub fn make_move(&mut self, move_option: Option<u8>) -> u64 {
        match move_option {
            Some(m) => {
                let num_directions = constants::SHIFT_DIRS.len();
                let disk = 0x80_00_00_00_00_00_00_00 >> m;

                let (player, opponent) = if self.dark_move {
                    self.dark_disks |= disk;
                    (self.dark_disks, self.light_disks)
                } else {
                    self.light_disks |= disk;
                    (self.light_disks, self.dark_disks)
                };

                let mut flood = 0;
                for i in 0..num_directions {
                    let shift = constants::SHIFT_DIRS[i];
                    let prop = opponent & constants::SHIFT_MASKS[i] & constants::MASKS[m as usize][i];
                    let mut temp_flood = 0;

                    let mut gen = disk;
                    let mut next = gen;
                    while gen != 0 {
                        temp_flood |= gen;
                        next = util::directional_shift(gen, shift);
                        gen = next & prop;
                    }

                    if next & player != 0 {
                        flood |= temp_flood ^ disk;
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

    pub fn undo_move(&mut self, undo: u64, move_option: Option<u8>) {
        match move_option {
            Some(m) => {
                self.light_disks ^= undo;
                self.dark_disks ^= undo;
                self.dark_move = !self.dark_move;

                let disk = 0x80_00_00_00_00_00_00_00 >> m;
                if self.dark_move {
                    self.dark_disks &= !disk;
                } else {
                    self.light_disks &= !disk;
                }
            },
            None => {
                self.dark_move = !self.dark_move;
            }
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
