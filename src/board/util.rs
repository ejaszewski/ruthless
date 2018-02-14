use board::constants;

pub fn coord_to_bitmask(pos: String) -> Option<u64> {
    let mut chars = pos.chars();

    let mut pos = match chars.next() {
        Some(c) => {
            match c.to_uppercase().next().unwrap() {
                'A' => constants::FILE_A, 'B' => constants::FILE_B,
                'C' => constants::FILE_C, 'D' => constants::FILE_D,
                'E' => constants::FILE_E, 'F' => constants::FILE_F,
                'G' => constants::FILE_G, 'H' => constants::FILE_G,
                _ => return None
            }
        },
        None => return None
    };

    pos &= match chars.next() {
        Some(c) => {
            match c.to_string().parse::<u8>() {
                Ok(r) => {
                    if r >= 1 && r <= 8 {
                        constants::RANKS[r as usize - 1]
                    } else {
                        return None;
                    }
                },
                Err(_) => return None
            }
        },
        None => return None
    };

    Some(pos)
}

#[inline]
pub fn directional_shift(x: u64, shift: i8) -> u64 {
    if shift < 0 {
        x << -shift
    } else {
        x >> shift
    }
}

pub fn directional_moves(player: u64, mask: u64, dir: i8) -> u64 {
    let mask_2 = mask & directional_shift(mask, dir);
    let mask_4 = mask_2 & directional_shift(mask_2, 2 * dir);

    let mut flip = player;
    flip |= mask   & directional_shift(flip, dir);
    flip |= mask_2 & directional_shift(flip, dir * 2);
    flip |= mask_4 & directional_shift(flip, dir * 4);

    directional_shift(flip & mask, dir)
}

pub fn all_moves(player: u64, opponent: u64) -> u64 {
    let mask = opponent & 0x7E_7E_7E_7E_7E_7E_7E_7E;

    let mut all_moves: u64 = 0;
    for shift in &constants::SHIFT_DIRS {
        let shift = *shift;
        if shift == 8 || shift == -8 {
            all_moves |= directional_moves(player, opponent, shift);
        } else {
            all_moves |= directional_moves(player, mask, shift);
        }
    }

    all_moves & !(player | opponent)
}
