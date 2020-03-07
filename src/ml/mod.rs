use crate::board::{ Board };
use crate::search::negamax;
use crate::search::eval::{ Evaluator, PieceSquareEvaluator };

use std::f32;
use std::io::{ self, Write, BufReader };
use std::fs::File;
use rand::prelude::*;
use serde_json::{ from_reader };

pub mod eval;

pub trait Trainable {
    fn update(&mut self, board: &Board, score: f32, lr: f32) -> f32;
    fn get_float_score(&self, board: &Board) -> f32;
    fn loss_ema(&self) -> f32;
}

pub struct BoardState {
    pub black_disks: u64,
    pub white_disks: u64,
    pub black_move: bool
}

impl BoardState {
    pub fn from(b: &Board) -> BoardState {
        BoardState {
            black_disks: b.black_disks,
            white_disks: b.white_disks,
            black_move: b.black_move
        }
    }

    pub fn to_board(&self) -> Board {
        Board::from_pos(self.black_disks, self.white_disks, self.black_move)
    }
}

const RESET_RATIO: f32 = 1.2;
const FORGIVENESS: f32 = 1.0;
const TD_LAMBDA: f32 = 0.2;

pub fn self_play<E: Evaluator + Trainable + Clone>(mut eval: E, mut lr: f32, mut e: f32, batch_size: usize, rounds: usize) -> E {
    let mut rng = thread_rng();

    let mut checkpoint: Option<(E, f32)> = None;

    for r in 0..rounds {
        let mut count = 0;

        while count < batch_size {
            let mut board = Board::new();
            let (_, n) = self_play_td_impl(&mut board, &mut rng, &mut eval, e, lr);
            count += n;
        }

        let (black_win_ps, white_win_ps, black_win_pat, white_win_pat) = game_stats(&eval);
        
        let escore = (black_win_ps + white_win_ps) * (black_win_pat + white_win_pat);

        let mut bps = 0.0;
        if let Some((_, ps_score)) = &checkpoint {
            bps = *ps_score;
        }

        eprintln!("{} {} {} {} {}", black_win_ps, white_win_ps, black_win_pat, white_win_pat, bps);

        if r % (rounds / 100) == 0 {
            println!("\rStats:           ");
            println!("\tAvg. Loss    : {}", eval.loss_ema());
            println!("\tB v. PST Wins: {:>5.1}%", black_win_ps * 100.0);
            println!("\tW v. PST Wins: {:>5.1}%", white_win_ps * 100.0);
            println!("\tB v. PAT Wins: {:>5.1}%", black_win_pat * 100.0);
            println!("\tW v. PAT Wins: {:>5.1}%", white_win_pat * 100.0);

            lr *= 0.99;
        }

        if r % 10 == 0 {
            if let Some((eval_best, ps_score)) = &checkpoint {
                if *ps_score > (escore * RESET_RATIO) {
                    println!("\rResetting evaluator to checkpoint.");
                    eval = eval_best.clone();
                } else if escore > (ps_score * FORGIVENESS) {
                    checkpoint = Some((eval.clone(), escore));
                    println!("\rSaving new checkpoint.");
                }
            } else {
                checkpoint = Some((eval.clone(), escore));
            }

            // e *= 0.95;
        }

        print!("\rProgress: {:>5.1}%", (100.0 * r as f32) / rounds as f32);
        io::stdout().flush().expect("Unable to flush stdout.");
    }

    if let Some((best, _)) = checkpoint {
        best
    } else {
        eval
    }
}

fn self_play_td_impl<E: Evaluator + Trainable>(board: &mut Board, rng: &mut ThreadRng, eval: &mut E, e: f32, lr: f32) -> (f32, usize) {
    if board.is_game_over() {
        return (
            if board.black_move { board.get_score() } else { -board.get_score() } as f32,
            1
        );
    }

    let m;
    if rng.gen::<f32>() > e {
        let (_, best_move, _) = negamax::negamax(board, 1, eval, false); 
        m = best_move;
    } else {
        let moves = board.get_moves();
        m = moves[rng.gen::<usize>() % moves.len()];
    }
    
    let undo = board.make_move(m);
    let (score, count) = self_play_td_impl(board, rng, eval, e, lr);
    board.undo_move(undo, m);

    eval.update(board, -score, lr);

    let td_score = (1.0 - TD_LAMBDA) * eval.get_float_score(board) - TD_LAMBDA * score;

    (td_score, count + 1)
}


fn game_stats<E: Evaluator>(eval: &E) -> (f32, f32, f32, f32) {
    let file = File::open("bench.json").expect("File read error.");
    let reader = BufReader::new(file);
    let pat_eval: eval::RLPatternEvaluator = from_reader(reader).expect("Unable to parse json");

    let ps_eval = PieceSquareEvaluator::new();

    let (black_win_ps, white_win_ps) = test(500, eval, &ps_eval);

    let (black_win_pat, white_win_pat) = test(500, eval, &pat_eval);

    (black_win_ps, white_win_ps, black_win_pat, white_win_pat)
}

fn test<E:Evaluator, B: Evaluator>(num_games: u64, eval: &E, bench: &B) -> (f32, f32) {
    let mut wins_b = 0;
    let mut wins_w = 0;
    let mut rng = thread_rng();

    for _ in 0..num_games {
        // Play as black
        let mut board = Board::new();
        while !board.is_game_over() {
            let (_, m, _) = negamax::negamax::<E>(&mut board, 1, eval, false);
            board.make_move(m);

            if rng.gen::<f32>() > 0.05 {
                let (_, m, _) = negamax::negamax(&mut board, 1, bench, false);
                board.make_move(m);
            } else {
                let moves = board.get_moves();
                board.make_move(moves[rng.gen::<usize>() % moves.len()]);
            }
        }

        if board.get_score() > 0 {
            wins_b += 1;
        }

        // Play as white
        let mut board = Board::new();
        while !board.is_game_over() {
            let (_, m, _) = negamax::negamax::<E>(&mut board, 1, eval, false);
            board.make_move(m);

            if rng.gen::<f32>() > 0.05 {
                let (_, m, _) = negamax::negamax(&mut board, 1, bench, false);
                board.make_move(m);
            } else {
                let moves = board.get_moves();
                board.make_move(moves[rng.gen::<usize>() % moves.len()]);
            }
        }

        if board.get_score() > 0 {
            wins_w += 1;
        }
    }

    (wins_b as f32 / num_games as f32, wins_w as f32 / num_games as f32)
}
