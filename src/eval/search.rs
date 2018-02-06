use ::board::Board;
use ::eval::properties::Properties;

pub fn negamax(board: &mut Board, props: &Properties, mut alpha: f32, beta: f32, depth: u8) -> (f32, u64) {
    if depth == 0 { return (super::get_score(board), 1) }
    let mut count = 0;
    for m in &board.get_moves() {

        let undo = board.make_move(*m);
        let result = negamax(board, props, -beta, -alpha, depth - 1);
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

pub fn negamax_endgame(board: &mut Board, mut alpha: i8, beta: i8) -> (i8, u64) {
    let moves = &board.get_moves();
    if moves[0] == None {
        return (super::get_score_endgame_solve(board), 1);
    }
    let mut count = 0;
    for m in moves {
        let undo = board.make_move(*m);
        let result = negamax_endgame(board, -beta, -alpha);
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
