pub mod util;
pub mod constants;

use std::fmt;

pub struct Board {
    light_disks: u64,
    dark_disks: u64,
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
        if self.dark_move {
            self.get_dark_moves()
        } else {
            self.get_light_moves()
        }
    }

    fn get_light_moves(&self) -> Vec<u8> {
        let mut mask = 0x80_00_00_00_00_00_00_00;
        let mut moves: Vec<u8> = vec![];

        for i in 0..64 {
            if mask & self.light_disks != 0 {
                let num_directions = constants::SHIFT_DIRS.len();
                for j in 0..num_directions {
                    let shift = constants::SHIFT_DIRS[j];

                    let mut dark_adjacent = self.dark_disks & constants::SHIFT_MASKS[j];
                    dark_adjacent = util::directional_shift(dark_adjacent, shift);

                    let light_ray = constants::MASKS[i][j];
                    let new_move = light_ray & (light_ray ^ (dark_adjacent ^ !self.all_disks()));
                    eprintln!("AAAAAAAAHHHH");
                    if new_move.count_ones() == 1 {
                        moves.push(new_move.leading_zeros() as u8);
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

                    let mut light_adjacent = self.light_disks & constants::SHIFT_MASKS[j];
                    light_adjacent = util::directional_shift(light_adjacent, shift);

                    let dark_ray = constants::MASKS[i][j];
                    let new_move = dark_ray & light_adjacent & !self.all_disks();

                    if new_move.count_ones() == 1 {
                        moves.push(new_move.leading_zeros() as u8);
                        if moves.len() == 1 {
                            eprintln!("{} {} {} {} {}", mask, dark_ray, light_adjacent, !self.all_disks(), new_move)
                        }
                    }
                }
            }
            mask >>= 1;
        }

        moves
    }

    pub fn make_move(&mut self, m: u8) {
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
            let prop = if self.dark_move { self.light_disks } else { self.dark_disks } & constants::SHIFT_MASKS[i];
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
