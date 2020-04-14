use crate::board::Board;
use super::{ PatternEvaluator, pattern::PatternFile };

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;
use serde_json::{ from_reader };

#[derive(Deserialize)]
pub struct StagedPatternFile {
    stage_map: HashMap<u32, usize>,
    evaluators: Vec<PatternFile>
}

pub struct StagedPatternEvaluator {
    stage_map: HashMap<u32, usize>,
    evaluators: Vec<PatternEvaluator>
}

impl StagedPatternEvaluator {
    pub fn new() -> StagedPatternEvaluator {
        StagedPatternEvaluator {
            stage_map: HashMap::new(),
            evaluators: Vec::new()
        }
    }

    pub fn from(stage_ends: Vec<u32>, evaluators: Vec<PatternEvaluator>) -> StagedPatternEvaluator {
        assert_eq!(stage_ends.len() + 1, evaluators.len());

        let mut stage_map = HashMap::new();
        let mut last = 0;
        for (idx, &stage) in stage_ends.iter().enumerate() {
            for i in last..stage {
                stage_map.insert(i, idx);
            }
            last = stage;
        }

        for i in last..64 {
            stage_map.insert(i, evaluators.len() - 1);
        }

        StagedPatternEvaluator {
            stage_map,
            evaluators
        }
    }

    pub fn from_file(path: &str) -> Result<StagedPatternEvaluator, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let st_file: StagedPatternFile = from_reader(reader)?;
    
        Ok(st_file.to_eval())
    }
}

impl super::Evaluator for StagedPatternEvaluator {
    fn get_score(&self, board: &Board) -> i32 {
        let disks = board.all_disks().count_ones();
        let stage = self.stage_map.get(&disks).unwrap();

        self.evaluators[*stage].get_score(board)
    }
}

impl StagedPatternFile {
    pub fn to_eval(mut self) -> StagedPatternEvaluator {
        let evals = self.evaluators.drain(0..).map(|e| e.to_eval()).collect();

        StagedPatternEvaluator {
            stage_map: self.stage_map,
            evaluators: evals
        }
    }
}