#[macro_use]
extern crate bencher;

use rust_chess::model::game_state::GameState;
use rust_chess::model::move_generator::MoveGenerator;
use rust_chess::search::test_utils;

use bencher::Bencher;

fn move_generation_from_starting_pos(bench: &mut Bencher) {
    let game_state = GameState::new();
    let move_generator = MoveGenerator::new();

    bench.iter(|| move_generator.generate_moves(&game_state));
}

fn move_generation_from_midgame_pos(bench: &mut Bencher) {
    let move_sequence: Vec<String> = [
        "e2e4", "c7c5", "d1h5", "e7e6", "g1f3", "g8f6", "h5e5", "b8c6", "e5f4", "d7d5", "e4e5",
        "f6h5", "f4g4", "g7g6", "f1b5", "f8g7", "e1g1", "e8g8", "b5c6", "b7c6", "d2d3", "d8c7",
        "g4g5", "h7h6", "g5g4",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let mut game_state = GameState::new();
    let move_generator = MoveGenerator::new();

    test_utils::apply_position(move_sequence, &mut game_state, &move_generator);

    bench.iter(|| move_generator.generate_moves(&game_state));
}

benchmark_group!(
    benches,
    move_generation_from_starting_pos,
    move_generation_from_midgame_pos
);
benchmark_main!(benches);
