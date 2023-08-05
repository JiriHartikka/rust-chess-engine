#[cfg(test)]
use crate::model::game_state::{CastlingRights, Move, MoveType, Piece, Position};
#[cfg(test)]
use crate::model::{game_state, move_generator};

#[test]
fn zobrish_hash_is_reversible_from_starting_position() {
    let initial_state = game_state::GameState::new();
    let move_generator = move_generator::MoveGenerator::new();

    let valid_moves = move_generator.generate_moves(&initial_state);

    let mut initial_state_clone = initial_state.clone();

    for next_move in valid_moves.moves {
        assert_eq!(initial_state.zobrist_hash, initial_state_clone.zobrist_hash);
        initial_state_clone.apply_move_mut(next_move);
        assert_ne!(initial_state.zobrist_hash, initial_state_clone.zobrist_hash);
        initial_state_clone.unapply_move_mut(next_move);
        assert_eq!(initial_state.zobrist_hash, initial_state_clone.zobrist_hash);
    }
}

#[test]
fn zobrist_hash_is_reversible_in_scandinavian_starting_sequence() {
    let scandinavian_opening_sequence = [
        Move {
            from: Position::new(5, 2),
            to: Position::new(5, 4),
            move_type: MoveType::Step,
            moving_piece: Piece::PAWN,
            promotes_to: None,
            last_en_passant: None,
            last_castling_rights: CastlingRights::initial(),
        },
        Move {
            from: Position::new(4, 7),
            to: Position::new(4, 5),
            move_type: MoveType::Step,
            moving_piece: Piece::PAWN,
            promotes_to: None,
            last_en_passant: Some(Position::new(5, 4)),
            last_castling_rights: CastlingRights::initial(),
        },
        Move {
            from: Position::new(5, 4),
            to: Position::new(4, 5),
            move_type: MoveType::Capture(Piece::PAWN),
            moving_piece: Piece::PAWN,
            promotes_to: None,
            last_en_passant: Some(Position::new(4, 5)),
            last_castling_rights: CastlingRights::initial(),
        },
        Move {
            from: Position::new(4, 8),
            to: Position::new(4, 5),
            move_type: MoveType::Capture(Piece::PAWN),
            moving_piece: Piece::QUEEN,
            promotes_to: None,
            last_en_passant: None,
            last_castling_rights: CastlingRights::initial(),
        },
    ];

    let mut state = game_state::GameState::new();

    for m in scandinavian_opening_sequence.iter() {
        let zobrist_before = state.zobrist_hash;
        state.apply_move_mut(*m);
        state.unapply_move_mut(*m);
        let zobrist_after_takeback = state.zobrist_hash;

        println!("{:?}", *m);

        assert_eq!(zobrist_before, zobrist_after_takeback);

        state.apply_move_mut(*m);
    }
}
