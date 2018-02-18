extern crate clap;
extern crate serde;
extern crate serde_json;

use std::str;
use self::serde_json::Error;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Heuristic {
    pub min_squares: u32,
    pub max_squares: u32,
    pub depth: u8,
    pub bias: f32,
    pub material_weight: f32,
    pub mobility_weight: f32,
    pub square_values: [f32; 10],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    pub id: String,
    pub heuristics: Vec<Heuristic>,
}

impl Properties {
    pub fn from_json(json: &str) -> Option<Properties> {
        let props: Result<Properties, Error> = serde_json::from_str(json);
        match props {
            Ok(properties) => Some(properties),
            Err(_) => None
        }
    }

    pub fn get_heuristic(&self, squares: u32) -> &Heuristic {
        let index = self.heuristics.iter().position(|h| squares > h.min_squares && squares < h.max_squares).unwrap_or(0);
        self.heuristics.get(index).unwrap()
    }
}
