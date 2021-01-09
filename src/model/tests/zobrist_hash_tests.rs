
use crate::model::{game_state, move_generator};
use crate::model::game_state::{Move, MoveType, Position, Piece};

#[test]
fn zobrish_hash_is_reversible_from_starting_position() {
    let initial_state = game_state::GameState::new();
    let move_generator = move_generator::MoveGenerator::new();

    let valid_moves = move_generator.generate_moves(&initial_state);

    let mut initial_state_clone = initial_state.clone();

    for next_move in valid_moves {
        assert_eq!(initial_state.zobrist_hash, initial_state_clone.zobrist_hash);
        initial_state_clone.apply_move_mut(next_move);
        assert_ne!(initial_state.zobrist_hash, initial_state_clone.zobrist_hash);
        initial_state_clone.unapply_move_mut(next_move);
        assert_eq!(initial_state.zobrist_hash, initial_state_clone.zobrist_hash);
    }
}