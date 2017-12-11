use ::board;

pub fn negamax(board: &mut board::Board, mut alpha: f32, beta: f32, depth: u8) -> f32 {
    if depth == 0 { return super::get_score(board) }
    for m in &board.get_moves() {

        let undo = board.make_move(*m);
        let score = -negamax(board, -beta, -alpha, depth - 1);
        board.undo_move(undo, *m);

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    return alpha;
}
