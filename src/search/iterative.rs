/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Contains an implementation of iterative deepening which allows you to select the algorithm
//! which you want to use to perform the search. Currently implemented are Best Node Search and
//! Negamax. All functions take time in millis as the parameter.

use crate::board::{ Board, Move };
use crate::search::{ SearchData, bns, negamax, eval::Evaluator };

const MIN_SEARCH_DEPTH: u8 = 8;

pub fn bns_iter_deep<T: Evaluator>(board: &mut Board, time: u32, evaluator: &T) -> (i32, Move, SearchData) {
    let mut depth = MIN_SEARCH_DEPTH;
    let mut time_prediction = 0;
    let mut time_spent = 0;

    let mut searched_total = 0;
    
    let mut best_move = board.get_moves()[0];
    let mut best_score = 0;

    let mut branching_factor;

    while time_prediction < time {
        let (score, m, data) = bns::best_node_search(board, depth, evaluator);

        best_move = m;
        best_score = score;
        searched_total += data.nodes;
        
        branching_factor = (data.nodes as f32).powf(1.0 / depth as f32);
        time_prediction = (data.time as f32 * branching_factor + time_spent as f32) as u32;

        depth += 1;
        time_spent += data.time;
    }

    depth -= 1;

    println!("Final search was depth {}. Total time was {:.2} s", depth - 1, time_spent as f32 / 1000.0);

    (best_score, best_move, SearchData { nodes: searched_total, time: time_spent, depth })
}

pub fn nm_iter_deep<T: Evaluator>(board: &mut Board, time: u32, evaluator: &T) -> (i32, Move, SearchData) {
    let mut depth = MIN_SEARCH_DEPTH;
    let mut time_prediction = 0;
    let mut time_spent = 0;

    let mut searched_total = 0;
    
    let mut best_move = board.get_moves()[0];
    let mut best_score = 0;

    let mut branching_factor;

    while time_prediction < time {
        let (score, m, data) = negamax::negamax(board, depth, evaluator, false);

        best_move = m;
        best_score = score;
        searched_total += data.nodes;
        
        branching_factor = (data.nodes as f32).powf(1.0 / depth as f32);
        time_prediction = (data.time as f32 * branching_factor + time_spent as f32) as u32;

        depth += 1;
        time_spent += data.time;
    }

    depth -= 1;

    eprintln!("Final search was depth {}. Total time was {:.2} s", depth, time_spent as f32 / 1000.0);

    (best_score, best_move, SearchData { nodes: searched_total, time: time_spent, depth })
}