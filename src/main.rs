mod model;
use model::game_state::{Position, GameState, Move, MoveType, Piece};

fn main() {
    let state = GameState::new();

    println!("{}", state.to_string());

    let e4 = Move { 
        from: Position::new(5, 2),
        to: Position::new(5, 4),
        move_type: MoveType::Step,
        moving_piece: Piece::PAWN, 
        last_en_passant: None,
    };

    let state_after_move = state.apply_move(e4);

    println!("{}", state_after_move.to_string());

}
