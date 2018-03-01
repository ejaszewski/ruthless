pub fn get_move_map(board: &mut board::Board, moves: &mut Vec<Option<u8>>,
                    heuristic: &properties::Heuristic, depth: u8) -> HashMap<Option<u8>, f32> {
    let mut move_map: HashMap<Option<u8>, f32> = HashMap::new();

    for m in moves {
        let undo = board.make_move(*m);
        let (mut score, _leaves) = negamax::negamax(board, heuristic, -10000., 10000., depth);
        board.undo_move(undo, *m);

        score = -score;

        move_map.insert(*m, score);
    }

    move_map
}
