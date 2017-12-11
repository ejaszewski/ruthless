use ::board;

pub fn negamax(board: &mut board::Board, mut alpha: f32, beta: f32, depth: u8) -> (f32, u64) {
    if depth == 0 { return (super::get_score(board), 1) }
    let mut count = 0;
    for m in &board.get_moves() {

        let undo = board.make_move(Some(*m));
        let result = negamax(board, -beta, -alpha, depth - 1);
        let score = -result.0;
        count += result.1;
        board.undo_move(undo, *m);

        if score >= beta {
            return (beta, count);
        }
        if score > alpha {
            alpha = score;
        }
    }
    return (alpha, count);
}
