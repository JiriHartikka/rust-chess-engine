use rand;
use rand::{SeedableRng, RngCore};

use super::game_state::{GameState, Piece, Color, Position, Move, MoveType, CastlingRights};

const ZOBRIST_SEED: u64 = 123456;

const WHITE_KING_SIDE: usize = 0;
const WHITE_QUEEN_SIDE: usize = 1;
const BLACK_KING_SIDE: usize = 2;
const BLACK_QUEEN_SIDE: usize = 3;

lazy_static! {
    static ref ZOBIRST: ZobristHasher = ZobristHasher::new(ZOBRIST_SEED);
}

pub fn hash(game_state: &GameState) -> u64 {
    ZOBIRST.hash(game_state)
}

pub fn apply_move(current_hash: u64, castling_rights: &CastlingRights, to_apply: Move, to_move: Color) -> u64 {
    let next_hash = ZOBIRST.apply_move(current_hash, to_apply, to_move);
    ZOBIRST.apply_castling_rights(next_hash, *castling_rights, to_apply, to_move)
}

pub fn unapply_move(current_hash: u64, castling_rights: &CastlingRights, to_unapply: Move, to_move: Color) -> u64 {
    let next_hash = ZOBIRST.unapply_move(current_hash, to_unapply, to_move);
    ZOBIRST.unapply_castling_rights(next_hash, *castling_rights, to_unapply, to_move)
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
                new_hash ^= self.pieces[to_square_index][captured_piece_index];
            },
            MoveType::EnPassant => {
                let direction_multiplier = if to_move == Color::WHITE { 1 } else { -1 };
                let captured_square_index = usize::from(to_apply.to.delta(0, -direction_multiplier).unwrap().to_numeric());
                let captured_piece_index = zobrist_index_for_piece(Piece::PAWN, to_move.opposite());
                new_hash = new_hash ^ self.pieces[captured_square_index][captured_piece_index];
            },
            MoveType::Castling => {
                let rank = to_apply.from.rank();
                let (old_rook_file, new_rook_file) = if to_apply.to.file() < 5 { (1, 4) } else { (8, 6) };
                let old_rook_position_index = usize::from(Position::new(old_rook_file, rank).to_numeric());
                let new_rook_position_index = usize::from(Position::new(new_rook_file, rank).to_numeric());
                let rook_piece_index = zobrist_index_for_piece(Piece::ROOK, to_move);
                new_hash ^= self.pieces[old_rook_position_index][rook_piece_index];
                new_hash ^= self.pieces[new_rook_position_index][rook_piece_index];
            },
            _ => (),
        }

        new_hash ^= self.pieces[from_square_index][moving_piece_index];

        match to_apply.promotes_to {
            None => {
                new_hash ^= self.pieces[to_square_index][moving_piece_index];
            },
            Some(piece) => {
                new_hash ^= self.pieces[to_square_index][zobrist_index_for_piece(piece, to_move)];
            },
        }

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
                new_hash ^= self.pieces[to_square_index][piece_index];
            },
            MoveType::EnPassant => {
                let direction_multiplier = if to_move.opposite() == Color::WHITE { 1 } else { -1 };
                let captured_square_index = usize::from(to_unapply.to.delta(0, -direction_multiplier).unwrap().to_numeric());
                let captured_piece_index = zobrist_index_for_piece(Piece::PAWN, to_move);
                new_hash ^= self.pieces[captured_square_index][captured_piece_index];
            },
            MoveType::Castling => {
                let rank = to_unapply.from.rank();
                let (old_rook_file, new_rook_file) = if to_unapply.to.file() < 5 { (1, 4) } else { (8, 6) };
                let old_rook_position_index = usize::from(Position::new(old_rook_file, rank).to_numeric());
                let new_rook_position_index = usize::from(Position::new(new_rook_file, rank).to_numeric());
                let rook_piece_index = zobrist_index_for_piece(Piece::ROOK, to_move.opposite());
                new_hash ^= self.pieces[old_rook_position_index][rook_piece_index];
                new_hash ^= self.pieces[new_rook_position_index][rook_piece_index];
            },
            _ => (),
        }

        new_hash ^= self.pieces[from_square_index][moving_piece_index];


        match to_unapply.promotes_to {
            None => {
                new_hash ^= self.pieces[to_square_index][moving_piece_index];
            },
            Some(piece) => {
                new_hash ^= self.pieces[to_square_index][zobrist_index_for_piece(piece, to_move.opposite())];
            },
        }

        if let Some(en_passant) = to_unapply.last_en_passant {
            new_hash ^= self.en_passant[usize::from(en_passant.to_numeric())];
        }

        if moving_piece == Piece::PAWN {
            let is_two_steps_move = 
                i16::abs(i16::from(to_unapply.from.to_numeric()) - i16::from(to_unapply.to.to_numeric())) == 16;
            if is_two_steps_move {
                new_hash ^= self.en_passant[usize::from(to_unapply.to.to_numeric())];
            }
        }

        new_hash = new_hash ^ self.to_move_white;

        new_hash
    }

    fn apply_castling_rights(&self, current_hash: u64, castling_rights: CastlingRights, m: Move, to_move: Color) -> u64 {
        let mut new_hash = current_hash;

        let moving_piece = m.moving_piece;
        // lose all castling rights when king moves
        if moving_piece == Piece::KING {
            if to_move == Color::WHITE {
                if castling_rights.white_king_side {
                    new_hash ^= self.castling_rights[WHITE_KING_SIDE]; 
                }
                if castling_rights.white_queen_side {
                    new_hash ^= self.castling_rights[WHITE_QUEEN_SIDE];
                }
            } else {
                if castling_rights.black_king_side {
                    new_hash ^= self.castling_rights[BLACK_KING_SIDE];
                }
                if castling_rights.black_queen_side {
                    new_hash ^= self.castling_rights[BLACK_QUEEN_SIDE];
                }
            }
        }

        // lose queen side castling rights when queen side rook moves 
        if moving_piece == Piece::ROOK && m.from.file() == 1 {
            if to_move == Color::WHITE {
                if castling_rights.white_queen_side {
                    new_hash ^= self.castling_rights[WHITE_QUEEN_SIDE];
                }
            } else {
                if castling_rights.black_queen_side {
                    new_hash ^= self.castling_rights[BLACK_QUEEN_SIDE];
                }
            }
        }

        // lose king side castling rights when king side rook moves
        if moving_piece == Piece::ROOK && m.from.file() == 8 {
            if to_move == Color::WHITE {
                if castling_rights.white_king_side {
                    new_hash ^= self.castling_rights[WHITE_KING_SIDE];
                }
            } else {
                if castling_rights.black_king_side {
                    new_hash ^= self.castling_rights[BLACK_KING_SIDE];
                }
            }
        }

        // lose castling rights when rook is captured
        match m.move_type {
            MoveType::Capture(Piece::ROOK) => {
                if m.to.file() == 1 {
                    if to_move == Color::WHITE {
                        if castling_rights.black_queen_side {
                            new_hash ^= self.castling_rights[BLACK_QUEEN_SIDE];
                        }
                    } else {
                        if castling_rights.white_queen_side {
                            new_hash ^= self.castling_rights[WHITE_QUEEN_SIDE];
                        }
                    }
                } else if m.to.file() == 8 {
                    if to_move == Color::WHITE {
                        if castling_rights.black_king_side {
                            new_hash ^= self.castling_rights[BLACK_KING_SIDE];
                        }
                    } else {
                        if castling_rights.white_king_side {
                            new_hash ^= self.castling_rights[WHITE_KING_SIDE];
                        }
                    }
                }
            
            },
            _ => (),
        } 

        new_hash
    }

    fn unapply_castling_rights(&self, current_hash: u64, castling_rights: CastlingRights, m: Move, _to_move: Color) -> u64 {
        let mut new_hash = current_hash;
        let last_castling_rights = m.last_castling_rights;

        if castling_rights.white_king_side != last_castling_rights.white_king_side {
            new_hash ^= self.castling_rights[WHITE_KING_SIDE];
        }
        if castling_rights.white_queen_side != last_castling_rights.white_queen_side {
            new_hash ^= self.castling_rights[WHITE_QUEEN_SIDE];
        }
        if castling_rights.black_king_side != last_castling_rights.black_king_side {
            new_hash ^= self.castling_rights[BLACK_KING_SIDE];
        }
        if castling_rights.black_queen_side != last_castling_rights.black_queen_side {
            new_hash ^= self.castling_rights[BLACK_QUEEN_SIDE];
        }

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