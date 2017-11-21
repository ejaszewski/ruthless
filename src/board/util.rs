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
