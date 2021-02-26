use crate::model::{game_state, move_generator};
use crate::model::game_state::{Move, MoveType, CastlingRights, Position, Piece};

#[test]
fn test_moves_for_scandinavian_opening_sequence() {
    let e4 = Move { 
        from: Position::new(5, 2),
        to: Position::new(5, 4),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN,
        promotes_to: None,
        last_en_passant: None, 
        last_castling_rights: CastlingRights::initial(),
    };
    let d5 = Move { 
        from: Position::new(4, 7),
        to: Position::new(4, 5),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN,
        promotes_to: None,
        last_en_passant: Some(Position::new(5, 4)),
        last_castling_rights: CastlingRights::initial(),
    };
    
    let exd5 = Move { 
        from: Position::new(5, 4),
        to: Position::new(4, 5),
        move_type: MoveType::Capture(Piece::PAWN),
        moving_piece: Piece::PAWN,
        promotes_to: None, 
        last_en_passant: Some(Position::new(4, 5)),
        last_castling_rights: CastlingRights::initial(),
    };
    
    let qxd5 = Move { 
        from: Position::new(4, 8),
        to: Position::new(4, 5),
        move_type: MoveType::Capture(Piece::PAWN),
        moving_piece: Piece::QUEEN,
        promotes_to: None, 
        last_en_passant: None,
        last_castling_rights: CastlingRights::initial(),
    };

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

#[test]
fn test_opening_sequence_with_en_passant() {
    let e4 = Move { 
        from: Position::new(5, 2),
        to: Position::new(5, 4),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN,
        promotes_to: None, 
        last_en_passant: None,
        last_castling_rights: CastlingRights::initial(),
    };

    let nf6 = Move {
        from: Position::new(7, 8),
        to: Position::new(6, 6),
        move_type: MoveType::Step,
        moving_piece: Piece::KNIGHT,
        promotes_to: None,
        last_en_passant: Some(Position::new(5, 4)),
        last_castling_rights: CastlingRights::initial(),
    };

    let e5 = Move { 
        from: Position::new(5, 4),
        to: Position::new(5, 5),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN,
        promotes_to: None, 
        last_en_passant: None,
        last_castling_rights: CastlingRights::initial(),
    };

    let d5 = Move { 
        from: Position::new(4, 7),
        to: Position::new(4, 5),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN,
        promotes_to: None, 
        last_en_passant: None,
        last_castling_rights: CastlingRights::initial(),
    };
    
    let exd5 = Move { 
        from: Position::new(5, 5),
        to: Position::new(4, 6),
        move_type: MoveType::EnPassant,
        moving_piece: Piece::PAWN, 
        promotes_to: None,
        last_en_passant: Some(Position::new(4, 5)),
        last_castling_rights: CastlingRights::initial(),
    };
    

    let mut game_state = game_state::GameState::new();
    let move_generator = move_generator::MoveGenerator::new();
    let mut valid_moves = move_generator.generate_moves(&game_state);

    assert!(valid_moves.contains(&e4));

    game_state = game_state.apply_move(e4);
    valid_moves = move_generator.generate_moves(&game_state);

    assert!(valid_moves.contains(&nf6));

    game_state = game_state.apply_move(nf6);
    valid_moves = move_generator.generate_moves(&game_state);

    assert!(valid_moves.contains(&e5));

    game_state = game_state.apply_move(e5);
    valid_moves = move_generator.generate_moves(&game_state);

    assert!(valid_moves.contains(&d5));

    game_state = game_state.apply_move(d5);
    valid_moves = move_generator.generate_moves(&game_state);

    assert!(valid_moves.contains(&exd5));
}

#[test]
fn test_castling_move_generation() {
    let mut game_state = game_state::GameState::new();
    let move_generator = move_generator::MoveGenerator::new();
    
    game_state.apply_move_mut(move_generator.get_move(&game_state, Position::new(7, 1), Position::new(6, 3)).unwrap());
    game_state.apply_move_mut(move_generator.get_move(&game_state, Position::new(7, 8), Position::new(6, 6)).unwrap());

    game_state.apply_move_mut(move_generator.get_move(&game_state, Position::new(5, 2), Position::new(5, 3)).unwrap());
    game_state.apply_move_mut(move_generator.get_move(&game_state, Position::new(5, 7), Position::new(5, 6)).unwrap());

    assert!(move_generator.generate_moves(&game_state).iter().all(|m| !matches!(m.move_type, MoveType::Castling)));

    game_state.apply_move_mut(move_generator.get_move(&game_state, Position::new(6, 1), Position::new(5, 2)).unwrap());

    assert!(move_generator.generate_moves(&game_state).iter().all(|m| !matches!(m.move_type, MoveType::Castling)));

    game_state.apply_move_mut(move_generator.get_move(&game_state, Position::new(6, 8), Position::new(5, 7)).unwrap());

    let white_castling_king_side = move_generator.generate_moves(&game_state).into_iter().find(|m| matches!(m.move_type, MoveType::Castling)).unwrap();
    assert_eq!(Position::new(7, 1), white_castling_king_side.to);
    assert_eq!(Position::new(5, 1), white_castling_king_side.from);
    
    game_state.apply_move_mut(move_generator.get_move(&game_state, Position::new(5, 1), Position::new(7, 1)).unwrap());

    let black_castling_king_side = move_generator.generate_moves(&game_state).into_iter().find(|m| matches!(m.move_type, MoveType::Castling)).unwrap();
    assert_eq!(Position::new(7, 8), black_castling_king_side.to);
    assert_eq!(Position::new(5, 8), black_castling_king_side.from);
}