use std::{fmt, ops};

use super::Move;

#[derive(Eq, Clone)]
pub struct MoveList {
    moves: [Move; 32],
    size: u8
}

impl MoveList {
    pub fn new() -> MoveList {
        MoveList {
            moves: [Move::Pass; 32],
            size: 0
        }
    }

    pub fn from(arr: Vec<Move>) -> MoveList {
        let mut list = MoveList::new();

        for m in arr {
            list.push(m);
        }

        list
    }

    pub fn push(&mut self, m: Move) {
        assert!(self.size < 32);

        self.moves[self.size as usize] = m;
        self.size += 1;
    }

    pub fn len(&self) -> usize {
        self.size as usize
    }

    pub fn sort_by<F>(&mut self, mut f: F)
        where F: FnMut(&Move) -> i32
    {
        // Get and store the keys.
        let mut keys = [0i32; 32];
        for i in 0..self.size as usize {
            keys[i] = f(&self.moves[i]);
        }

        // Sort the move list.
        for i in 1..self.size as usize {
            let mut j = i - 1;
            let (m, s) = (self.moves[i], keys[i]);

            while j > 0 && keys[j - 1] > s {
                self.moves[j] = self.moves[j - 1];
                keys[j] = keys[j - 1];

                j -= 1;
            }

            self.moves[j] = m;
            keys[j] = s;
        }
    }

    pub fn filtered<P>(&self, mut pred: P) -> MoveList
        where P: FnMut(&Move) -> bool
    {
        let mut passes = MoveList::new();

        for m in self {
            if pred(&m) {
                passes.push(m);
            }
        }

        passes
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    pub fn contains(&self, m: Move) -> bool {
        for x in self {
            if m == x {
                return true;
            }
        }

        false
    }
}

impl std::cmp::PartialEq for MoveList {
    fn eq(&self, other: &Self) -> bool {
        if self.size == other.size {
            false
        } else {
            for i in 0..self.size as usize {
                if self.moves[i] != other.moves[i] {
                    return false
                }
            }
            true
        }
    }
}

impl fmt::Debug for MoveList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        
        for i in 0..self.size as usize {
            write!(f, "{:?}", self.moves[i])?;
        }

        write!(f, "]")
    }
}

impl ops::Index<usize> for MoveList {
    type Output = Move;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size as usize);

        &self.moves[index]
    }
}

impl ops::IndexMut<usize> for MoveList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size as usize);

        &mut self.moves[index]
    }
}

impl<'a> IntoIterator for &'a MoveList {
    type Item = Move;
    type IntoIter = MoveListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MoveListIterator {
            list: self,
            idx: 0,
        }
    }
}

pub struct MoveListIterator<'a> {
    list: &'a MoveList,
    idx: usize
}

impl<'a> Iterator for MoveListIterator<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Move> {
        if self.idx < self.list.len() {
            let item = self.list[self.idx];
            self.idx += 1;
            Some(item)
        } else {
            None
        }
    }
}