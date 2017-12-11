extern crate time;

pub mod search;

use ::board;

pub fn get_score(board: &board::Board) -> f32 {
    if board.dark_move {
        let eval = board.dark_disks.count_ones() as f32 - board.light_disks.count_ones() as f32;
        eprintln!("Dark Evaluation: {}", eval);
        eval
    } else {
        let eval = board.light_disks.count_ones() as f32 - board.dark_disks.count_ones() as f32;
        eprintln!("Light Evaluation: {}", eval);
        eval
    }
}

pub fn do_search(board: &mut board::Board) -> Option<u8> {
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
        let (m_score, leaves) = search::negamax(board, -10000., 10000., 2);
        let m_score = -m_score;
        searched += leaves;
        board.undo_move(undo, *m);
        if m_score > score {
            best_move = *m;
            score = m_score;
        }
    }

    let end_time = time::now();
    let time_taken: time::Duration = end_time - start_time;

    eprintln!("Making move {} with score {}.", best_move, score);
    eprintln!("Searched {} nodes in {} millis.", searched, time_taken.num_milliseconds());
    return Some(best_move)
}
