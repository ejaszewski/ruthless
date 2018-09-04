/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
 
use std::fmt;

pub mod bitboard;

#[cfg(test)]
pub mod test;

/// A function which generates a bitmask corresponding to the given string coordinate.
/// # Arguments:
/// * `coord`: String coordinate of a square on the board.
/// # Returns:
/// * A bitmask representing the coordinate, or None if the coordinate was invalid.
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

impl Move {
    /// A function which generates a bitmask corresponding to the given string coordinate.
    /// # Arguments:
    /// * `coord`: String coordinate of a square on the board.
    /// # Returns:
    /// * A Move, Play if the coord is valid, Pass otherwise.
    pub fn from_coord(coord: String) -> Move {
        let mut chars = coord.chars();

        let file = match chars.next() {
            Some(c) => match c.to_uppercase().next().unwrap() {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                'D' => 3,
                'E' => 4,
                'F' => 5,
                'G' => 6,
                'H' => 7,
                _ => return Move::Pass,
            },
            None => return Move::Pass,
        };

        let rank = match chars.next() {
            Some(c) => match c.to_string().parse::<u8>() {
                Ok(r) => {
                    if r >= 1 && r <= 8 {
                        r - 1
                    } else {
                        return Move::Pass;
                    }
                }
                Err(_) => return Move::Pass,
            },
            None => return Move::Pass,
        };

        Move::Play(file + rank * 8)
    }
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

#[derive(Clone, PartialEq, Eq)]
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
    /// Creates a new board in the starting position, with white disks on d4 and e5 and black disks
    /// on e4 and d5.
    /// # Returns:
    /// * A mask of the disks flipped when the given move is made.
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

    pub fn from_pos(black_disks: u64, white_disks: u64, black_move: bool) -> Board {
        Board {
            white_disks,
            black_disks,
            black_move,
            white_moves: 0,
            white_moves_gen: false,
            black_moves: 0,
            black_moves_gen: false,
        }
    }

    /// A function which generates all of the moves that black can make in the current position.
    /// # Returns:
    /// * A mask of the disks where black can make a move.
    pub fn get_black_moves(&mut self) -> u64 {
        if !self.black_moves_gen {
            self.gen_black_moves();
        }
        return self.black_moves;
    }

    /// A function which generates all of the moves that white can make in the current position.
    /// # Returns:
    /// * A mask of the disks where white can make a move.
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

    /// A function which generates all of the moves that the current player can make.
    /// # Returns:
    /// * A list of moves that can be made by the current player.
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

    /// A function which counts the number of moves that the current player can make.
    /// # Returns:
    /// * The number of moves that can be made by the current player.
    pub fn move_count(&mut self) -> u32 {
        if self.black_move {
            self.get_black_moves()
        } else {
            self.get_white_moves()
        }.count_ones()
    }

    pub fn move_count_after(&mut self, move_option: Move) -> u32 {
        match move_option {
            Move::Play(m) => {
                let disk = 0x80_00_00_00_00_00_00_00 >> m;

                let (player, opponent) = if self.black_move {
                    (self.black_disks, self.white_disks)
                } else {
                    (self.white_disks, self.black_disks)
                };

                let flood = bitboard::get_flip(m as usize, player, opponent);

                let white = self.white_disks ^ flood;
                let black = self.black_disks ^ flood;
                if self.black_move {
                    bitboard::all_moves(white ^ disk, black).count_ones()
                } else {
                    bitboard::all_moves(black ^ disk, white).count_ones()
                }
            }
            Move::Pass => {
                if self.black_move {
                    self.get_white_moves()
                } else {
                    self.get_black_moves()
                }.count_ones()
            }
        }
    }

    /// A function which determines whether a player can make moves.
    /// # Returns:
    /// * true if either player has moves, false otherwise.
    pub fn moves_exist(&mut self) -> bool {
        (self.get_white_moves() | self.get_black_moves()) != 0
    }

    /// A function makes a move for the current player. Does not check if the move is valid in the
    /// current position.
    /// # Arguments:
    /// * `move_option`: The move to make
    pub fn make_move(&mut self, move_option: Move) -> u64 {
        match move_option {
            Move::Play(m) => {
                let disk = 0x80_00_00_00_00_00_00_00 >> m;

                let (player, opponent) = if self.black_move {
                    (self.black_disks, self.white_disks)
                } else {
                    (self.white_disks, self.black_disks)
                };

                let flood = bitboard::get_flip(m as usize, player, opponent);

                self.white_disks ^= flood;
                self.black_disks ^= flood;
                if self.black_move {
                    self.white_disks ^= disk;
                } else {
                    self.black_disks ^= disk;
                }

                self.black_move = !self.black_move;


                self.black_moves_gen = false;
                self.white_moves_gen = false;

                flood ^ disk
            }
            Move::Pass => {
                self.black_move = !self.black_move;
                self.black_moves_gen = false;
                self.white_moves_gen = false;
                0
            }
        }
    }

    /// A function undoes a move for the current player. Does not check if the move is valid in the
    /// current position.
    /// # Arguments:
    /// * `move_option`: The move to undo
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
    /// A function that returns a bitmask of all of the disks on the board.
    /// # Returns:
    /// * A bitmask containing all of the disks.
    pub fn all_disks(&self) -> u64 {
        self.white_disks | self.black_disks
    }

    /// A function that checks if the game is over.
    /// # Returns:
    /// * true if the game is over, false otherwise
    pub fn is_game_over(&mut self) -> bool {
        self.black_disks == 0 || self.white_disks == 0 || !self.moves_exist()
    }

    pub fn get_score(&self) -> i32 {
        self.black_disks.count_ones() as i32 - self.white_disks.count_ones() as i32
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  A B C D E F G H \t  A B C D E F G H \n").unwrap();

        let disk_char = | r: usize, f: usize, black: bool | {
            let disk = bitboard::RANKS[r] & bitboard::FILES[f];
            if self.black_disks & disk > 0 && black {
                "#"
            } else if self.white_disks & disk > 0 && !black {
                "#"
            } else {
                "-"
            }
        };

        for rank in 0..8 {
            write!(f, "{} {} {} {} {} {} {} {} {}", rank + 1,
                   disk_char(rank, 0, true), disk_char(rank, 1, true),
                   disk_char(rank, 2, true), disk_char(rank, 3, true),
                   disk_char(rank, 4, true), disk_char(rank, 5, true),
                   disk_char(rank, 6, true), disk_char(rank, 7, true)).unwrap();
            write!(f, "\t").unwrap();
            write!(f, "{} {} {} {} {} {} {} {} {}", rank + 1,
                   disk_char(rank, 0, false), disk_char(rank, 1, false),
                   disk_char(rank, 2, false), disk_char(rank, 3, false),
                   disk_char(rank, 4, false), disk_char(rank, 5, false),
                   disk_char(rank, 6, false), disk_char(rank, 7, false)).unwrap();
            write!(f, "\n").unwrap();
        }
        write!(f, "       BLACK      \t       WHITE      ")
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
