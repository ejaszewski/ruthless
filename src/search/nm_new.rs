use crate::board::{ Board, Move };
use crate::search::{ SearchData, eval::{ Evaluator, PieceSquareEvaluator }, hashtable:: { Score, HashTable } };

use std::collections::HashMap;
use std::i32;
use std::io::{ Write, stdout };
use std::time::Instant;

const MIN_SEARCH_DEPTH: u8 = 8;

// Hashtable Params
const DEFAULT_TABLE_SIZE: usize = 8276803;

// Probcut Params
const PROBCUT_DEPTH: u8 = 6;
const PROBCUT_SHALLOW: u8 = 3;
const PROBCUT_SIGMA: f32 = 0.0;
const PROBCUT_THRESHOLD: f32 = 0.0;

pub struct NegamaxSearcher<E: Evaluator> {
    eval: E,
    verbose: u8,
    output: Box<dyn Write>,
    hashtable: HashTable
}

impl<E: Evaluator> NegamaxSearcher<E> {
    pub fn new() -> NegamaxSearcher<PieceSquareEvaluator> {
        let eval = PieceSquareEvaluator::new();

        NegamaxSearcher {
            eval: eval,
            verbose: 1,
            output: Box::new(stdout()),
            hashtable: HashTable::empty(DEFAULT_TABLE_SIZE)
        }
    }

    pub fn with_eval(eval: E) -> Self {
        NegamaxSearcher {
            eval: eval,
            verbose: 1,
            output: Box::new(stdout()),
            hashtable: HashTable::empty(DEFAULT_TABLE_SIZE)
        }
    }

    pub fn set_verbose(&mut self, verbose: u8) {
        self.verbose = verbose;
    }

    pub fn set_output(&mut self, output: Box<dyn Write>) {
        self.output = output;
    }

    pub fn search(&mut self, board: &mut Board, time: u32) -> (i32, Move, SearchData) {
        let mut moves = board.get_moves();
        moves.sort_by(|&m| -self.eval.move_order_score(board, m));

        let mut scores: HashMap<Move, i32> = HashMap::new();

        let mut total_nodes = 0;
        let mut total_millis = 0;

        let mut depth = MIN_SEARCH_DEPTH;
        let mut time_prediction = 0;

        let mut branching_factor;

        let mut best_move = moves[0];
        let mut best_move_score = 0;

        while time_prediction + total_millis < time {
            let beta = i32::MAX;

            let mut best_score = -beta;

            let mut iter_nodes = 0;
            let mut iter_time = 0;

            let mut first = true;

            for m in &moves {
                if self.verbose > 0 {
                    write!(self.output, "Evaluating: {}", m).expect("Unable to write to output stream.");
                    self.output.flush().expect("Unable to flush output stream.");
                }

                let start_time = Instant::now();

                let mut score;
                let mut nodes = 0;

                let undo = board.make_move(m);
                if first {
                    let (result, s_nodes) = self.pvs_impl(board, -beta, -best_score, depth - 1);
    
                    score = -result;
                    nodes += s_nodes;
                } else {
                    let (result, s_nodes) = self.pvs_impl(board, -best_score - 1, -best_score, depth - 1);
    
                    score = -result;
                    nodes += s_nodes;
    
                    if score > best_score && score < beta {
                        let (result, s_nodes) = self.pvs_impl(board, -beta, -score, depth - 1);
                        score = -result;
                        nodes += s_nodes;
                    }
                }
                board.undo_move(undo, m);

                scores.insert(m, score);

                let end_time = Instant::now();
                let duration = end_time - start_time;
                let time_taken = duration.as_secs() as u32 * 1000 + duration.subsec_millis();

                if score > best_score {
                    best_move = m;
                    best_move_score = score;
                    if self.verbose > 0 {
                        writeln!(
                            self.output,
                            " -- Score: {:3}, Nodes: {}, Time {} ms",
                            score,
                            nodes,
                            time_taken
                        ).expect("Unable to write to output stream.");
                    }
                    best_score = score;
                } else if self.verbose > 0 {
                    writeln!(
                        self.output,
                        " --             Nodes: {}, Time {} ms",
                        nodes,
                        time_taken
                    ).expect("Unable to write to output stream.");
                }
                
                iter_nodes += nodes;
                iter_time += time_taken;
                total_nodes += nodes;
                total_millis += time_taken;

                first = false;
            }

            moves.sort_by(|&m| {
                let undo = board.make_move(m);

                let entry = self.hashtable.probe(board, depth, -i32::MAX, i32::MAX);
                let value = if let Some(score) = entry {
                    match score {
                        Score::Exact(score) => score,
                        Score::Lower(score) => score,
                        Score::Upper(score) => score
                    }
                } else {
                    i32::MAX
                };

                board.undo_move(undo, m);
        
                value
            });
            
            branching_factor = (iter_nodes as f32).powf(1.0 / depth as f32);
            
            depth += 2;
            time_prediction = (iter_time as f32 * (branching_factor.powi(2) + 1.0)) as u32;

            writeln!(self.output, "Took {} ms", iter_time).expect("Unable to write to output stream.");
            writeln!(self.output, "Predicted next time is {} ms", time_prediction).expect("Unable to write to output stream.");
        }

        depth -= 2;
        
        if self.verbose > 0 {
            writeln!(
                self.output,
                "Searched {} nodes in {} ms ({:.2} kn/s)",
                total_nodes,
                total_millis,
                total_nodes as f32 / total_millis as f32
            ).expect("Unable to write to output stream.");
            writeln!(self.output, "Final search was depth {}", depth).expect("Unable to write to output stream.");
        }

        self.hashtable.set_replace();

        (best_move_score, best_move, SearchData { nodes: total_nodes, time: total_millis, depth })
    }

    fn pvs_impl(&mut self, board: &mut Board, mut alpha: i32, mut beta: i32, depth: u8) -> (i32, u64) {
        if board.is_game_over() || depth == 0 {
            return (self.eval.get_score(board), 1);
        }

        if depth > 3 {
            let entry = self.hashtable.probe(board, depth, alpha, beta);
            if let Some(score) = entry {
                let node_value;

                match score {
                    Score::Exact(score) => return (score, 1),
                    Score::Lower(score) => {
                        alpha = score;
                        node_value = score;
                    },
                    Score::Upper(score) => {
                        beta = score;
                        node_value = score;
                    }
                }

                if alpha >= beta {
                    return (node_value, 1);
                }
            }
        }
        
        let mut moves = board.get_moves();
        if depth > 4 {
            moves.sort_by(|&m| {
                let half_depth = ((depth / 2) & !0x1) | (depth & 0x1);
                let undo = board.make_move(m);
                let (result, _) = self.pvs_impl(board, -beta, -alpha, half_depth);
                board.undo_move(undo, m);
        
                -result
            });
        }
        
        let mut total_nodes = 1;

        let alpha_original = alpha;
        let mut best_score = -i32::MAX;

        let mut first = true;
    
        for m in &moves {
            let undo = board.make_move(m);
            let mut score;
            if first {
                let (result, nodes) = self.pvs_impl(board, -beta, -alpha, depth - 1);

                score = -result;
                total_nodes += nodes;
            } else {
                let (result, nodes) = self.pvs_impl(board, -alpha - 1, -alpha, depth - 1);

                score = -result;
                total_nodes += nodes;

                if score > alpha && score < beta {
                    let (result, nodes) = self.pvs_impl(board, -beta, -score, depth - 1);
                    score = -result;
                    total_nodes += nodes;
                }
            }
            board.undo_move(undo, m);
    
    
            if score > best_score {
                best_score = score;
                if score > alpha {
                    alpha = score;
                }
            }
    
            if alpha >= beta {
                break;
            }

            first = false;
        }

        if depth > 3 {
            let node_score;
            if best_score < alpha_original {
                node_score = Score::Upper(best_score);
                self.hashtable.save(board, node_score, depth);
            } else if best_score >= beta {
                node_score = Score::Lower(best_score);
                self.hashtable.save(board, node_score, depth);
            } else if best_score > alpha_original && alpha < beta {
                node_score = Score::Exact(best_score);
                self.hashtable.save(board, node_score, depth);
            }
        }
    
        (alpha, total_nodes)
    }
}