use super::game_state::{bit_mask_to_positions, Color, GameState, Piece};

const BASE_VALUE_PAWN: i32 = 1000;
const BASE_VALUE_KNIGT: i32 = 3000;
const BASE_VALUE_BISHOP: i32 = 3000;
const BASE_VALUE_ROOK: i32 = 5000;
const BASE_VALUE_QUEEN: i32 = 9000;
const BASE_VALUE_KING: i32 = 1_000_000;

const POSITION_PAWN: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 0, 10, 10, 0, 5, 5, 10, 25, 5, 100, 100,
    5, 25, 10, 50, 75, 100, 150, 150, 100, 75, 50, 100, 125, 150, 200, 200, 150, 125, 100, 150,
    175, 200, 250, 250, 200, 175, 150, 0, 0, 0, 0, 0, 0, 0, 0,
];

const POSITION_KNIGHT: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 25, 25, 25, 25, 25, 0, 0, 25, 100, 50, 50, 100, 25, 0, 0, 25,
    75, 225, 225, 75, 25, 0, 0, 25, 100, 250, 250, 100, 25, 0, 0, 25, 150, 150, 150, 150, 25, 0, 0,
    25, 25, 25, 25, 25, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const POSITION_BISHOP: [i32; 64] = [
    25, 0, 0, 0, 0, 0, 0, 25, 0, 50, 0, 0, 0, 0, 50, 0, 0, 25, 75, 25, 25, 75, 25, 0, 0, 25, 100,
    150, 150, 100, 25, 0, 0, 75, 100, 150, 150, 100, 75, 0, 0, 25, 25, 25, 25, 25, 25, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const POSITION_ROOK: [i32; 64] = [
    0, 0, 25, 100, 100, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 200, 200, 100, 200, 200, 200, 200, 200,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const POSITION_QUEEN: [i32; 64] = [
    0, 0, 0, 25, 25, 0, 0, 0, 0, 0, 40, 40, 25, 0, 0, 0, 0, 25, 50, 50, 50, 50, 25, 0, 0, 25, 75,
    200, 200, 75, 25, 0, 0, 25, 75, 200, 200, 75, 25, 0, 0, 25, 75, 100, 100, 75, 25, 0, 0, 100,
    125, 125, 125, 125, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const POSITION_KING: [i32; 64] = [
    50, 100, 0, 0, 0, 75, 100, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];

pub fn evaluate(game_state: &GameState) -> i32 {
    let mut evaluation = 0;

    evaluation += evaluate_pawns(game_state);
    evaluation += evaluate_knights(game_state);
    evaluation += evaluate_bishops(game_state);
    evaluation += evaluate_rooks(game_state);
    evaluation += evaluate_queens(game_state);
    evaluation += evaluate_kings(game_state);

    evaluation
}

fn evaluate_pawns(game_state: &GameState) -> i32 {
    evaluate_piece_with_position_modifier(game_state, Piece::PAWN)
}

fn evaluate_knights(game_state: &GameState) -> i32 {
    evaluate_piece_with_position_modifier(game_state, Piece::KNIGHT)
}

fn evaluate_bishops(game_state: &GameState) -> i32 {
    evaluate_piece_with_position_modifier(game_state, Piece::BISHOP)
}

fn evaluate_rooks(game_state: &GameState) -> i32 {
    evaluate_piece_with_position_modifier(game_state, Piece::ROOK)
}

fn evaluate_queens(game_state: &GameState) -> i32 {
    evaluate_piece_with_position_modifier(game_state, Piece::QUEEN)
}

fn evaluate_kings(game_state: &GameState) -> i32 {
    evaluate_piece_with_position_modifier(game_state, Piece::KING)
}

fn evaluate_piece_with_position_modifier(game_state: &GameState, piece: Piece) -> i32 {
    let (base_value, position_bonus) = match piece {
        Piece::PAWN => (BASE_VALUE_PAWN, POSITION_PAWN),
        Piece::KNIGHT => (BASE_VALUE_KNIGT, POSITION_KNIGHT),
        Piece::BISHOP => (BASE_VALUE_BISHOP, POSITION_BISHOP),
        Piece::ROOK => (BASE_VALUE_ROOK, POSITION_ROOK),
        Piece::QUEEN => (BASE_VALUE_QUEEN, POSITION_QUEEN),
        Piece::KING => (BASE_VALUE_KING, POSITION_KING),
    };

    let white_piece_positions =
        bit_mask_to_positions(*game_state.get_piece_mask(piece, Color::WHITE));
    let black_piece_positions =
        bit_mask_to_positions(*game_state.get_piece_mask(piece, Color::BLACK));

    let mut evaluation = 0;

    for piece_position in white_piece_positions {
        evaluation += base_value + position_bonus[usize::from(piece_position.to_numeric())];
    }

    for piece_position in black_piece_positions {
        evaluation -=
            base_value + position_bonus[usize::from(piece_position.mirror_rank().to_numeric())];
    }

    evaluation
}
