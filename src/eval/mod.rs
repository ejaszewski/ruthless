extern crate time;

pub mod search;
pub mod properties;

use std::collections::HashMap;
use ::board;

// A B C D D C B A
// B B E F F E B B
// C E G F F G E C
// D F F E E F F D
const SCORE_FUNC: [(u64, f32); 7] = [
    (0x81_00_00_00_00_00_00_81, 20.0), // A
    (0x42_C3_00_00_00_00_C3_42, -5.0), // B
    (0x24_00_81_00_00_81_00_24,  5.0), // C
    (0x18_00_00_81_81_00_00_18,  5.0), // D
    (0x00_24_42_00_00_42_24_00,  2.0), // E
    (0x00_18_18_66_66_18_18_00,  2.0), // F
    (0x00_00_24_00_00_24_00_00,  2.0)  // G
];

const MATERIAL_WEIGHT: f32 = 0.5;
const MOBILITY_WEIGHT: f32 = 1.0;

fn disk_count(x: u64, mask: u64) -> f32 {
    return (x & mask).count_ones() as f32
}

pub fn get_score(board: &board::Board) -> f32 {
    let mut eval: f32 = 0.0;
    for sc in SCORE_FUNC.iter() {
        eval += (disk_count(board.dark_disks, sc.0) - disk_count(board.light_disks, sc.0)) * sc.1;
    }
    eval *= MATERIAL_WEIGHT;
    eval += board.get_moves().len() as f32 * MOBILITY_WEIGHT;
    if board.dark_move {
        eval
    } else {
        -eval
    }
}

pub fn get_score_with_props(board: &board::Board, properties: &properties::Properties) -> f32 {
    let mut material_score = 0.0;
    for &(mask, score) in SCORE_FUNC.iter() {
        material_score += (disk_count(board.dark_disks, mask) - disk_count(board.light_disks, mask)) * score;
    }
    let mobility_score = board.get_moves().len() as f32;
    let score = material_score * properties.material_weight + mobility_score * properties.mobility_weight;
    if board.dark_move {
        score
    } else {
        -score
    }
}

pub fn do_search(board: &mut board::Board, props: &properties::Properties) -> Option<u8> {

    eprintln!("Current board score: {}", get_score_with_props(board, props));

    let mut moves = board.get_moves();
    if moves.len() == 0 {
        return None
    }

    let mut searched = 0;
    let start_time = time::now();

    let mut best_move = 0;
    let mut best_score: f32 = -10001.0;

    let max_depth = props.max_depth;

    for depth in max_depth..(max_depth + 1) {
        eprint!("Evaluating moves with depth {}.", depth);

        let mut move_map: HashMap<u8, f32> = HashMap::new();

        for m in &moves {
            let undo = board.make_move(Some(*m));
            let (mut score, leaves) = search::negamax(board, props, -10000., 10000., depth);
            board.undo_move(undo, *m);

            score = -score;
            searched += leaves;

            move_map.insert(*m, score);
        }

        let mut sorted_moves: Vec<(&u8, &f32)> = move_map.iter().collect();
        sorted_moves.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        best_move = *sorted_moves[0].0;
        best_score = *sorted_moves[0].1;

        moves.clear();
        moves = sorted_moves.iter().map(|&x| *x.0).collect();

        eprintln!(" Move ordering: {:?}", moves);
        eprintln!("\tBest Move was {} with score {}.", sorted_moves[0].0, sorted_moves[0].1);
        eprintln!("\tSearched {} nodes.", searched);
        // searched = 0;
    }

    let end_time = time::now();
    let time_taken = (end_time - start_time).num_milliseconds();
    let nps = searched as f32 / time_taken as f32;

    eprintln!("Searched {} nodes in {} millis. ({} knodes/sec)", searched, time_taken, nps);
    eprintln!("Found best move {} with score {}.", best_move, best_score);
    return Some(best_move)
}
