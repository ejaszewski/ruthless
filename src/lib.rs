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
            min_squares: 0,
            max_squares: 64,
            depth: 2,
            bias: 0.0,
            material_weight: 1.0,
            mobility_weight: 0.0,
            square_values: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]
        };

        let light_disks = 0x0000004000000000;
        let dark_disks = 0x000040BC00000000;

        let pos = board::TrainPosition {
            light_disks, dark_disks,
            dark_move: false,
            result: board::GameResult::Unknown
        };

        let mut board = board::Board::from_train_pos(&pos);
        assert_eq!(eval::search::negamax(&mut board, &heuristic).0, Some(9));
    }
}
