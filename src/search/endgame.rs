use std::time::Instant;

use ::board::{ Board, Move };

pub fn endgame_solve(board: &mut Board, wld: bool) -> (i8, Move) {
    let start_time = Instant::now();
    let mut total_nodes = 0;

    let mut moves = board.get_moves();
    moves.sort_unstable_by_key(|&m| board.move_count_after(m));

    let beta = if wld { 1 } else { 64 };
    let mut best_score = -beta;
    let mut best_move = moves[0];

    for m in moves {
        let undo = board.make_move(m);
        let (mut result, nodes) = endgame_negamax(board, -beta, -best_score, wld);
        board.undo_move(undo, m);

        total_nodes += nodes;

        result = -result;

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

    println!("[{}] Searched {} nodes in {} ms.", if wld { "WLD" } else { "FULL" }, total_nodes, time_taken);

    return (best_score, best_move);
}

fn endgame_negamax(board: &mut Board, mut alpha: i8, beta: i8, wld: bool) -> (i8, u64) {
    if board.is_game_over() {
        let score = if board.black_move { board.get_score() } else { -board.get_score() };
        if wld {
            return (score.signum(), 1);
        } else {
            return (score, 1);
        }
    }

    let mut moves = board.get_moves();
    if board.move_count() > 1 && board.all_disks().count_zeros() > 3 {
        moves.sort_unstable_by_key(|&m| board.move_count_after(m));
    }

    let mut total_nodes = 1;

    for m in moves {
        let undo = board.make_move(m);
        let (mut result, nodes) = endgame_negamax(board, -beta, -alpha, wld);
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
    use board::{ Board, Move };

    fn move_in_list(moves: Vec<u8>, m: Move) -> bool {
        moves.iter().any(|&x| Move::Play(x) == m)
    }

    #[test]
    fn ffo_pos_40_wld() {
        let mut board = Board::from_pos(0x0101312303010100, 0x9E7ECEDCFC1E0800, true);

        assert_eq!(super::endgame_solve(&mut board, true).0, 1);
    }

    #[test]
    fn ffo_pos_40_exact() {
        let mut board = Board::from_pos(0x0101312303010100, 0x9E7ECEDCFC1E0800, true);

        let (score, m) = super::endgame_solve(&mut board, false);
        assert_eq!(score, 38);
        assert!(move_in_list(vec![ 8 ], m));
    }

    #[test]
    fn ffo_pos_41_wld() {
        let mut board = Board::from_pos(0x000200F8642C1800, 0x7C3C7E0618D02472, true);

        assert_eq!(super::endgame_solve(&mut board, true).0, 0);
    }

    #[test]
    #[ignore]
    fn ffo_pos_41_exact() {
        let mut board = Board::from_pos(0x000200F8642C1800, 0x7C3C7E0618D02472, true);

        let (score, m) = super::endgame_solve(&mut board, false);
        assert_eq!(score, 0);
        assert!(move_in_list(vec![ 31 ], m));
    }

    #[test]
    fn ffo_pos_42_wld() {
        let mut board = Board::from_pos(0x000C040486040200, 0x3801FB7B391B1D3C, true);

        assert_eq!(super::endgame_solve(&mut board, true).0, 1);
    }

    #[test]
    #[ignore]
    fn ffo_pos_42_exact() {
        let mut board = Board::from_pos(0x000C040486040200, 0x3801FB7B391B1D3C, true);

        let (score, m) = super::endgame_solve(&mut board, false);
        assert_eq!(score, 6);
        assert!(move_in_list(vec![ 14 ], m));
    }

    #[test]
    fn ffo_pos_43_wld() {
        let mut board = Board::from_pos(0x3E3C0C1E1C08143E, 0x0000706062F60800, false);

        assert_eq!(super::endgame_solve(&mut board, true).0, -1);
    }

    #[test]
    #[ignore]
    fn ffo_pos_43_exact() {
        let mut board = Board::from_pos(0x3E3C0C1E1C08143E, 0x0000706062F60800, false);

        let (score, m) = super::endgame_solve(&mut board, false);
        assert_eq!(score, -12);
        assert!(move_in_list(vec![ 50, 22 ], m));
    }

    #[test]
    fn ffo_pos_44_wld() {
        let mut board = Board::from_pos(0x08081C0E0CC83C1C, 0x222563F1F0340000, false);

        assert_eq!(super::endgame_solve(&mut board, true).0, -1);
    }

    #[test]
    #[ignore]
    fn ffo_pos_44_exact() {
        let mut board = Board::from_pos(0x08081C0E0CC83C1C, 0x222563F1F0340000, false);

        let (score, m) = super::endgame_solve(&mut board, false);
        assert_eq!(score, -14);
        assert!(move_in_list(vec![ 57, 11 ], m));
    }

    #[test]
    #[ignore]
    fn ffo_pos_46_wld() {
        let mut board = Board::from_pos(0x1C04060703173078, 0x003838783C280C02, true);

        assert_eq!(super::endgame_solve(&mut board, true).0, -1);
    }

    #[test]
    #[ignore]
    fn ffo_pos_46_exact() {
        let mut board = Board::from_pos(0x1C04060703173078, 0x003838783C280C02, true);

        let (score, m) = super::endgame_solve(&mut board, false);
        assert_eq!(score, -8);
        assert!(move_in_list(vec![ 17 ], m));
    }
}
