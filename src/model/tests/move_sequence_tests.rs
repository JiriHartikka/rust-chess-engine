#[cfg(test)]
use crate::model::{game_state, move_generator};

#[test]
fn all_moves_from_starting_position_are_revertible() {
    let initial_state = game_state::GameState::new();
    let move_generator = move_generator::MoveGenerator::new();

    let valid_moves = move_generator.generate_moves(&initial_state);

    let mut initial_state_clone = initial_state.clone();

    for next_move in valid_moves.moves {
        assert_eq!(initial_state, initial_state_clone);
        initial_state_clone.apply_move_mut(next_move);
        assert_ne!(initial_state, initial_state_clone);
        initial_state_clone.unapply_move_mut(next_move);
        assert_eq!(initial_state, initial_state_clone);
    }

}