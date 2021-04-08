#[cfg(test)]
use crate::uci::uci_utils::parse_move;
#[cfg(test)]
use crate::model::game_state::GameState;
#[cfg(test)]
use crate::model::move_generator::MoveGenerator;

#[cfg(test)]
pub fn apply_position<I>(moves: I, game_state: &mut GameState, move_generator: &MoveGenerator) where I: IntoIterator<Item=String> {
    let parsed_moves = moves.into_iter()
        .map(|m| parse_move(m.as_str()))
        .map(|m| m.unwrap());

    for m in parsed_moves {
        let matching_move = move_generator.get_move(game_state, m.0, m.1).unwrap();
        game_state.apply_move_mut(matching_move);
    }
}