mod model;
use model::game_state::Position;
use model::game_state::GameState;

fn main() {
    let state = GameState::new();

    println!("{}", state.to_string());

    let state_after_move = state.apply_move((Position::new(5, 2), Position::new(5, 4)));

    println!("{}", state_after_move.to_string());

}
