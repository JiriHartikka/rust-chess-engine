#[cfg(test)]
use crate::model::game_state::{GameState};
#[cfg(test)]
use crate::model::move_generator::MoveGenerator;
#[cfg(test)]
use crate::search::minimax_search::{negamax_alpha_beta, negamax_alpha_beta_with_trasposition_table};
#[cfg(test)]
use crate::search::transposition_table::{TranspositionTable};
#[cfg(test)]
use crate::uci::uci_utils;


#[test]
fn negamax_does_not_modify_game_state() {
    let move_generator = MoveGenerator::new();
    let mut state = GameState::new();
    let initial_state = state.clone();
    let depth = 4;

    let (_, _, _) = negamax_alpha_beta(&mut state, &move_generator, depth);
    assert_eq!(initial_state, state);
}

#[test]
fn negamax_should_be_deterministic() {
    let move_generator = MoveGenerator::new();
    let mut state = GameState::new();
      
    for depth in 0..=4 {
        let (first_move, first_eval, _) = negamax_alpha_beta(&mut state, &move_generator, depth);
        let (second_move, second_eval, _) = negamax_alpha_beta(&mut state, &move_generator, depth);
     
        assert_eq!(first_eval, second_eval);
        assert_eq!(first_move, second_move);
    }
}

#[test]
fn negamax_with_transposition_table_does_not_modify_game_state() {
    let move_generator = MoveGenerator::new();
    let mut state = GameState::new();
    let initial_state = state.clone();
    let depth = 4;

    let transposition_table = &mut TranspositionTable::with_capacity(10_000);

    let (_, _, _) = negamax_alpha_beta_with_trasposition_table(&mut state, &move_generator, transposition_table, depth);
    assert_eq!(initial_state, state);
}

#[test]
fn negamax_with_transposition_table_should_be_deterministic() {
    let move_generator = MoveGenerator::new();
    let mut state = GameState::new();
    let transposition_table = &mut TranspositionTable::with_capacity(10_000);

    for depth in 0..=4 {
        let (first_move, first_eval, _) = negamax_alpha_beta_with_trasposition_table(&mut state, &move_generator, transposition_table, depth);
        let (second_move, second_eval, _) = negamax_alpha_beta_with_trasposition_table(&mut state, &move_generator, transposition_table, depth);
        
        assert_eq!(first_eval, second_eval);
        assert_eq!(first_move, second_move);
    }
}

#[test]
fn transposition_table_should_not_change_eval() {
    let move_generator = MoveGenerator::new();
    let mut state = GameState::new();
    let transposition_table = &mut TranspositionTable::with_capacity(10_000);

    for depth in 0..=4 {
        let (first_move, first_eval, _) = negamax_alpha_beta(&mut state, &move_generator, depth);
        let (second_move, second_eval, _) = negamax_alpha_beta_with_trasposition_table(&mut state, &move_generator, transposition_table, depth);
        
        assert_eq!(first_eval, second_eval);
        assert_eq!(first_move, second_move);
    }
}


#[test]
fn negamax_finds_fools_mate() {
    let move_generator = MoveGenerator::new();
    let mut state = GameState::new();
    let transposition_table = &mut TranspositionTable::with_capacity(10_000);

    apply_move("f2f3", &move_generator, &mut state);
    apply_move("e7e6", &move_generator, &mut state);
    apply_move("g2g4", &move_generator, &mut state);

    let (first_move, first_eval, _) = negamax_alpha_beta_with_trasposition_table(&mut state, &move_generator, transposition_table, 3);

    assert_eq!("d8h4", uci_utils::move_to_uci(&first_move.unwrap()).to_string());
    assert!(first_eval > 1000000);
}

#[cfg(test)]
fn apply_move(to_apply: &str, move_generator: &MoveGenerator, game_state: &mut GameState) {
    let parsed_move = uci_utils::parse_move(to_apply).unwrap();
    let to_apply = move_generator.generate_moves(game_state).moves.into_iter()
        .find(|m| m.from == parsed_move.0 && m.to == parsed_move.1 && m.promotes_to == parsed_move.2)
        .unwrap();
    game_state.apply_move_mut(to_apply);
}
