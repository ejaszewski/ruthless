use std::fmt;

pub mod bitboard;

#[cfg(test)]
pub mod test;

pub fn coord_to_bitmask(coord: String) -> Option<u64> {
    let mut chars = coord.chars();

    let mut pos = match chars.next() {
        Some(c) => match c.to_uppercase().next().unwrap() {
            'A' => bitboard::FILE_A,
            'B' => bitboard::FILE_B,
            'C' => bitboard::FILE_C,
            'D' => bitboard::FILE_D,
            'E' => bitboard::FILE_E,
            'F' => bitboard::FILE_F,
            'G' => bitboard::FILE_G,
            'H' => bitboard::FILE_H,
            _ => return None,
        },
        None => return None,
    };

    pos &= match chars.next() {
        Some(c) => match c.to_string().parse::<u8>() {
            Ok(r) => {
                if r >= 1 && r <= 8 {
                    bitboard::RANKS[r as usize - 1]
                } else {
                    return None;
                }
            }
            Err(_) => return None,
        },
        None => return None,
    };

    Some(pos)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Move {
    Play(u8),
    Pass
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let &Move::Play(play) = self {
            let letter = ["a", "b", "c", "d", "e", "f", "g", "h"][(play % 8) as usize];
            let number = play / 8 + 1;
            write!(f, "{}{}", letter, number)
        } else {
            write!(f, "PASS")
        }
    }
}

pub struct Board {
    pub white_disks: u64,
    white_moves: u64,
    white_moves_gen: bool,
    pub black_disks: u64,
    black_moves: u64,
    black_moves_gen: bool,
    pub black_move: bool,
}

impl Board {
    pub fn new() -> Board {
        let mut white_disks = 0;
        white_disks |= coord_to_bitmask(String::from("d4")).unwrap();
        white_disks |= coord_to_bitmask(String::from("e5")).unwrap();

        let mut black_disks = 0;
        black_disks |= coord_to_bitmask(String::from("e4")).unwrap();
        black_disks |= coord_to_bitmask(String::from("d5")).unwrap();

        let black_move = true;

        let mut board = Board {
            white_disks,
            black_disks,
            black_move,
            white_moves: 0,
            white_moves_gen: false,
            black_moves: 0,
            black_moves_gen: false,
        };
        board.gen_black_moves();

        board
    }

    pub fn get_black_moves(&mut self) -> u64 {
        if !self.black_moves_gen {
            self.gen_black_moves();
        }
        return self.black_moves;
    }

    pub fn get_white_moves(&mut self) -> u64 {
        if !self.white_moves_gen {
            self.gen_white_moves();
        }
        return self.white_moves;
    }

    fn gen_black_moves(&mut self) {
        self.black_moves = bitboard::all_moves(self.black_disks, self.white_disks);
        self.black_moves_gen = true;
    }

    fn gen_white_moves(&mut self) {
        self.white_moves = bitboard::all_moves(self.white_disks, self.black_disks);
        self.white_moves_gen = true;
    }

    pub fn get_moves(&mut self) -> Vec<Move> {
        let mut all_moves = if self.black_move {
            self.get_black_moves()
        } else {
            self.get_white_moves()
        };

        let num_moves = all_moves.count_ones() as usize;
        let mut moves: Vec<Move> = Vec::with_capacity(num_moves);

        if all_moves == 0 {
            moves.push(Move::Pass);
        } else {
            let mask = 0x80_00_00_00_00_00_00_00;
            for _i in 0..num_moves {
                let index = all_moves.leading_zeros();
                moves.push(Move::Play(index as u8));
                all_moves ^= mask >> index;
            }
        }

        moves
    }

    pub fn move_count(&mut self) -> u32 {
        if self.black_move {
            self.get_black_moves()
        } else {
            self.get_white_moves()
        }.count_ones()
    }

    pub fn moves_exist(&mut self) -> bool {
        (self.get_white_moves() | self.get_black_moves()) != 0
    }

    pub fn make_move(&mut self, move_option: Move) -> u64 {
        match move_option {
            Move::Play(m) => {
                let num_directions = bitboard::SHIFT_DIRS.len();
                let disk = 0x80_00_00_00_00_00_00_00 >> m;

                let (player, opponent) = if self.black_move {
                    self.black_disks |= disk;
                    (self.black_disks, self.white_disks)
                } else {
                    self.white_disks |= disk;
                    (self.white_disks, self.black_disks)
                };

                let mut flood = 0;
                for i in 0..num_directions {
                    let shift = bitboard::SHIFT_DIRS[i];
                    let prop = opponent & bitboard::SHIFT_MASKS[i] & bitboard::SHIFT_RAYS[m as usize][i];
                    let mut temp_flood = 0;

                    let mut gen = disk;
                    let mut next = gen;
                    while gen != 0 {
                        temp_flood |= gen;
                        next = bitboard::directional_shift(gen, shift);
                        gen = next & prop;
                    }

                    if next & player != 0 {
                        flood |= temp_flood ^ disk;
                    }
                }

                self.white_disks ^= flood;
                self.black_disks ^= flood;
                self.black_move = !self.black_move;

                self.black_moves_gen = false;
                self.white_moves_gen = false;

                flood
            }
            Move::Pass => {
                self.black_move = !self.black_move;
                self.black_moves_gen = false;
                self.white_moves_gen = false;
                0
            }
        }
    }

    pub fn undo_move(&mut self, undo: u64, move_option: Move) {
        match move_option {
            Move::Play(m) => {
                self.white_disks ^= undo;
                self.black_disks ^= undo;
                self.black_move = !self.black_move;

                let disk = 0x80_00_00_00_00_00_00_00 >> m;
                if self.black_move {
                    self.black_disks &= !disk;
                } else {
                    self.white_disks &= !disk;
                }

                self.black_moves_gen = false;
                self.white_moves_gen = false;
            }
            Move::Pass => {
                self.black_move = !self.black_move;

                self.black_moves_gen = false;
                self.white_moves_gen = false;
            }
        }
    }

    #[inline]
    pub fn all_disks(&self) -> u64 {
        self.white_disks | self.black_disks
    }

    pub fn is_game_over(&mut self) -> bool {
        self.black_disks == 0 || self.white_disks == 0 || !self.moves_exist()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    A   B   C   D   E   F   G   H  \n").unwrap();
        write!(f, "  ╔═══╤═══╤═══╤═══╤═══╤═══╤═══╤═══╗\n").unwrap();

        let disk_char = | r: usize, f: usize | {
            let disk = bitboard::RANKS[r] & bitboard::FILES[f];
            if self.black_disks & disk > 0 {
                "●"
            } else if self.white_disks & disk > 0 {
                "○"
            } else {
                " "
            }
        };

        for rank in 0..8 {
            write!(f, "{} ║ {} │ {} │ {} │ {} │ {} │ {} │ {} │ {} ║ {0}", rank + 1,
                   disk_char(rank, 0), disk_char(rank, 1),
                   disk_char(rank, 2), disk_char(rank, 3),
                   disk_char(rank, 4), disk_char(rank, 5),
                   disk_char(rank, 6), disk_char(rank, 7)).unwrap();

            if rank == 7 {
                write!(f, "\n  ╚═══╧═══╧═══╧═══╧═══╧═══╧═══╧═══╝\n").unwrap();
            } else {
                write!(f, "\n  ╟───┼───┼───┼───┼───┼───┼───┼───╢\n").unwrap();
            }
        }
        write!(f, "    A   B   C   D   E   F   G   H  ")
    }
}
