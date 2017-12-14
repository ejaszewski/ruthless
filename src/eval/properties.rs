extern crate clap;

use std::str;
use self::clap::ArgMatches;

static mut MAX_DEPTH: u8 = 1;

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
static mut MATERIAL_EVAL: [(u64, f32); 10] = [
    (0x81_00_00_00_00_00_00_81, 1.0), // A
    (0x42_81_00_00_00_00_81_42, 1.0), // B
    (0x24_00_81_00_00_81_00_24, 1.0), // C
    (0x18_00_00_81_81_00_00_18, 1.0), // D
    (0x00_42_00_00_00_00_42_00, 1.0), // E
    (0x00_18_00_42_42_00_18_00, 1.0), // G
    (0x00_24_42_00_00_42_24_00, 1.0), // F
    (0x00_00_24_00_00_24_00_00, 1.0), // H
    (0x00_00_18_24_24_18_00_00, 1.0), // I
    (0x00_00_00_18_18_00_00_00, 1.0), // J
];

static mut MATERIAL_WEIGHT: f32 = 1.0;
static mut MOBILITY_WEIGHT: f32 = 1.0;

pub unsafe fn load_from_args(matches: &ArgMatches) {
    match matches.value_of("depth") {
        Some(depth) => {
            MAX_DEPTH = str::parse::<u8>(depth).unwrap_or(1);
        },
        None => {}
    }
    match matches.value_of("mat_weight") {
        Some(material_weight) => {
            MATERIAL_WEIGHT = str::parse::<f32>(material_weight).unwrap_or(1.0);
        },
        None => {}
    }
    match matches.value_of("mob_weight") {
        Some(mobility_weight) => {
            MOBILITY_WEIGHT = str::parse::<f32>(mobility_weight).unwrap_or(1.0);
        },
        None => {}
    }
    match matches.values_of("tile_weights") {
        Some(tile_weights) => { println!("{:?}", tile_weights.collect::<Vec<&str>>()) },
        None => {}
    }
}
