use std::iter;
use std::fmt::{Display, Debug, Formatter, Error};
use std::convert::TryFrom;
use super::zobrist_hash;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum MoveType {
    Step,
    Capture(Piece),
    EnPassant,
    Promotion(Piece),
    Castling,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Move {
    pub move_type: MoveType,
    pub moving_piece: Piece,
    pub from: Position,
    pub to: Position,
    pub last_en_passant: Option<Position>,
    pub last_castling_rights: CastlingRights,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct CastlingRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

impl CastlingRights {
    pub fn initial() -> CastlingRights {
        CastlingRights {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }

    pub fn none(color: Color) -> CastlingRights {
        CastlingRights {
            white_king_side: color == Color::BLACK,
            white_queen_side: color == Color::BLACK,
            black_king_side: color == Color::WHITE,
            black_queen_side: color == Color::WHITE,
        }
    }

    pub fn get_king_side_mut(&mut self, color: Color) -> &mut bool {
        if color == Color::WHITE {
            &mut self.white_king_side
        } else {
            &mut self.black_king_side
        }
    }

    pub fn get_queen_side_mut(&mut self, color: Color) -> &mut bool {
        if color == Color::WHITE {
            &mut self.white_queen_side
        } else {
            &mut self.black_queen_side
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Color {
    WHITE,
    BLACK,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct Position {
    position: u8
}

impl Position {
    pub fn new(file: u8, rank: u8) -> Position {
        if file < 1 || file > 8 {
            panic!("File must be between 1 and 8")
        }
        if rank < 1 || rank > 8 {
            panic!("Rank must be between 1 and 8")
        }

        Position {
            position: (file - 1) + (rank - 1) * 8
        }
    }

    pub fn to_bit_mask(&self) -> u64 {
        1 << self.position
    }

    pub fn delta(&self, delta_file: i8, delta_rank: i8) -> Option<Position> {
        let new_file = i16::from(self.file()) + i16::from(delta_file);
        let new_rank = i16::from(self.rank()) + i16::from(delta_rank);

        if new_file < 1 || new_file > 8 {
            return Option::None;
        } 
        if new_rank < 1 || new_rank > 8 {
            return Option::None;
        }

        let new_position = Position::new(u8::try_from(new_file).unwrap(), u8::try_from(new_rank).unwrap());

        Option::Some(new_position)
    }

    pub fn to_numeric(&self) -> u8 {
        self.position
    }

    pub fn from_numeric(numeric_position: u8) -> Self {
        Position { position: numeric_position }
    }

    pub fn rank(&self) -> u8 {
        self.to_numeric() / 8 + 1
    }

    pub fn file(&self) -> u8 {
        self.to_numeric() % 8 + 1
    }

    pub fn mirror_rank(&self) -> Position {
        Position::new(self.file(), 9 - self.rank())
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let file_repr = std::char::from_u32(('A' as u32) + (self.file() as u32 - 1)); 
        let rank_repr = self.rank().to_string();

        if let Some(repr) = file_repr {
            write!(f, "{}{}", repr, rank_repr)
        } else {
            Err(Error)
        }
    }
} 

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Display::fmt(self, f)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Piece {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct GameState {
    white_pawn: u64,
    white_knight: u64,
    white_bishop: u64,
    white_rook: u64,
    white_queen: u64,
    white_king: u64,

    black_pawn: u64,
    black_knight: u64,
    black_bishop: u64,
    black_rook: u64,
    black_queen: u64,
    black_king: u64,

    en_passant: Option<Position>,

    pub castling_rights: CastlingRights,

    pub to_move: Color,

    pub zobrist_hash: u64,
}

impl GameState {
    fn new_empty() -> Self {
        GameState {
            white_pawn: 0,
            white_knight: 0,
            white_bishop: 0,
            white_rook: 0,
            white_queen: 0,
            white_king: 0,

            black_pawn: 0,
            black_knight: 0,
            black_bishop: 0,
            black_rook: 0,
            black_queen: 0,
            black_king: 0,

            en_passant: None,

            castling_rights: CastlingRights::initial(),

            to_move: Color::WHITE,

            zobrist_hash: 0,
        }
    }

    pub fn new() -> Self {
        let mut state = GameState::new_empty();

        for file in 1..9 {
            state.set_piece(Piece::PAWN, Color::WHITE, Position::new(file, 2));
            state.set_piece(Piece::PAWN, Color::BLACK, Position::new(file, 7));
        }

        state.set_piece(Piece::ROOK, Color::WHITE, Position::new(1, 1));
        state.set_piece(Piece::ROOK, Color::WHITE, Position::new(8, 1));
        state.set_piece(Piece::ROOK, Color::BLACK, Position::new(1, 8));
        state.set_piece(Piece::ROOK, Color::BLACK, Position::new(8, 8));

        state.set_piece(Piece::BISHOP, Color::WHITE, Position::new(3, 1));
        state.set_piece(Piece::BISHOP, Color::WHITE, Position::new(6, 1));
        state.set_piece(Piece::BISHOP, Color::BLACK, Position::new(3, 8));
        state.set_piece(Piece::BISHOP, Color::BLACK, Position::new(6, 8));

        state.set_piece(Piece::KNIGHT, Color::WHITE, Position::new(2, 1));
        state.set_piece(Piece::KNIGHT, Color::WHITE, Position::new(7, 1));
        state.set_piece(Piece::KNIGHT, Color::BLACK, Position::new(2, 8));
        state.set_piece(Piece::KNIGHT, Color::BLACK, Position::new(7, 8));

        state.set_piece(Piece::QUEEN, Color::WHITE, Position::new(4, 1));
        state.set_piece(Piece::QUEEN, Color::BLACK, Position::new(4, 8));
        state.set_piece(Piece::KING, Color::WHITE, Position::new(5, 1));
        state.set_piece(Piece::KING, Color::BLACK, Position::new(5, 8));

        state.zobrist_hash = zobrist_hash::hash(&state);

        state
    }

    pub fn to_move(&self) -> Color {
        self.to_move
    }

    pub fn en_passant(&self) -> Option<Position> {
        self.en_passant
    }

    pub fn set_piece(&mut self, piece: Piece, color: Color, position: Position) {
        let position_bit_mask = position.to_bit_mask();
        let piece_mask = self.get_piece_mask_mut(piece, color);
        *piece_mask = *piece_mask | position_bit_mask;
    }

    pub fn get_piece_position(&self, piece: Piece, color: Color) -> Vec<Position> {
        bit_mask_to_positions(*self.get_piece_mask(piece, color))
    }

    pub fn collide(&self, position: Position) -> Option<Color> {
        let white_mask = self.white_mask();
        let black_mask = self.black_mask();

        let position_bit_mask = position.to_bit_mask();

        if position_bit_mask & white_mask > 0 {
            Option::Some(Color::WHITE)
        } else if position_bit_mask & black_mask > 0 {
            Option::Some(Color::BLACK)
        } else {
            Option::None
        }
    }

    pub fn collide_mask(&self, to_test: u64) -> u64 {
        to_test & (self.white_mask() | self.black_mask())
    }

    pub fn collide_mask_color(&self, to_test: u64, color: Color) -> u64 {
        to_test & if color == Color::WHITE { self.white_mask() } else { self.black_mask() }
    }

    fn white_mask(&self) -> u64 { 
        self.white_pawn | self.white_knight | self.white_bishop | self.white_rook | self.white_queen | self.white_king
    }

    fn black_mask(&self) -> u64 {
        self.black_pawn | self.black_knight | self.black_bishop | self.black_rook | self.black_queen | self.black_king
    }

    pub fn apply_move(&self, to_apply: Move) -> GameState {
        let mut new_state  = *self;
        new_state.apply_move_mut(to_apply);
        new_state
    }

    pub fn apply_move_mut(&mut self, to_apply: Move) {
        self.zobrist_hash = zobrist_hash::apply_move(self.zobrist_hash, &self.castling_rights, to_apply, self.to_move());

        let moving_piece = to_apply.moving_piece;
        let piece_mask_for_moving = *self.get_piece_mask(moving_piece, self.to_move());
        
        match to_apply.move_type {
            MoveType::Capture(captured_piece) => {
                let new_state_taken_piece_mask = self.get_piece_mask_mut(captured_piece, self.to_move().opposite());
                *new_state_taken_piece_mask = *new_state_taken_piece_mask ^ to_apply.to.to_bit_mask();
            },
            MoveType::EnPassant => {
                let direction_multiplier = if self.to_move == Color::WHITE { 1 } else { -1 };
                let new_state_taken_piece_mask = self.get_piece_mask_mut(Piece::PAWN, self.to_move().opposite());
                *new_state_taken_piece_mask = *new_state_taken_piece_mask ^ (to_apply.to.delta(0, -direction_multiplier).unwrap().to_bit_mask());
            },
            MoveType::Castling => {
                let rank = to_apply.from.rank();
                let (old_rook_file, new_rook_file) = if to_apply.to.file() < 5 { (1, 4) } else { (8, 6) };
                let old_rook_position = Position::new(old_rook_file, rank);
                let new_rook_position = Position::new(new_rook_file, rank);
                let rook_piece_mask = self.get_piece_mask_mut(Piece::ROOK, self.to_move());
                *rook_piece_mask = *rook_piece_mask ^ old_rook_position.to_bit_mask();
                *rook_piece_mask = *rook_piece_mask ^ new_rook_position.to_bit_mask();
                self.castling_rights = CastlingRights::none(self.to_move());
            },
            _ => (),
        }

        // the moving piece type's bit mask is cleared from move's origin and set to destination
        let new_piece_mask_for_moving = piece_mask_for_moving ^ (to_apply.from.to_bit_mask() | to_apply.to.to_bit_mask());

        let new_state_moving_piece_mask = self.get_piece_mask_mut(moving_piece, self.to_move());
        // update bit mask for moving piece
        *new_state_moving_piece_mask = new_piece_mask_for_moving;

        // set en passant bit mask
        if moving_piece == Piece::PAWN {
            let is_two_steps_move = i16::abs(i16::from(to_apply.from.to_numeric()) - i16::from(to_apply.to.to_numeric())) == 16;
            if is_two_steps_move {
                self.en_passant = Some(to_apply.to);
            } else {
                self.en_passant = None;
            }
        } else {
            self.en_passant = None;
        }

        // lose all castling rights when king moves
        if moving_piece == Piece::KING {
            self.castling_rights = CastlingRights::none(self.to_move());
        }

        // lose queen side castling rights when queen side rook moves 
        if moving_piece == Piece::ROOK && to_apply.from.file() == 1 {
            *self.castling_rights.get_queen_side_mut(self.to_move()) = false;
        }

        // lose king side castling rights when king side rook moves
        if moving_piece == Piece::ROOK && to_apply.from.file() == 8 {
            *self.castling_rights.get_king_side_mut(self.to_move()) = false;
        }

        self.to_move = self.to_move.opposite();
    }

    pub fn unapply_move_mut(&mut self, to_unapply: Move) {
        self.zobrist_hash = zobrist_hash::unapply_move(self.zobrist_hash, &self.castling_rights, to_unapply, self.to_move());

        let moving_piece = to_unapply.moving_piece;
        let piece_mask_for_moving = *self.get_piece_mask(moving_piece, self.to_move().opposite());
        
        match to_unapply.move_type {
            MoveType::Capture(captured_piece) => {
                let new_state_taken_piece_mask = self.get_piece_mask_mut(captured_piece, self.to_move());
                *new_state_taken_piece_mask = *new_state_taken_piece_mask ^ to_unapply.to.to_bit_mask();
            },
            MoveType::EnPassant => {
                let direction_multiplier = if self.to_move.opposite() == Color::WHITE { 1 } else { -1 };
                let new_state_taken_piece_mask = self.get_piece_mask_mut(Piece::PAWN, self.to_move());
                *new_state_taken_piece_mask = *new_state_taken_piece_mask ^ (to_unapply.to.delta(0, -direction_multiplier).unwrap().to_bit_mask());
            },
            MoveType::Castling => {
                let rank = to_unapply.from.rank();
                let (old_rook_file, new_rook_file) = if to_unapply.to.file() < 5 { (1, 4) } else { (8, 6) };
                let old_rook_position = Position::new(old_rook_file, rank);
                let new_rook_position = Position::new(new_rook_file, rank);
                let rook_piece_mask = self.get_piece_mask_mut(Piece::ROOK, self.to_move().opposite());
                *rook_piece_mask = *rook_piece_mask ^ old_rook_position.to_bit_mask();
                *rook_piece_mask = *rook_piece_mask ^ new_rook_position.to_bit_mask();
                self.castling_rights = CastlingRights::none(self.to_move().opposite());
            },
            _ => (),
        }

        // the moving piece type's bit mask is cleared from move's origin and set to destination
        let new_piece_mask_for_moving = piece_mask_for_moving ^ (to_unapply.from.to_bit_mask() | to_unapply.to.to_bit_mask());

        let new_state_moving_piece_mask = self.get_piece_mask_mut(moving_piece, self.to_move().opposite());
        // update bit mask for moving piece
        *new_state_moving_piece_mask = new_piece_mask_for_moving;

        self.en_passant = to_unapply.last_en_passant;
        self.castling_rights = to_unapply.last_castling_rights;
        self.to_move = self.to_move.opposite();
    }

    pub fn get_piece_mask(&self, piece: Piece, color: Color) -> &u64 {
        match (piece, color) {
            (Piece::PAWN, Color::WHITE) => &self.white_pawn,
            (Piece::KNIGHT, Color::WHITE) => &self.white_knight,
            (Piece::BISHOP, Color::WHITE) => &self.white_bishop,
            (Piece::ROOK, Color::WHITE) => &self.white_rook,
            (Piece::QUEEN, Color::WHITE) => &self.white_queen,
            (Piece::KING, Color::WHITE) => &self.white_king,
            (Piece::PAWN, Color::BLACK) => &self.black_pawn,
            (Piece::KNIGHT, Color::BLACK) => &self.black_knight,
            (Piece::BISHOP, Color::BLACK) => &self.black_bishop,
            (Piece::ROOK, Color::BLACK) => &self.black_rook,
            (Piece::QUEEN, Color::BLACK) => &self.black_queen,
            (Piece::KING, Color::BLACK) => &self.black_king,
        }
    }

    fn get_piece_mask_mut(&mut self, piece: Piece, color: Color) -> &mut u64 {
        match (piece, color) {
            (Piece::PAWN, Color::WHITE) => &mut self.white_pawn,
            (Piece::KNIGHT, Color::WHITE) => &mut self.white_knight,
            (Piece::BISHOP, Color::WHITE) => &mut self.white_bishop,
            (Piece::ROOK, Color::WHITE) => &mut self.white_rook,
            (Piece::QUEEN, Color::WHITE) => &mut self.white_queen,
            (Piece::KING, Color::WHITE) => &mut self.white_king,
            (Piece::PAWN, Color::BLACK) => &mut self.black_pawn,
            (Piece::KNIGHT, Color::BLACK) => &mut self.black_knight,
            (Piece::BISHOP, Color::BLACK) => &mut self.black_bishop,
            (Piece::ROOK, Color::BLACK) => &mut self.black_rook,
            (Piece::QUEEN, Color::BLACK) => &mut self.black_queen,
            (Piece::KING, Color::BLACK) => &mut self.black_king,
        }
    }

    fn square_to_unicode(&self, position: Position) -> &str {
        let maybe_piece = self.get_piece(position)
            .map(|piece|  
                match piece {
                    (Piece::PAWN, Color::WHITE) => "♙",
                    (Piece::KNIGHT, Color::WHITE) => "♘",
                    (Piece::BISHOP, Color::WHITE) => "♗",
                    (Piece::ROOK, Color::WHITE)  => "♖",
                    (Piece::QUEEN, Color::WHITE) => "♕",
                    (Piece::KING, Color::WHITE) => "♔",
                    (Piece::PAWN, Color::BLACK) => "♟︎",
                    (Piece::KNIGHT, Color::BLACK) => "♞",
                    (Piece::BISHOP, Color::BLACK) => "♝",
                    (Piece::ROOK, Color::BLACK) => "♜",
                    (Piece::QUEEN, Color::BLACK) => "♛",
                    (Piece::KING, Color::BLACK) => "♚",
            });
        maybe_piece.unwrap_or(" ")
    }

    pub fn get_piece(&self, position: Position) -> Option<(Piece, Color)> {
        let bit_mask = position.to_bit_mask();

        if self.white_pawn & bit_mask > 0 {
            Option::Some((Piece::PAWN, Color::WHITE))
        } else if self.white_knight & bit_mask > 0 {
            Option::Some((Piece::KNIGHT, Color::WHITE)) 
        } else if self.white_bishop & bit_mask > 0 {
            Option::Some((Piece::BISHOP, Color::WHITE))
        } else if self.white_rook & bit_mask > 0{
            Option::Some((Piece::ROOK, Color::WHITE))
        } else if self.white_queen & bit_mask > 0 {
            Option::Some((Piece::QUEEN, Color::WHITE))
        } else if self.white_king & bit_mask > 0 {
            Option::Some((Piece::KING, Color::WHITE))
        } else if self.black_pawn & bit_mask > 0 {
            Option::Some((Piece::PAWN, Color::BLACK))
        } else if self.black_knight & bit_mask > 0 {
            Option::Some((Piece::KNIGHT, Color::BLACK))
        } else if self.black_bishop & bit_mask > 0 {
            Option::Some(( Piece::BISHOP, Color::BLACK))
        } else if self.black_rook & bit_mask > 0 {
            Option::Some((Piece::ROOK, Color::BLACK))
        } else if self.black_queen & bit_mask > 0 {
            Option::Some((Piece::QUEEN, Color::BLACK))
        } else if self.black_king & bit_mask > 0 {
            Option::Some((Piece::KING, Color::BLACK))
        } else {
            Option::None
        }
    }

    pub fn to_string(&self) -> String {
        let mut builder = String::from("");

        for rank in (1..9).rev() {
            for file in 1..9 {
                builder.push_str(self.square_to_unicode(Position::new(file, rank)));            
            }
            if rank > 1 {
                builder.push_str("\n");
            }   
        }

        builder
    }

}

impl Debug for GameState {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "\n{}", self.to_string())
    }
}


pub fn bit_mask_to_positions(bit_mask: u64) -> Vec<Position> {
    let mut vec = vec![];
    let mut current_pos = 0;

    let mut mask = bit_mask;
    
    while mask > 0 {
        let trailing_zeros = mask.trailing_zeros();
        let (shifted, overflow) = mask.overflowing_shr(trailing_zeros + 1);
        mask = if overflow { 0 } else { shifted };
        current_pos += trailing_zeros + 1;
        vec.push(Position::from_numeric(u8::try_from(current_pos - 1).unwrap()));
    } 

    vec
}