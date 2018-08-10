use std::time::Instant;

use ::board::{ Board, Move };

pub fn endgame_solve_fast(board: &mut Board) -> (i8, Move) {
    let start_time = Instant::now();
    let mut total_nodes = 0;

    let mut moves = board.get_moves();
    moves.sort_unstable_by_key(|&m| board.move_count_after(m));

    let beta = 1;
    let mut best_score = -1;
    let mut best_move = moves[0];

    for m in moves {

        let undo = board.make_move(m);
        let (mut result, nodes) = endgame_solve_fast_impl(board, -beta, -best_score);
        board.undo_move(undo, m);

        total_nodes += nodes;

        if board.black_move {
            result = -result;
        }

        if result >= beta {
            best_move = m;
            best_score = beta;
            break;
        }

        if result > best_score {
            best_move = m;
            best_score = result;
        }
    }

    let end_time = Instant::now();
    let duration = end_time - start_time;
    let time_taken = duration.as_secs() as u32 * 1000 + duration.subsec_millis();
    println!("Searched {} nodes in {} ms.", total_nodes, time_taken);

    return (best_score, best_move);
}

fn endgame_solve_fast_impl(board: &mut Board, mut alpha: i8, beta: i8) -> (i8, u64) {
    if board.is_game_over() {
        let black_disks = board.black_disks.count_ones() as i8;
        let white_disks = board.white_disks.count_ones() as i8;
        if board.black_move {
            return ((black_disks - white_disks).signum(), 1);
        } else {
            return ((white_disks - black_disks).signum(), 1);
        }
    }

    let mut moves = board.get_moves();
    if board.move_count() > 1 && board.all_disks().count_zeros() > 3 {
        moves.sort_unstable_by_key(|&m| board.move_count_after(m));
    }

    let mut total_nodes = 1;

    for m in moves {
        let undo = board.make_move(m);
        let (mut result, nodes) = endgame_solve_fast_impl(board, -beta, -alpha);
        board.undo_move(undo, m);

        result = -result;
        total_nodes += nodes;

        if result > alpha {
            alpha = result;
        }

        if alpha >= beta {
            break;
        }
    }

    (alpha, total_nodes)
}

#[cfg(test)]
mod test {
    use board::Board;

    #[test]
    fn ffo_pos_40() {
        let mut board = Board::from_pos(0x0101312303010100, 0x9E7ECEDCFC1E0800, true);
        assert_eq!(super::endgame_solve_fast(&mut board).0, 1);
    }

    #[test]
    #[ignore]
    fn ffo_pos_41() {
        let mut board = Board::from_pos(0x000200F8642C1800, 0x7C3C7E0618D02472, true);
        assert_eq!(super::endgame_solve_fast(&mut board).0, 0);
    }

    #[test]
    fn ffo_pos_42() {
        let mut board = Board::from_pos(0x000C040486040200, 0x3801FB7B391B1D3C, true);
        assert_eq!(super::endgame_solve_fast(&mut board).0, 1);
    }

    #[test]
    fn ffo_pos_43() {
        let mut board = Board::from_pos(0x3E3C0C1E1C08143E, 0x0000706062F60800, false);
        assert_eq!(super::endgame_solve_fast(&mut board).0, 1);
    }

    #[test]
    fn ffo_pos_44() {
        let mut board = Board::from_pos(0x08081C0E0CC83C1C, 0x222563F1F0340000, false);
        assert_eq!(super::endgame_solve_fast(&mut board).0, 1);
    }
}
