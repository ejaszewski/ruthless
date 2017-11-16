pub mod util;

use std::fmt;

pub struct Board {
    light_disks: u64,
    dark_disks: u64,
    dark_move: bool
}

impl Board {
    pub fn new() -> Board {
        let mut light_disks = 0;
        light_disks |= util::coord_to_bitmask(String::from("e5")).unwrap();
        light_disks |= util::coord_to_bitmask(String::from("d4")).unwrap();

        let mut dark_disks = 0;
        dark_disks |= util::coord_to_bitmask(String::from("e4")).unwrap();
        dark_disks |= util::coord_to_bitmask(String::from("d5")).unwrap();

        let dark_move = false;
        Board {
            light_disks, dark_disks, dark_move
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
