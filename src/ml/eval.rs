use crate::board::Board;
use crate::search::eval::{ Evaluator, pattern_util::* };

use super::Trainable;

use std::collections::HashMap;
use serde::{ Deserialize, Serialize };
use rand::prelude::*;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct RLPatternEvaluator {
    masks: Vec<u64>,
    weights: Vec<Vec<f32>>,
    parity_e: f32,
    parity_o: f32,

    #[serde(skip)]
    loss_ema: f32
}

impl RLPatternEvaluator {
    pub fn new() -> Self {
        RLPatternEvaluator {
            masks: Vec::new(),
            weights: Vec::new(),
            parity_e: 0f32,
            parity_o: 0f32,
            loss_ema: 0f32
        }
    }

    pub fn from(masks: Vec<u64>, weights: Vec<Vec<f32>>, parity_e: f32, parity_o: f32) -> RLPatternEvaluator {
        RLPatternEvaluator {
            masks, weights, parity_e, parity_o, 
            loss_ema: 0f32
        }
    }

    pub fn from_masks(masks: Vec<u64>) -> RLPatternEvaluator {
        let mut rng = thread_rng();
        let mut weights = Vec::new();

        let mut small_random = || {
            rng.gen::<f32>() * 0.2 - 0.1
        };
        
        for mask in &masks {
            let size = 3usize.pow(mask.count_ones());
            let mut w_arr = Vec::with_capacity(size);

            for _ in 0..size {
                w_arr.push(small_random());
            }

            weights.push(w_arr);
        }

        RLPatternEvaluator {
            masks,
            weights,
            parity_e: 0f32,
            parity_o: 0f32,
            loss_ema: 0f32
        }
    }

    pub fn reset(&mut self) {
        for weights in self.weights.iter_mut() {
            for i in 0..weights.len() {
                weights[i] = 0.0;
            }
        }
    }
}

impl Trainable for RLPatternEvaluator {
    fn update(&mut self, board: &Board, score: f32, lr: f32) -> f32 {
        let mut error = self.get_float_score(board) - score;

        if !board.black_move {
            error = -error;
        }

        let gradient = error;//error / (error.powi(2) + 1.0).sqrt();
        let loss = error.powi(2);//(error.powi(2) + 1.0).sqrt() - 1.0;

        const EMA_A: f32 = 0.0001;

        self.loss_ema = (1.0 - EMA_A) * self.loss_ema + EMA_A * loss;

        if board.all_disks().count_zeros() & 1 == 1 {
            self.parity_o -= lr * gradient;
        } else {
            self.parity_e -= lr * gradient;
        };

        let mut blacks = board.black_disks;
        let mut whites = board.white_disks;

        for (mask, weights) in self.masks.iter().zip(self.weights.iter_mut()) {
            for _ in 0..4 {
                // Extract the pattern from both bitboards
                let black_pat = pext64(blacks, *mask) as usize;
                let white_pat = pext64(whites, *mask) as usize;
                // Get the index of the given pattern
                let index = ONES_TERNARY[white_pat] + TWOS_TERNARY[black_pat];

                // Add the pattern's weight to the score
                weights[index] -= lr * gradient;

                // Rotate
                blacks = flip_vertical(flip_diag(blacks));
                whites = flip_vertical(flip_diag(whites));
            }
        }

        loss
    }

    fn get_float_score(&self, board: &Board) -> f32 {
        let mut score: f32 = if board.all_disks().count_zeros() & 1 == 1 {
            self.parity_o
        } else {
            self.parity_e
        };

        let mut blacks = board.black_disks;
        let mut whites = board.white_disks;

        for (mask, weights) in self.masks.iter().zip(self.weights.iter()) {
            for _ in 0..4 {
                // Extract the pattern from both bitboards
                let black_pat = pext64(blacks, *mask) as usize;
                let white_pat = pext64(whites, *mask) as usize;
                // Get the index of the given pattern
                let index = ONES_TERNARY[white_pat] + TWOS_TERNARY[black_pat];

                // Add the pattern's weight to the score
                score += weights[index];

                // Rotate
                blacks = flip_vertical(flip_diag(blacks));
                whites = flip_vertical(flip_diag(whites));
            }
        }

        if board.black_move {
            score
        } else {
            -score
        }
    }

    fn loss_ema(&self) -> f32 {
        self.loss_ema
    }
}

impl Evaluator for RLPatternEvaluator {
    fn get_score(&self, board: &Board) -> i32 {
        (self.get_float_score(board) * 100.0) as i32
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct StagedRLPatternEvaluator {
    stage_map: HashMap<u32, usize>,
    evaluators: Vec<RLPatternEvaluator>,

    #[serde(skip)]
    loss_ema: f32
}

impl StagedRLPatternEvaluator {
    pub fn new() -> StagedRLPatternEvaluator {
        StagedRLPatternEvaluator {
            stage_map: HashMap::new(),
            evaluators: Vec::new(),
            loss_ema: 0f32
        }
    }

    pub fn from(stage_ends: Vec<u32>, evaluators: Vec<RLPatternEvaluator>) -> StagedRLPatternEvaluator {
        assert_eq!(stage_ends.len() + 1, evaluators.len());

        let mut stage_map = HashMap::new();
        let mut last = 0;
        for (idx, &stage) in stage_ends.iter().enumerate() {
            for i in last..stage {
                stage_map.insert(i, idx);
            }
            last = stage;
        }

        for i in last..65 {
            stage_map.insert(i, evaluators.len() - 1);
        }

        StagedRLPatternEvaluator {
            stage_map,
            evaluators,
            loss_ema: 0f32
        }
    }

    pub fn from_masks(masks: Vec<u64>, stage_ends: Vec<u32>) -> StagedRLPatternEvaluator {
        let mut evaluators = Vec::new();
        for _ in 0..stage_ends.len() + 1 {
            evaluators.push(RLPatternEvaluator::from_masks(masks.clone()));
        }

        let mut stage_map = HashMap::new();
        let mut last = 0;
        for (idx, &stage) in stage_ends.iter().enumerate() {
            for i in last..stage {
                stage_map.insert(i, idx);
            }
            last = stage;
        }

        for i in last..65 {
            stage_map.insert(i, evaluators.len() - 1);
        }

        StagedRLPatternEvaluator {
            stage_map,
            evaluators,
            loss_ema: 0f32
        }
    }
}

impl Trainable for StagedRLPatternEvaluator {
    fn update(&mut self, board: &Board, score: f32, lr: f32) -> f32 {
        let disks = board.all_disks().count_ones();
        let stage = self.stage_map.get(&disks).unwrap();

        let loss = self.evaluators[*stage].update(&board, score, lr);

        const EMA_A: f32 = 0.0001;

        self.loss_ema = (1.0 - EMA_A) * self.loss_ema + EMA_A * loss;

        loss
    }

    fn get_float_score(&self, board: &Board) -> f32 {
        let disks = board.all_disks().count_ones();
        let stage = self.stage_map.get(&disks).unwrap();

        self.evaluators[*stage].get_float_score(board)
    }

    fn loss_ema(&self) -> f32 {
        self.loss_ema
    }
}

impl Evaluator for StagedRLPatternEvaluator {
    fn get_score(&self, board: &Board) -> i32 {
        let disks = board.all_disks().count_ones();
        let stage = self.stage_map.get(&disks).unwrap();

        self.evaluators[*stage].get_score(board)
    }
}