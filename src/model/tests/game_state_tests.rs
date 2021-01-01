use crate::model::game_state;
use crate::model::game_state::Position;
use crate::model::game_state::Piece;
use crate::model::game_state::Color;


#[test]
fn get_pieces() {
    let board = game_state::GameState::new();
    
    let e2 = board.get_piece(Position::new(5, 2));
    let a7 = board.get_piece(Position::new(1, 7));
    let e8 = board.get_piece(Position::new(5, 8));

    assert_eq!((Piece::PAWN, Color::WHITE), e2.unwrap());
    assert_eq!((Piece::PAWN, Color::BLACK), a7.unwrap());
    assert_eq!((Piece::KING, Color::BLACK), e8.unwrap());
}

#[test]
fn get_piece_positions_rook() {
    let board = game_state::GameState::new();

    let white_rooks = board.get_piece_position(Piece::ROOK, Color::WHITE);
    let black_rooks = board.get_piece_position(Piece::ROOK, Color::BLACK);

    assert_eq!(vec![Position::new(1,1), Position::new(8, 1)], white_rooks);
    assert_eq!(vec![Position::new(1, 8), Position::new(8, 8)], black_rooks);
}

#[test]
fn get_piece_positions_pawn() {
    let board = game_state::GameState::new();
    
    let mut expected_white = vec![];
    let mut expected_black = vec![];

    for file in 1..9 {
        expected_white.push(Position::new(file, 2));
        expected_black.push(Position::new(file, 7));
    }

    let white_pawns = board.get_piece_position(Piece::PAWN, Color::WHITE);
    let black_pawns = board.get_piece_position(Piece::PAWN, Color::BLACK);

    assert_eq!(expected_white, white_pawns);
    assert_eq!(expected_black, black_pawns);
}

#[test]
fn get_piece_positions_king() {
    let board = game_state::GameState::new();
    
    let expected_white = vec![Position::new(5, 1)];
    let expected_black = vec![Position::new(5, 8)];

    let white_kings = board.get_piece_position(Piece::KING, Color::WHITE);
    let black_kings = board.get_piece_position(Piece::KING, Color::BLACK);

    assert_eq!(expected_white, white_kings);
    assert_eq!(expected_black, black_kings);
}

#[test]
fn collide() {
    let board = game_state::GameState::new();

    let collision_a1 = board.collide(Position::new(1, 1));
    let collision_b5 = board.collide(Position::new(2, 5));
    let collision_g7 = board.collide(Position::new(7, 7));

    assert_eq!(Option::Some(Color::WHITE), collision_a1);
    assert_eq!(Option::None, collision_b5);
    assert_eq!(Option::Some(Color::BLACK), collision_g7);
}