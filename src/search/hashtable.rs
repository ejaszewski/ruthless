use crate::board::{ Board, Move };

#[derive(Clone, Copy, Debug)]
pub enum Score {
    Exact(i32),
    Lower(i32),
    Upper(i32)
}

#[derive(Clone, Copy, Debug)]
struct Entry {
    depth: u8,
    score: Score,
    state: u128,
    replace: bool
}

pub struct HashTable {
    size: usize,
    table: Vec<Option<Entry>>
}

impl HashTable {
    pub fn empty(size: usize) -> Self {
        let mut table = Vec::new();
        table.resize_with(size, || None);

        HashTable {
            size,
            table
        }
    }

    pub fn probe(&self, board: &Board, depth: u8, alpha: i32, beta: i32) -> Option<Score> {
        let state = ((board.black_disks as u128) << 64) + board.white_disks as u128;
        let index = (state % self.size as u128) as usize;
        let value = self.table[index];

        if let Some(entry) = value {
            if entry.state == state && entry.depth >= depth {
                return match entry.score {
                    Score::Exact(_) => Some(entry.score),
                    Score::Lower(score) if score >= beta => Some(entry.score),
                    Score::Upper(score) if score <= alpha => Some(entry.score),
                    _ => None
                }
            }
        }

        None
    }

    pub fn save(&mut self, board: &Board, score: Score, depth: u8) {
        let state = ((board.black_disks as u128) << 64) + board.white_disks as u128;
        let index = (state % self.size as u128) as usize;
        let value = self.table[index];

        match value {
            Some(entry) if entry.depth > depth && !entry.replace => {},
            _ => {
                let new_entry = Entry {
                    depth, score, state, replace: false
                };

                self.table[index] = Some(new_entry);
            }
        }
    }

    pub fn set_replace(&mut self) {
        for value in self.table.iter_mut() {
            if let Some(entry) = value {
                entry.replace = true;
            }
        }
    }
}