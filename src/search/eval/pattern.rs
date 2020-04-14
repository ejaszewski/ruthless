/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::board::Board;
use super::pattern_util::*;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;
use serde_json::{ from_reader };

#[derive(Deserialize)]
pub struct PatternFile {
    masks: Vec<u64>,
    weights: Vec<Vec<f32>>,
    parity_e: f32,
    parity_o: f32
}

#[derive(Default)]
pub struct PatternEvaluator {
    patterns: Vec<(u64, Vec<f32>)>,
    parity_e: f32,
    parity_o: f32
}

impl PatternEvaluator {
    pub fn new() -> Self {
        PatternEvaluator {
            patterns: Vec::new(),
            parity_e: 0f32,
            parity_o: 0f32
        }
    }

    pub fn from(masks: Vec<u64>, weights: Vec<Vec<f32>>, parity_e: f32, parity_o: f32) -> PatternEvaluator {
        let rotate = | mask: u64, weight: &Vec<f32> | {
            let mut new_masks = vec![mask];
            let mut new_weights = vec![weight.clone()];

            let max_idx = 3usize.pow(mask.count_ones() as u32);
            let max_val = 2usize.pow(mask.count_ones() as u32);

            let mut board = [-1; 64];
            let mut idx = 0;
            for i in 0..64 {
                if mask & (0x80_00_00_00_00_00_00_00 << i) == 1 {
                    board[i] = idx;
                    idx += 1;
                } 
            }

            let rotate_board = | a: u64, i: u8 | {
                let mut temp = a;
                for _ in 0..i {
                    temp = flip_vertical(flip_diag(temp));
                }
                temp
            };

            for r in 1..4 {
                // 90 degree clockwise rotation
                let rotated = rotate_board(mask, r);
                
                //iterate through 0..3**n
                //pdep into pattern
                //rotate pdeped val
                //pext with rotated pattern
                //write to vec at index

                let mut rot_weight = vec![0.0; max_idx];
                for i in 0..max_val {
                    for j in 0..max_val {
                        if i & j == 0 {
                            let mut idep = pdep64(i as u64, mask);
                            let mut jdep = pdep64(j as u64, mask);
                            idep = rotate_board(idep, r);
                            jdep = rotate_board(jdep, r);
                            let iext = pext64(idep, rotated) as usize;
                            let jext = pext64(jdep, rotated) as usize;
                            rot_weight[ONES_TERNARY[iext] + TWOS_TERNARY[jext]] = weight[ONES_TERNARY[i] + TWOS_TERNARY[j]];
                        }
                    }
                }

                new_masks.push(rotated);
                new_weights.push(rot_weight);
            }

            (new_masks, new_weights)
        };

        let mut all_masks = vec![];
        let mut all_weights = vec![];

        masks.iter().zip(weights).map(| (&m, w) | rotate(m, &w)).for_each(| (mut m, mut w) | {
            all_masks.append(&mut m);
            all_weights.append(&mut w);
        });

        PatternEvaluator {
            patterns: all_masks.iter().zip(all_weights).map(| (&m, w) | (m, w)).collect(),
            parity_e, parity_o
        }
    }

    pub fn from_file(path: &str) -> Result<PatternEvaluator, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let pat_file: PatternFile = from_reader(reader)?;
    
        Ok(pat_file.to_eval())
    }
}

impl super::Evaluator for PatternEvaluator {
    fn get_score(&self, board: &Board) -> i32 {
        let mut score: f32 = if board.all_disks().count_zeros() & 1 == 1 {
            self.parity_o
        } else {
            self.parity_e
        };

        let blacks = board.black_disks;
        let whites = board.white_disks;

        for (mask, weights) in self.patterns.iter() {
            // Extract the pattern from both bitboards
            let black_pat = pext64(blacks, *mask) as usize;
            let white_pat = pext64(whites, *mask) as usize;
            // Get the index of the given pattern
            let index = ONES_TERNARY[white_pat] + TWOS_TERNARY[black_pat];

            // Add the pattern's weight to the score
            score += weights[index];
        }

        if board.black_move {
            (score * 100.0) as i32
        } else {
            (-score * 100.0) as i32
        }
    }
}

impl PatternFile {
    pub fn to_eval(self) -> PatternEvaluator {
        PatternEvaluator::from(self.masks, self.weights, self.parity_e, self.parity_o)
    }
}