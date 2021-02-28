#[macro_use]
extern crate lazy_static;

mod model;
mod search;
mod demo;
use model::game_state::{GameState, Color};
use model::move_generator::MoveGenerator;
use search::minimax_search::{negamax_alpha_beta_with_trasposition_table};
use search::transposition_table::{TranspositionTable};
use demo::chess_agent::ChessAgent;

fn main() {
    /*let move_generator = MoveGenerator::new();
    let mut state = GameState::new();
    let transposition_table = &mut TranspositionTable::with_capacity(1_000_000);
    let depth = 5;

    let (best_move, best_eval, node_count) = negamax_alpha_beta_with_trasposition_table(&mut state, &move_generator, transposition_table, depth);

    println!("Best move: {:?}", best_move);
    println!("Best eval {}", best_eval);
    println!("At depth {}", depth);
    println!("Visited nodes: {}", node_count);*/

    let mut agent = ChessAgent::new(Color::BLACK, 6);

    loop {
        let game_continues = agent.proceed();
        if !game_continues {
            break;
        }
    }

}
