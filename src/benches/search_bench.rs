#[macro_use]
extern crate bencher;

use rust_chess::model::game_state::GameState;
use rust_chess::model::move_generator::MoveGenerator;
use rust_chess::search::minimax_search::negamax_alpha_beta_with_trasposition_table;
use rust_chess::search::transposition_table::TranspositionTable;

use bencher::Bencher;

fn minimax_search_from_starting_pos(bench: &mut Bencher) {
    let mut game_state = GameState::new();
    let move_generator = MoveGenerator::new();

    let mut transposition_table = TranspositionTable::with_capacity(1);

    bench.iter(|| {
        negamax_alpha_beta_with_trasposition_table(
            &mut game_state,
            &move_generator,
            &mut transposition_table,
            4,
        )
    });
}

benchmark_group!(benches, minimax_search_from_starting_pos);
benchmark_main!(benches);
