use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use crossbeam::thread::scope;

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

    if next_moves.is_checkmate() {
        return (None, -color_multiplier * EVAL_MAX, 1);
    }

    if next_moves.moves.is_empty() {
        return (None, color_multiplier * evaluator::evaluate(&game_state), 1);
    }

    // could order moves here ...

    let mut best_eval = EVAL_MIN;
    let mut best_move = None;
    let mut node_count: u64 = 1;
    let mut current_alpha = alpha;

    for next_move in next_moves.moves {
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

    if next_moves.is_checkmate() {
        return (None, -color_multiplier * EVAL_MAX, 1);
    }

    if next_moves.moves.is_empty() {
        return (None, color_multiplier * evaluator::evaluate(&game_state), 1);
    }

    // could order moves here ...

    let mut best_eval = EVAL_MIN;
    let mut best_move = None;
    let mut node_count: u64 = 1;

    for next_move in next_moves.moves {
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

fn negamax_alpha_beta_with_trasposition_table_and_principal_variation(
    game_state: &mut GameState,
    move_generator: &MoveGenerator,
    table: &mut TranspositionTable,
    principal_move: Option<&Move>,
    depth: u16,
    stop: Arc<Mutex<bool>>) -> (Option<Move>, i32, u64) {
    
    negamax_alpha_beta_with_trasposition_table_and_principal_variation_helper(game_state, move_generator, table, EVAL_MIN, EVAL_MAX, principal_move, depth, depth, stop)
}

fn negamax_alpha_beta_with_trasposition_table_and_principal_variation_helper(
    game_state: &mut GameState,
    move_generator: &MoveGenerator,
    table: &mut TranspositionTable,
    alpha: i32,
    beta: i32,
    principal_move: Option<&Move>,
    depth: u16,
    starting_depth: u16,
    stop: Arc<Mutex<bool>>) -> (Option<Move>, i32, u64) {

    // stop when signaled   
    if *stop.lock().unwrap() {
        return (None, 0, 0);
    }

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

    let mut next_moves = move_generator.generate_moves(game_state);

    if next_moves.is_checkmate() {
        return (None, -color_multiplier * EVAL_MAX, 1);
    }

    if next_moves.moves.is_empty() {
        return (None, color_multiplier * evaluator::evaluate(&game_state), 1);
    }

    //order moves, taking the principal variation first if available
    if let Some(index) = next_moves.moves.iter().position(|m| Some(m) == principal_move) {
        next_moves.moves.swap(0, index);
    }

    let mut best_eval = EVAL_MIN;
    let mut best_move = None;
    let mut node_count: u64 = 1;

    for next_move in next_moves.moves {
        game_state.apply_move_mut(next_move);
        let (_, eval, child_node_count) = negamax_alpha_beta_with_trasposition_table_and_principal_variation_helper(game_state, move_generator, table, -current_beta, -current_alpha, principal_move, depth - 1, starting_depth, stop.clone());
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


pub fn iterative_alpha_beta(game_state: &mut GameState, move_generator: &MoveGenerator, table: &mut TranspositionTable, search_time: Duration) -> (Option<Move>, i32, u16) {
    let start = Instant::now();

    let stop_signal = Arc::new(Mutex::new(false));
    let mut depth = 1;
    let stop_signal_clone = stop_signal.clone();
    let (init_best_move, init_best_eval, _) = negamax_alpha_beta_with_trasposition_table_and_principal_variation(game_state, move_generator, table, None, 1, stop_signal_clone);
    let mut best_move = match init_best_move {
        Some(m) => m,
        None => return (None, init_best_eval, depth),
    };
    let mut best_eval = init_best_eval;


    while start.elapsed() < search_time {
        depth += 1;
        let (sender, receiver) = mpsc::channel();
        let stop_signal_clone = stop_signal.clone();
        let best_move_clone = best_move.clone();

        scope(|s| {
            s.spawn(|_| {
                let sender = sender;
                let (current_best_move, current_best_eval, _) = negamax_alpha_beta_with_trasposition_table_and_principal_variation(game_state, move_generator, table, Some(&best_move_clone), depth, stop_signal_clone);
                sender.send((current_best_move, current_best_eval)).unwrap();
            });

            match receiver.recv_timeout(search_time - start.elapsed()) {
                Ok((cur_best_move, cur_best_eval)) => {
                    if let Some(m) = cur_best_move {
                        best_move = m;
                    }
                    best_eval = cur_best_eval;
                },
                Err(_) => {
                    *stop_signal.lock().unwrap() = true;
                },
            };    

        }).unwrap();
    }
    
    (Some(best_move), best_eval, depth)
}

