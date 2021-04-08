#[cfg(test)]
use crate::model::game_state::GameState;
#[cfg(test)]
use crate::model::move_generator::MoveGenerator;
#[cfg(test)]
use crate::search::minimax_search::negamax_alpha_beta_with_trasposition_table;
#[cfg(test)]
use crate::search::transposition_table::TranspositionTable;
#[cfg(test)]
use crate::uci::uci_utils::parse_move;

#[cfg(test)]
use crate::search::test_utils;


#[test]
fn avoid_checkmate_in_one() {
    let move_sequence: Vec<String> = [
        "e2e4", "c7c5",
        "d1h5", "e7e6", 
        "g1f3", "g8f6", 
        "h5e5", "b8c6",
        "e5f4", "d7d5",
        "e4e5", "f6h5",
        "f4g4", "g7g6",
        "f1b5", "f8g7",
        "e1g1", "e8g8",
        "b5c6", "b7c6",
        "d2d3", "d8c7",
        "g4g5", "h7h6",
        "g5g4", "g7e5",
        "f3e5", "c7e5",
        "c1h6", "f8e8",
        "b1c3", "e5d6",
        "g4h4", "f7f5",
        "b2b4", "c5b4",
        "c3e2", "c8b7",
        "a1e1", "c6c5", 
        "c2c3", "d5d4",
        "c3b4", "d6d5"
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let mut game_state = GameState::new();
    let move_generator = MoveGenerator::new();

    let mut transposition_table = TranspositionTable::with_capacity(10_000);

    test_utils::apply_position(move_sequence, &mut game_state, &move_generator);

    let (best_move, _, _) = negamax_alpha_beta_with_trasposition_table(&mut game_state, &move_generator, &mut transposition_table, 3); 

    game_state.apply_move_mut(best_move.unwrap());

    let uci_move_to_counter = parse_move("d5g2").unwrap();

    let move_to_counter = move_generator.get_move(&game_state, uci_move_to_counter.0, uci_move_to_counter.1);
    
    match move_to_counter {
        // ok, move was prevented
        None => return,
        Some(m) => {
            // if the move is not prevented, the mate should be prevented in any case
            game_state.apply_move_mut(m);
            let generated_moves = move_generator.generate_moves(&game_state);
            assert!(!generated_moves.is_checkmate());
        }
    };

}