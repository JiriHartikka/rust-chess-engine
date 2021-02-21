use crate::model::game_state::{GameState, Move, Color};
use crate::model::move_generator::MoveGenerator;
use crate::model::evaluator;
use crate::search::transposition_table::{TranspositionTable, MatchType};


// we rely on the invariance of the expression EVAL_MIN = -EVAL_MAX 
// which does not hold for i32::MAX and i32::MIN 
const EVAL_MAX: i32 = i32::MAX;
const EVAL_MIN: i32 = -EVAL_MAX;


pub fn negamax_alpha_beta(game_state: &mut GameState, move_generator: &MoveGenerator, depth: u16) -> (Option<Move>, i32, u64) {
    negamax_alpha_beta_helper(game_state, move_generator, EVAL_MIN, EVAL_MAX, depth)
}

fn negamax_alpha_beta_helper(game_state: &mut GameState, move_generator: &MoveGenerator, alpha: i32, beta: i32, depth: u16) -> (Option<Move>, i32, u64) {
    let color_multiplier = if game_state.to_move() == Color::WHITE { 1 } else { -1 };

    if depth == 0 {
        return (None, color_multiplier * evaluator::evaluate(game_state), 1);
    }

    let next_moves = move_generator.generate_moves(game_state);

    if next_moves.is_empty() {
        return (None, color_multiplier * evaluator::evaluate(&game_state), 1);
    }

    // could order moves here ...

    let mut best_eval = EVAL_MIN;
    let mut best_move = None;
    let mut node_count: u64 = 1;
    let mut current_alpha = alpha;

    for next_move in next_moves {
        game_state.apply_move_mut(next_move);
        let (_, eval, child_node_count) = negamax_alpha_beta_helper(game_state, move_generator, -beta, -current_alpha, depth - 1);
        game_state.unapply_move_mut(next_move);
    
        node_count += child_node_count;

        if -eval > best_eval {
            best_eval = -eval;
            best_move = Some(next_move);
        }

        current_alpha = i32::max(current_alpha, best_eval);
        
        if current_alpha >= beta {
            break;
        }
    }

    (best_move, best_eval, node_count)
}

pub fn negamax_alpha_beta_with_trasposition_table(game_state: &mut GameState, move_generator: &MoveGenerator, table: &mut TranspositionTable, depth: u16) -> (Option<Move>, i32, u64) {
    negamax_alpha_beta_with_trasposition_table_helper(game_state, move_generator, table, EVAL_MIN, EVAL_MAX, depth, depth)
}

fn negamax_alpha_beta_with_trasposition_table_helper(game_state: &mut GameState, move_generator: &MoveGenerator, table: &mut TranspositionTable, alpha: i32, beta: i32, depth: u16, starting_depth: u16) -> (Option<Move>, i32, u64) {
    let color_multiplier = if game_state.to_move() == Color::WHITE { 1 } else { -1 };

    let mut current_alpha = alpha;
    let mut current_beta = beta;

    // on starting depth, do not check transposition table, because we need the move, not only the eval
    if depth != starting_depth {
        // transposition table lookup
        if let Some((match_type, eval)) = table.check(game_state.zobrist_hash, depth) {
            match match_type {
                MatchType::EXACT => {
                    return (None, eval, 0);        
                },
                MatchType::LOWERBOUND => {
                    current_alpha = i32::max(current_alpha, eval);
                },
                MatchType::UPPERBOUND => {
                    current_beta = i32::min(current_beta, eval);
                },
            }

            if current_alpha >= current_beta {
                return (None, eval, 0)
            }
        }
    }
    
    
    if depth == 0 {
        let eval = color_multiplier * evaluator::evaluate(&game_state);
        return (None, eval, 1);
    }

    let next_moves = move_generator.generate_moves(game_state);

    if next_moves.is_empty() {
        return (None, color_multiplier * evaluator::evaluate(&game_state), 1);
    }

    // could order moves here ...

    let mut best_eval = EVAL_MIN;
    let mut best_move = None;
    let mut node_count: u64 = 1;

    for next_move in next_moves {
        game_state.apply_move_mut(next_move);
        let (_, eval, child_node_count) = negamax_alpha_beta_with_trasposition_table_helper(game_state, move_generator, table, -current_beta, -current_alpha, depth - 1, starting_depth);
        game_state.unapply_move_mut(next_move);
        
        node_count += child_node_count;

        if -eval > best_eval {
            best_eval = -eval;
            best_move = Some(next_move);
        }

        current_alpha = i32::max(current_alpha, best_eval);
        
        if current_alpha >= current_beta {
            break;
        }
    }

    let match_type = if best_eval <= alpha {
        MatchType::UPPERBOUND
    } else if best_eval >= current_beta {
        MatchType::LOWERBOUND
    } else {
        MatchType::EXACT
    };

    table.update(game_state.zobrist_hash, depth, best_eval, match_type);

    (best_move, best_eval, node_count)
}
