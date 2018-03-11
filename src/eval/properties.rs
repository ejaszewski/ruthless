extern crate clap;
extern crate serde;
extern crate serde_json;

use std::str;
use self::serde_json::Error;

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
