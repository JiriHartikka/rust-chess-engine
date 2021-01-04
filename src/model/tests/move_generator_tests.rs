use crate::model::{game_state, move_generator};
use crate::model::game_state::{Move, MoveType, Position, Piece};

#[test]
fn test_moves_for_scandinavian_opening_sequence() {
    let e4 = Move { 
        from: Position::new(5, 2),
        to: Position::new(5, 4),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN,
        last_en_passant: None, 
    };
    let d5 = Move { 
        from: Position::new(4, 7),
        to: Position::new(4, 5),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN,
        last_en_passant: Some(Position::new(5, 4)),
    };
    
    let exd5 = Move { 
        from: Position::new(5, 4),
        to: Position::new(4, 5),
        move_type: MoveType::Capture(Piece::PAWN),
        moving_piece: Piece::PAWN, 
        last_en_passant: Some(Position::new(4, 5)),
    };
    
    let qxd5 = Move { 
        from: Position::new(4, 8),
        to: Position::new(4, 5),
        move_type: MoveType::Capture(Piece::PAWN),
        moving_piece: Piece::QUEEN, 
        last_en_passant: None,
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
        last_en_passant: None,
    };

    let nf6 = Move {
        from: Position::new(7, 8),
        to: Position::new(6, 6),
        move_type: MoveType::Step,
        moving_piece: Piece::KNIGHT,
        last_en_passant: Some(Position::new(5, 4)),
    };

    let e5 = Move { 
        from: Position::new(5, 4),
        to: Position::new(5, 5),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN, 
        last_en_passant: None,
    };

    let d5 = Move { 
        from: Position::new(4, 7),
        to: Position::new(4, 5),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN, 
        last_en_passant: None,
    };
    
    let exd5 = Move { 
        from: Position::new(5, 5),
        to: Position::new(4, 6),
        move_type: MoveType::EnPassant,
        moving_piece: Piece::PAWN, 
        last_en_passant: Some(Position::new(4, 5)),
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