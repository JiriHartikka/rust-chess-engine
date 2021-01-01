use crate::model::{game_state, move_generator};
use crate::model::game_state::{Move, MoveType, Position, Piece};

#[test]
fn test_moves_for_scandinavian_opening_sequence() {
    let e4 = Move { 
        from: Position::new(5, 2),
        to: Position::new(5, 4),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN, 
    };
    let d5 = Move { 
        from: Position::new(4, 7),
        to: Position::new(4, 5),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN, 
    };
    
    let exd5 = Move { 
        from: Position::new(5, 4),
        to: Position::new(4, 5),
        move_type: MoveType::Capture(Piece::PAWN),
        moving_piece: Piece::PAWN, 
    };
    
    let qxd5 = Move { 
        from: Position::new(4, 8),
        to: Position::new(4, 5),
        move_type: MoveType::Capture(Piece::PAWN),
        moving_piece: Piece::QUEEN, 
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