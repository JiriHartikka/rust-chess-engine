use crate::model::game_state;
use crate::model::game_state::Position;
use crate::model::move_generator;

#[test]
fn test_moves_for_scandinavian_opening_sequence() {
    let e4 = (Position::new(5, 2), Position::new(5, 4));
    let d5 = (Position::new(4, 7), Position::new(4, 5));
    let exd5 = (Position::new(5, 4), Position::new(4, 5));
    let qxd5 = (Position::new(4, 8), Position::new(4, 5));

    let mut game_state = game_state::GameState::new();
    let move_generator = move_generator::MoveGenerator::new();
    let mut valid_moves = move_generator.generate_moves(&game_state);

    assert!(valid_moves.contains(&e4));
    assert!(!valid_moves.contains(&d5));

    game_state = game_state.apply_move(e4);
    valid_moves = move_generator.generate_moves(&game_state);

    assert!(!valid_moves.contains(&e4));
    assert!(!valid_moves.contains(&qxd5));
    assert!(valid_moves.contains(&d5));

    game_state = game_state.apply_move(d5);
    valid_moves = move_generator.generate_moves(&game_state);

    assert!(valid_moves.contains(&exd5));

    game_state = game_state.apply_move(exd5);
    valid_moves = move_generator.generate_moves(&game_state);

    assert!(valid_moves.contains(&qxd5));
}