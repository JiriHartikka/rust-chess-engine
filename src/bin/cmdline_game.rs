use rust_chess::demo::cmdline_game::Game;
use rust_chess::model::game_state::Color;

fn main() {
    let mut game = Game::new(Color::BLACK, 5);

    loop {
        let game_continues = game.proceed();
        if !game_continues {
            break;
        }
    }
}
