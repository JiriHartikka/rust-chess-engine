use rand;
use rand::{SeedableRng, RngCore};

use super::game_state::{GameState, Piece, Color, Position, Move, MoveType};

const ZOBRIST_SEED: u64 = 123456;

lazy_static! {
    static ref ZOBIRST: ZobristHasher = ZobristHasher::new(ZOBRIST_SEED);
}

pub fn hash(game_state: &GameState) -> u64 {
    ZOBIRST.hash(game_state)
}

pub fn apply_move(current_hash: u64, to_apply: Move, to_move: Color) -> u64 {
    ZOBIRST.apply_move(current_hash, to_apply, to_move)
}

pub fn unapply_move(current_hash: u64, to_unapply: Move, to_move: Color) -> u64 {
    ZOBIRST.unapply_move(current_hash, to_unapply, to_move)
}

struct ZobristHasher {
    pieces: [[u64; 12]; 64],
    en_passant: [u64; 64],
    to_move_white: u64,
    castling_rights: [u64; 4],
}

impl ZobristHasher {
    pub fn new(seed: u64) -> ZobristHasher {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        ZobristHasher {
            pieces: {
                let mut table  = [[0; 12]; 64];

                for i in 0..table.len() {
                    for j in 0..table[i].len() {
                        table[i][j] = rng.next_u64();
                    }
                }
        
                table
            },

            en_passant: {
                let mut table  = [0; 64];

                for i in 0..table.len() {
                    table[i] = rng.next_u64();
                }

                table
            },

            castling_rights: {
                let mut table  = [0; 4];

                for i in 0..table.len() {
                    table[i] = rng.next_u64();
                }

                table
            },

            to_move_white: rng.next_u64(),
        }
    }

    pub fn hash(&self, game_state: &GameState) -> u64 {
        let mut hash = 0;

        for i in 0..64 {
            let maybe_piece_index = game_state
                .get_piece(Position::from_numeric(i))
                .map(|piece| { zobrist_index_for_piece(piece.0, piece.1) });
    
            if let Some(piece_index) = maybe_piece_index {
                hash = hash ^ self.pieces[usize::from(i)][piece_index];
            }
        }
    
        if let Some(en_passant_square) = game_state.en_passant() {
            hash = hash ^ self.en_passant[usize::from(en_passant_square.to_numeric())];
        }
    
        if game_state.to_move() == Color::WHITE {
            hash = hash ^ self.to_move_white;
        }
    
        // TODO castling rights...
    
        hash
    }

    pub fn apply_move(&self, current_hash: u64, to_apply: Move, to_move: Color) -> u64 {
        let moving_piece = to_apply.moving_piece;
        let moving_piece_index = zobrist_index_for_piece(moving_piece, to_move);
        let from_square_index = usize::from(to_apply.from.to_numeric());
        let to_square_index = usize::from(to_apply.to.to_numeric());

        let mut new_hash = current_hash;
        
        match to_apply.move_type {
            MoveType::Capture(captured_piece) => {
                let captured_piece_index = zobrist_index_for_piece(captured_piece, to_move);
                new_hash = new_hash ^ self.pieces[to_square_index][captured_piece_index];
            },
            MoveType::EnPassant => {
                let direction_multiplier = if to_move == Color::WHITE { 1 } else { -1 };
                let captured_square_index = usize::from(to_apply.to.delta(0, -direction_multiplier).unwrap().to_numeric());
                let captured_piece_index = zobrist_index_for_piece(Piece::PAWN, to_move.opposite());
                new_hash = new_hash ^ self.pieces[captured_square_index][captured_piece_index];
            },
            _ => (),
        }

        new_hash = new_hash ^ self.pieces[from_square_index][moving_piece_index];
        new_hash = new_hash ^ self.pieces[to_square_index][moving_piece_index];

        if let Some(en_passant) = to_apply.last_en_passant {
            new_hash = new_hash ^ self.en_passant[usize::from(en_passant.to_numeric())];
        }

        // set en passant bit mask
        if moving_piece == Piece::PAWN {
            let is_two_steps_move = 
                i16::abs(i16::from(to_apply.from.to_numeric()) - i16::from(to_apply.to.to_numeric())) == 16;
            if is_two_steps_move {
                new_hash = new_hash ^ self.en_passant[usize::from(to_apply.to.to_numeric())];
            }
        }

        new_hash = new_hash ^ self.to_move_white;

        new_hash
    }

    pub fn unapply_move(&self, current_hash: u64, to_unapply: Move, to_move: Color) -> u64 {
        let moving_piece = to_unapply.moving_piece;
        let moving_piece_index = zobrist_index_for_piece(moving_piece, to_move.opposite());
        let from_square_index = usize::from(to_unapply.from.to_numeric());
        let to_square_index = usize::from(to_unapply.to.to_numeric());

        let mut new_hash = current_hash;
        
        match to_unapply.move_type {
            MoveType::Capture(captured_piece) => {
                let piece_index = zobrist_index_for_piece(captured_piece, to_move.opposite());
                new_hash = new_hash ^ self.pieces[to_square_index][piece_index];
            },
            MoveType::EnPassant => {
                let direction_multiplier = if to_move.opposite() == Color::WHITE { 1 } else { -1 };
                let captured_square_index = usize::from(to_unapply.to.delta(0, -direction_multiplier).unwrap().to_numeric());
                let captured_piece_index = zobrist_index_for_piece(Piece::PAWN, to_move);
                new_hash = new_hash ^ self.pieces[captured_square_index][captured_piece_index];
            },
            _ => (),
        }

        new_hash = new_hash ^ self.pieces[from_square_index][moving_piece_index];
        new_hash = new_hash ^ self.pieces[to_square_index][moving_piece_index];

        if let Some(en_passant) = to_unapply.last_en_passant {
            new_hash = new_hash ^ self.en_passant[usize::from(en_passant.to_numeric())];
        }

        if moving_piece == Piece::PAWN {
            let is_two_steps_move = 
                i16::abs(i16::from(to_unapply.from.to_numeric()) - i16::from(to_unapply.to.to_numeric())) == 16;
            if is_two_steps_move {
                new_hash = new_hash ^ self.en_passant[usize::from(to_unapply.to.to_numeric())];
            }
        }

        new_hash = new_hash ^ self.to_move_white;

        new_hash
    }
}

fn zobrist_index_for_piece(piece: Piece, color: Color) -> usize {
    match (piece, color) {
        (Piece::PAWN, Color::WHITE) => 0,
        (Piece::KNIGHT, Color::WHITE) => 1,
        (Piece::BISHOP, Color::WHITE) => 2,
        (Piece::ROOK, Color::WHITE) => 3,
        (Piece::QUEEN, Color::WHITE) => 4,
        (Piece::KING, Color::WHITE) => 5,
        (Piece::PAWN, Color::BLACK) => 6,
        (Piece::KNIGHT, Color::BLACK) => 7,
        (Piece::BISHOP, Color::BLACK) => 8,
        (Piece::ROOK, Color::BLACK) => 9,
        (Piece::QUEEN, Color::BLACK) => 10,
        (Piece::KING, Color::BLACK) => 11,
    }
}