use crate::model::game_state::{Move, Piece, Position};
use std::convert::TryFrom;
use std::fmt::Display;

pub struct UciMove(pub Position, pub Position, pub Option<Piece>);

impl Display for UciMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let from_file_repr =
            std::char::from_u32(('a' as u32) + (self.0.file() as u32 - 1)).unwrap();
        let from_rank_repr = self.0.rank().to_string();
        let to_file_repr = std::char::from_u32(('a' as u32) + (self.1.file() as u32 - 1)).unwrap();
        let to_rank_repr = self.1.rank().to_string();
        let promotes_repr = self
            .2
            .map(|piece| match piece {
                Piece::QUEEN => "q",
                Piece::ROOK => "r",
                Piece::BISHOP => "b",
                Piece::KNIGHT => "n",
                _ => panic!(format!("Cannot promote to {:?}", piece)),
            })
            .unwrap_or("");

        write!(
            f,
            "{}{}{}{}{}",
            from_file_repr, from_rank_repr, to_file_repr, to_rank_repr, promotes_repr
        )
    }
}

pub fn move_to_uci(m: &Move) -> UciMove {
    UciMove(m.from, m.to, m.promotes_to)
}

pub fn parse_move(uci_move: &str) -> Result<UciMove, String> {
    if uci_move.len() < 4 || uci_move.len() > 5 {
        return Err("UCI move must be of length 4 or 5".to_string());
    }

    let from = parse_position(
        uci_move.chars().nth(0).unwrap(),
        uci_move.chars().nth(1).unwrap(),
    );
    let to = parse_position(
        uci_move.chars().nth(2).unwrap(),
        uci_move.chars().nth(3).unwrap(),
    );
    let promotes_to = if uci_move.len() == 5 {
        match parse_promotes_to(uci_move.chars().nth(4).unwrap()) {
            Err(msg) => return Err(msg),
            Ok(piece) => Some(piece),
        }
    } else {
        None
    };

    match (from, to) {
        (Ok(from), Ok(to)) => Ok(UciMove(from, to, promotes_to)),
        (Err(msg), Ok(_)) | (Ok(_), Err(msg)) => Err(msg),
        (Err(msg1), Err(msg2)) => Err(format!("{}, {}", msg1, msg2)),
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
        (Err(a), Err(b)) => Err(format!("{}, {}", a, b)),
    }
}

fn parse_promotes_to(promotes_to: char) -> Result<Piece, String> {
    match promotes_to {
        'q' => Ok(Piece::QUEEN),
        'r' => Ok(Piece::ROOK),
        'b' => Ok(Piece::BISHOP),
        'n' => Ok(Piece::KNIGHT),
        _ => Err(format!("Cannot promote to piece: {}", promotes_to)),
    }
}
