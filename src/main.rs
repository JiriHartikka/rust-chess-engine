#[macro_use]
extern crate lazy_static;

mod model;
mod search;
use model::game_state::{GameState};
use model::move_generator::MoveGenerator;
use search::minimax_search::{negamax_alpha_beta_with_trasposition_table};
use search::transposition_table::{TranspositionTable};

fn main() {
    let move_generator = MoveGenerator::new();
    let mut state = GameState::new();
    let initial_state = state.clone();
    let depth = 7;

    let transposition_table = &mut TranspositionTable::with_capacity(1_000_000);

    let (best_move, best_eval, node_count) = negamax_alpha_beta_with_trasposition_table(&mut state, &move_generator, transposition_table, depth);
    assert_eq!(initial_state, state);

    println!("Best move: {:?}", best_move);
    println!("Best eval {}", best_eval);
    println!("At depth {}", depth);
    println!("Visited nodes: {}", node_count);

}
