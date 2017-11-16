pub fn coord_to_bitmask(pos: String) -> Option<u64> {
    let mut chars = pos.chars();
    let mut pos = 0x1;
    
    match chars.next() {
        Some(c) => {
            pos = match c.to_uppercase().next().unwrap() {
                'A' => pos << 7, 'B' => pos << 6,
                'C' => pos << 5, 'D' => pos << 4,
                'E' => pos << 3, 'F' => pos << 2,
                'G' => pos << 1, 'H' => pos,
                _ => return None
            }
        },
        None => return None
    }

    match chars.next() {
        Some(c) => {
            match c.to_string().parse::<u8>() {
                Ok(r) => {
                    pos = if r <= 8 && r >= 1 {
                        pos << 8 * (8 - r)
                    } else { return None }
                },
                Err(_) => return None
            }
        },
        None => return None
    };

    Some(pos)
}
