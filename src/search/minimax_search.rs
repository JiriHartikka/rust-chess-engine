use crate::model::game_state::{GameState, Move, Color};
use crate::model::move_generator::MoveGenerator;
use crate::model::evaluator;
use std::collections::HashMap;

pub type TranspositionTable = HashMap<ZobristKey, TranspositionEntry>;
pub type ZobristKey = u64;

pub struct TranspositionEntry {
    pub evaluation: i32,
    pub depth: u16,
}

pub fn negamax(game_state: &mut GameState, move_generator: &MoveGenerator, depth: u16) -> (Option<Move>, i32, u64) {
    let color_multiplier = if game_state.to_move() == Color::WHITE { 1 } else { -1 };

    if depth == 0 {
        return (None, color_multiplier * evaluator::evaluate(&game_state), 1);
    }

    let next_moves = move_generator.generate_moves(game_state);

    if next_moves.is_empty() {
        return (None, color_multiplier * evaluator::evaluate(&game_state), 1);
    }

    let mut best_move = None;
    let mut best_eval = std::i32::MIN;
    let mut node_count: u64 = 0;

    for next_move in next_moves {
        game_state.apply_move_mut(next_move);
        let (_, eval, child_node_count) = negamax(game_state, move_generator, depth - 1);
        node_count += child_node_count;

        if -eval > best_eval {
            best_eval = -eval;
            best_move = Some(next_move);
        }
        game_state.unapply_move_mut(next_move);
    }

    (best_move, best_eval, node_count + 1)
}

pub fn negamax_with_transposition_table(game_state: &mut GameState, move_generator: &MoveGenerator, table: &mut TranspositionTable, depth: u16) -> (Option<Move>, i32, u64) {    
    let color_multiplier = if game_state.to_move() == Color::WHITE { 1 } else { -1 };

    if let Some(eval) = check_table(table, game_state, depth) {
        return (None, color_multiplier * eval, 0);
    } 

    if depth == 0 {
        let eval = color_multiplier * evaluator::evaluate(&game_state);
        update_table(table, game_state, depth, eval);
        return (None, eval, 1);
    }

    let next_moves = move_generator.generate_moves(game_state);

    if next_moves.is_empty() {
        return (None, color_multiplier * evaluator::evaluate(&game_state), 1);
    }

    let mut best_move = None;
    let mut best_eval = std::i32::MIN;
    let mut node_count: u64 = 0;

    for next_move in next_moves {
        game_state.apply_move_mut(next_move);
        let (_, eval, child_node_count) = negamax_with_transposition_table(game_state, move_generator, table, depth - 1);
        update_table(table, game_state, depth, eval);
        node_count += child_node_count;

        if -eval > best_eval {
            best_eval = -eval;
            best_move = Some(next_move);
        }
        game_state.unapply_move_mut(next_move);
    }

    (best_move, best_eval, node_count + 1)
}

fn check_table(table: &TranspositionTable, game_state: &GameState, depth: u16) -> Option<i32> {
    if let Some(entry) = table.get(&game_state.zobrist_hash) {
        if entry.depth >= depth {
            return Some(entry.evaluation);
        }
    }

    None
}

fn update_table(table: &mut TranspositionTable, game_state: &GameState, depth: u16, evaluation: i32) {
    let entry = TranspositionEntry {
        evaluation: evaluation,
        depth: depth,
    };

    table.insert(game_state.zobrist_hash, entry);
}