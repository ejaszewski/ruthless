use board;
use eval::{score};

const CORNER_MASK: u64 = 0x8100000000000081;
const ANGLE_MASK: u64 = 0xC3810000000081C3;

pub fn get_stable_squares(player: u64) -> u64 {
    let mut new_stable = CORNER_MASK & player;
    let mut stable = new_stable;
    let first = 0x80_00_00_00_00_00_00_00;

    while new_stable != 0 {
        new_stable = 0;
        let unknown = player & !stable;
        for _i in 0..unknown.count_ones() {
            let index = unknown.leading_zeros();
            let disk = first >> index;
            let mut stable_count = 0;
            for ind in 0..board::constants::SHIFT_DIRS.len() {
                let shift = board::constants::SHIFT_DIRS[ind];
                let shift_mask = board::constants::SHIFT_MASKS[ind];

                if disk & shift_mask == 0 {
                    stable_count += 1;
                } else if board::util::directional_shift(disk, shift) & stable != 0 {
                    stable_count += 1;
                }
            }

            if stable_count >= 4 {
                new_stable |= disk;
            }
        }
        stable |= new_stable;
    }

    return stable;
}
