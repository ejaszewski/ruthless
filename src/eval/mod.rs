extern crate time;

pub mod search;

use ::board;

// A B C D D C B A
// B B E F F E B B
// C E G F F G E C
// D F F E E F F D
const SCORE_FUNC: [(u64, f32); 7] = [
    (0x81_00_00_00_00_00_00_81, 30.0), // A
    (0x42_C3_00_00_00_00_C3_42,  5.0), // B
    (0x24_00_81_00_00_81_00_24, 15.0), // C
    (0x18_00_00_81_81_00_00_18, 10.0), // D
    (0x00_24_42_00_00_42_24_00,  1.0), // E
    (0x00_18_18_66_66_18_18_00,  2.0), // F
    (0x00_00_24_00_00_24_00_00,  5.0)  // G
];

const MATERIAL_WEIGHT: f32 = 0.1;
const MOBILITY_WEIGHT: f32 = 0.5;

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

pub fn do_search(board: &mut board::Board) -> Option<u8> {

    eprintln!("Current board score: {}", get_score(board));

    let moves = board.get_moves();
    if moves.len() == 0 {
        return None
    }

    let mut searched = 0;
    let start_time = time::now();

    let mut best_move = moves[0];
    let mut score = -10000.;

    for m in &moves {
        let undo = board.make_move(Some(*m));
        let (m_score, leaves) = search::negamax(board, -10000., 10000., 7);
        let m_score = -m_score;
        searched += leaves;
        board.undo_move(undo, *m);
        if m_score > score {
            best_move = *m;
            score = m_score;
        }
    }

    let end_time = time::now();
    let time_taken = (end_time - start_time).num_milliseconds();
    let nps = searched as f32 / time_taken as f32;

    eprintln!("Searched {} nodes in {} millis. ({} knodes/sec)", searched, time_taken, nps);
    eprintln!("Making move {} with score {}.", best_move, score);
    return Some(best_move)
}
