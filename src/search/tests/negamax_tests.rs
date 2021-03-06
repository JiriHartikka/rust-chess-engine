#[cfg(test)]
use crate::model::game_state::{GameState};
#[cfg(test)]
use crate::model::move_generator::MoveGenerator;
#[cfg(test)]
use crate::search::minimax_search::{negamax_alpha_beta, negamax_alpha_beta_with_trasposition_table};
#[cfg(test)]
use crate::search::transposition_table::{TranspositionTable};

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