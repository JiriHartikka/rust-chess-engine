use crate::model::game_state::{Color, GameState, Move, Position};
use crate::model::move_generator::MoveGenerator;
use crate::search::transposition_table::TranspositionTable;
use crate::search::minimax_search::negamax_alpha_beta_with_trasposition_table;

use std::io::{self, BufRead, Write};
use std::convert::TryFrom;

pub struct Game {
    ai_color: Color,
    game_state: GameState,
    move_generator: MoveGenerator,
    transposition_table: TranspositionTable,
    search_depth: u16,
}

impl Game {
    pub fn new(ai_color: Color, depth: u16) -> Self {
        let game_state = GameState::new();
        let transposition_table = TranspositionTable::with_capacity(1_000_000);
        let move_generator = MoveGenerator::new();

        Game {
            ai_color: ai_color,
            game_state: game_state,
            move_generator: move_generator,
            transposition_table: transposition_table,
            search_depth: depth,
        }
    }

    pub fn proceed(&mut self) -> bool {
        println!("{:?}", self.game_state);

        if self.game_state.to_move() == self.ai_color {
            let (maybe_best_move, _, _) = negamax_alpha_beta_with_trasposition_table(&mut self.game_state, &self.move_generator, &mut self.transposition_table, self.search_depth);
            if let Some(best_move) = maybe_best_move {
                self.game_state.apply_move_mut(best_move);
            } else {
                println!("Game over");
                return false;
            }            
        } else {
            let player_move = self.read_player_move();
            self.game_state.apply_move_mut(player_move);
        }

        true
    }
    
    fn read_player_move(&self) -> Move {

        loop {
            print!("Your move:");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().lock().read_line(&mut line).unwrap();
            let mut split = line.split_whitespace();
        
            let (maybe_pos1_raw, maybe_pos2_raw) = (split.next(), split.next());

            let (pos1_raw, pos2_raw) = match (maybe_pos1_raw, maybe_pos2_raw) {
                (Some(a), Some(b)) => (a, b),
                (_, _) => continue,
            };
            
            let pos1 = Game::parse_position(
                pos1_raw.chars().nth(0).unwrap(),
                pos1_raw.chars().nth(1).unwrap()
            );

            let pos2 = Game::parse_position(
                pos2_raw.chars().nth(0).unwrap(),
                pos2_raw.chars().nth(1).unwrap()
            );

            match (pos1, pos2) {
                (Ok(from), Ok(to)) => {
                    match self.move_generator.get_move(&self.game_state, from, to) {
                        Some(next_move) => return next_move,
                        _ => {
                            println!("Move is not valid");
                            continue;
                        }
                    }

                },
                (_, _) => {
                    println!("Bad coordinates");
                    continue;
                }
            }
        }
    }

    fn parse_position(file_raw: char, rank_raw: char) -> Result<Position, String> {
        let file = match file_raw {
            'a' => Ok(1),
            'b' => Ok(2),
            'c' => Ok(3),
            'd' => Ok(4),
            'e' => Ok(5),
            'f' => Ok(6),
            'g' => Ok(7),
            'h' => Ok(8),
            _ => Err("Invalid file"),
        };

        let rank = rank_raw
            .to_digit(10)
            .ok_or("Invalid rank")
            .and_then(|digit| u8::try_from(digit).map_err(|_| "Cannot convert from u32 to u8"));


        match (file, rank) {
            (Ok(file), Ok(rank)) => return Ok(Position::new(file, rank)),
            (Err(a), Ok(_)) => Err(a.to_string()),
            (Ok(_), Err(a)) => Err(a.to_string()),
            (Err(a), Err(b)) => Err(format!("{} and {}", a, b)),
        }
    }

}