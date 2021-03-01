#[macro_use]
extern crate lazy_static;

mod model;
mod search;
mod demo;
use model::game_state::Color;
use demo::cmdline_game::Game;

fn main() {
    let mut game = Game::new(Color::BLACK, 5);

    loop {
        let game_continues = game.proceed();
        if !game_continues {
            break;
        }
    }

}
