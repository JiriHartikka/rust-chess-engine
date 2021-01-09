#[macro_use]
extern crate lazy_static;

mod model;
mod search;
use model::game_state::{GameState};
use model::move_generator::MoveGenerator;
use search::minimax_search::{negamax, negamax_with_transposition_table};
use std::collections::HashMap;

fn main() {
    let move_generator = MoveGenerator::new();
    let mut state = GameState::new();
    let depth = 5;

    let (best_move, best_eval, node_count) = negamax(&mut state, &move_generator, depth);

    println!("Best move: {:?}", best_move);
    println!("Best eval {}", best_eval);
    println!("At depth {}", depth);
    println!("Visited nodes: {}", node_count);


    let transposition_table = &mut HashMap::new();
    transposition_table.reserve(5_000_000);
    let (best_move, best_eval, node_count) = negamax_with_transposition_table(&mut state,  &move_generator, transposition_table, depth);

    println!("Best move: {:?}", best_move);
    println!("Best eval {}", best_eval);
    println!("At depth {}", depth);
    println!("Visited nodes: {}", node_count);
    println!("Table size: {}", transposition_table.len());

}
