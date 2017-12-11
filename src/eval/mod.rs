pub mod search;

use ::board;

pub fn get_score(board: &board::Board) -> f32 {
    if board.dark_move {
        board.dark_disks.count_ones() as f32 - board.light_disks.count_ones() as f32
    } else {
        board.light_disks.count_ones() as f32 - board.dark_disks.count_ones() as f32
    }
}

pub fn do_search(board: &mut board::Board) -> Option<u8> {
    let moves = board.get_moves();
    if moves.len() == 0 {
        return None
    }
    let mut best_move = moves[0];
    let mut score = -10000.;
    for m in &moves {
        let undo = board.make_move(*m);
        let m_score = search::negamax(board, -10000., 10000., 3);
        board.undo_move(undo, *m);
        if m_score > score {
            best_move = *m;
            score = m_score;
        }
    }
    eprintln!("Making move {} with score {}.", best_move, score);
    return Some(best_move)
}
