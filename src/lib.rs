#[macro_use]
extern crate serde_derive;

pub mod board;
pub mod eval;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimax() {
        let heuristic = eval::properties::Heuristic {
            id: String::from("stupid"),
            min_squares: 0,
            max_squares: 64,
            depth: 2,
            bias: 0.0,
            mobility_weight: 0.0,
            material_weight: 1.0,
            mobility_values: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            unstable_material_values: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
            stable_material_values: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
        };

        let light_disks = 0x0000004000000000;
        let dark_disks = 0x000040BC00000000;

        let pos = board::TrainPosition {
            light_disks, dark_disks,
            dark_move: false,
            score: 0f32
        };

        let mut board = board::Board::from_train_pos(&pos);
        assert_eq!(eval::search::negamax(&mut board, &heuristic).0, Some(9));
    }

    #[test]
    fn ffo_40_fast() {
        let light_disks = 0x9E7ECEDCFC1E0800;
        let dark_disks = 0x0101312303010100;

        let pos = board::TrainPosition {
            light_disks, dark_disks,
            dark_move: true,
            score: 0f32
        };

        let mut board = board::Board::from_train_pos(&pos);

        let (best_move, score) = eval::search::endgame_solve_fast(&mut board);

        assert_eq!(score, 1);
    }

    #[test]
    fn ffo_40_full() {
        let light_disks = 0x9E7ECEDCFC1E0800;
        let dark_disks = 0x0101312303010100;

        let pos = board::TrainPosition {
            light_disks, dark_disks,
            dark_move: true,
            score: 0f32
        };

        let mut board = board::Board::from_train_pos(&pos);

        let (best_move, score) = eval::search::endgame_solve_full(&mut board);

        assert_eq!(score, 38);
        assert!(best_move == Some(8));
    }

    #[test]
    fn ffo_41_full() {
        let light_disks = 0x7C3C7E0618D02472;
        let dark_disks = 0x000200F8642C1800;

        let pos = board::TrainPosition {
            light_disks, dark_disks,
            dark_move: true,
            score: 0f32
        };

        let mut board = board::Board::from_train_pos(&pos);

        let (best_move, score) = eval::search::endgame_solve_full(&mut board);

        assert_eq!(score, 0);
        assert!(best_move == Some(31));
    }

    #[test]
    fn ffo_41_fast() {
        let light_disks = 0x7C3C7E0618D02472;
        let dark_disks = 0x000200F8642C1800;

        let pos = board::TrainPosition {
            light_disks, dark_disks,
            dark_move: true,
            score: 0f32
        };

        let mut board = board::Board::from_train_pos(&pos);

        let (best_move, score) = eval::search::endgame_solve_fast(&mut board);

        assert_eq!(score, 0);
    }

    #[test]
    fn ffo_44_full() {
        let light_disks = 0x222563F1F0340000;
        let dark_disks = 0x08081C0E0CC83C1C;

        let pos = board::TrainPosition {
            light_disks, dark_disks,
            dark_move: false,
            score: 0f32
        };

        let mut board = board::Board::from_train_pos(&pos);

        let (best_move, score) = eval::search::endgame_solve_full(&mut board);

        assert_eq!(score, -14);
        assert!(best_move == Some(57) || best_move == Some(11));
    }

    #[test]
    fn ffo_44_fast() {
        let light_disks = 0x222563F1F0340000;
        let dark_disks = 0x08081C0E0CC83C1C;

        let pos = board::TrainPosition {
            light_disks, dark_disks,
            dark_move: false,
            score: 0f32
        };

        let mut board = board::Board::from_train_pos(&pos);

        let (best_move, score) = eval::search::endgame_solve_fast(&mut board);

        assert_eq!(score, -1);
    }

}
