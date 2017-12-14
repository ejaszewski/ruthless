extern crate clap;

use std::str;
use self::clap::ArgMatches;

/*
Layout of these constants:
    A B C D D C B A
    B E F G G F E B
    C F H I I H F C
    D G I J J I G D
    D G I J J I G D
    C F H I I H F C
    B E F G G F E B
    A B C D D C B A
*/
const MATERIAL_MASKS: [u64; 10] = [
    0x81_00_00_00_00_00_00_81, // A
    0x42_81_00_00_00_00_81_42, // B
    0x24_00_81_00_00_81_00_24, // C
    0x18_00_00_81_81_00_00_18, // D
    0x00_42_00_00_00_00_42_00, // E
    0x00_18_00_42_42_00_18_00, // G
    0x00_24_42_00_00_42_24_00, // F
    0x00_00_24_00_00_24_00_00, // H
    0x00_00_18_24_24_18_00_00, // I
    0x00_00_00_18_18_00_00_00, // J
];

struct Properties {
    max_depth: u8,
    material_weight: f32,
    mobility_weight: f32,
    material_eval: [(u64, f32)]
}

impl Properties {
    pub fn from_args(matches: &ArgMatches) {
        let max_depth: u8;
        let material_weight: f32;
        let mobility_weight: f32;
        let mut material_eval: [(u64, f32); 10];

        match matches.value_of("depth") {
            Some(depth) => {
                max_depth = str::parse::<u8>(depth).unwrap_or(1);
            },
            None => {}
        }

        match matches.value_of("mat_weight") {
            Some(weight) => {
                material_weight = str::parse::<f32>(weight).unwrap_or(1.0);
            },
            None => {}
        }

        match matches.value_of("mob_weight") {
            Some(weight) => {
                mobility_weight = str::parse::<f32>(weight).unwrap_or(1.0);
            },
            None => {}
        }

        match matches.values_of("tile_weights") {
            Some(weights) => {
                material_eval = [(0, 0.0); 10];
                let mut i = 0;
                for weight in weights {
                    material_eval[i] = (MATERIAL_MASKS[i], str::parse::<f32>(weight).unwrap_or(1.0));
                    i += 1;
                }
            },
            None => {}
        }
    }
}
