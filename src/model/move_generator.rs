use super::game_state::GameState;
use super::game_state::Position;
use super::game_state::Color;
use super::game_state::Piece;
use super::game_state::Move;
use super::game_state::MoveType;
use super::game_state::bit_mask_to_positions;
use super::attack_trace;

const MASK_FILE1: u64 = 0x0101010101010101;
const MASK_FILE8: u64 = 0x8080808080808080;
const MASK_RANK3: u64 = 0x0000000000ff0000;
const MASK_RANK6: u64 = 0x0000ff0000000000;

pub struct GeneratedMoves {
    pub moves: Vec<Move>,
    pub is_check: bool,
}

impl GeneratedMoves {
    pub fn is_checkmate(&self) -> bool {
        self.is_check && self.moves.len() == 0
    }
}

pub struct MoveGenerator {
    rook_trace: Vec<Vec<Vec<Position>>>,
    bishop_trace: Vec<Vec<Vec<Position>>>,
    queen_trace: Vec<Vec<Vec<Position>>>,
    knight_trace: Vec<Vec<Vec<Position>>>,
}

impl MoveGenerator {
    pub fn new() -> Self {
        MoveGenerator {
            rook_trace: attack_trace::attack_trace_for_rook(),
            bishop_trace: attack_trace::attack_trace_for_bishop(),
            queen_trace: attack_trace::attack_trace_for_queen(),
            knight_trace: attack_trace::attack_trace_for_knight(),
        }
    }

    pub fn get_move(&self, board: &GameState, from: Position, to: Position) -> Option<Move> {
        self.generate_moves(board).moves.into_iter().find(|m| m.to == to && m.from == from)
    }


    // TODO: use mutable board with apply_move_mut and takeback to avoid copying
    pub fn generate_moves(&self, board: &GameState) -> GeneratedMoves {
        let candidate_moves = self.generate_moves_unchecked(board);
        let is_check = self.is_check(board, board.to_move());
        let to_move = board.to_move();

        GeneratedMoves {
            moves: candidate_moves.into_iter().filter(|m| !self.is_check(&board.apply_move(*m), to_move)).collect(),
            is_check: is_check,
        }
    }

    fn is_check(&self, board: &GameState, color: Color) -> bool {
        (board.get_piece_mask(Piece::KING, color) & self.generate_threats(board, color.opposite())) != 0
    }

    // TODO: test performance with smallvec: https://github.com/servo/rust-smallvec
    pub fn generate_moves_unchecked(&self, board: &GameState) -> Vec<Move> {
        let mut moves = vec![];
        moves.append(&mut self.generate_queen_moves(board, board.to_move()));
        moves.append(&mut self.generate_rook_moves(board, board.to_move()));
        moves.append(&mut self.generate_bishop_moves(board, board.to_move()));
        moves.append(&mut self.generate_knight_moves(board, board.to_move()));
        moves.append(&mut self.generate_pawn_moves(board, board.to_move()));
        moves.append(&mut self.generate_king_moves(board, board.to_move()));
        moves.append(&mut self.generate_castling_moves(board, board.to_move()));

        moves
    }

    fn generate_threats(&self, board: &GameState, color: Color) -> u64 {
        let mut moves = vec![];
        moves.append(&mut self.generate_queen_moves(board, color));
        moves.append(&mut self.generate_rook_moves(board, color));
        moves.append(&mut self.generate_bishop_moves(board, color));
        moves.append(&mut self.generate_knight_moves(board, color));
        moves.append(&mut self.generate_pawn_captures(board, color));
        moves.append(&mut self.generate_king_moves(board, color));

        moves.into_iter().fold(0, |mask, m| mask | m.to.to_bit_mask())
    }

    pub fn generate_rook_moves(&self, board: &GameState, to_move: Color) -> Vec<Move> {
        self.generate_moves_from_trace_and_piece_positions(
            board,
            to_move,
            board.get_piece_position(Piece::ROOK, to_move),
            Piece::ROOK,
            &self.rook_trace)
    }

    pub fn generate_bishop_moves(&self, board: &GameState, to_move: Color) -> Vec<Move> {
        self.generate_moves_from_trace_and_piece_positions(
            board,
            to_move,
            board.get_piece_position(Piece::BISHOP, to_move),
            Piece::BISHOP,
            &self.bishop_trace)
    }

    pub fn generate_knight_moves(&self, board: &GameState, to_move: Color) -> Vec<Move> {
        self.generate_moves_from_trace_and_piece_positions(
            board,
            to_move,
            board.get_piece_position(Piece::KNIGHT, to_move),
            Piece::KNIGHT,
            &self.knight_trace)
    }

    pub fn generate_castling_moves(&self, board: &GameState, to_move: Color) -> Vec<Move> {
        let opponent_threats = self.generate_threats(board, to_move.opposite());
        let king = board.get_piece_position(Piece::KING, to_move)[0];

        let mut moves = vec![];

        if to_move == Color::WHITE && board.castling_rights.white_king_side ||
           to_move == Color::BLACK && board.castling_rights.black_king_side {
            let positions = [king, king.delta(1, 0).unwrap(), king.delta(2, 0).unwrap()];
            
            let is_no_threat_for_castling = positions[1..=2].iter().all(|pos| (pos.to_bit_mask() & opponent_threats) == 0); 
            let is_room_for_castling = positions[1..=2].iter().all(|pos| board.get_piece(*pos).is_none());

            if is_no_threat_for_castling && is_room_for_castling {
                moves.push(
                    Move { 
                        move_type: MoveType::Castling,
                        moving_piece: Piece::KING,
                        from: king,
                        to: positions[2],
                        promotes_to: None,
                        last_en_passant: board.en_passant(),
                        last_castling_rights: board.castling_rights, 
                });
            }
        }

        if to_move == Color::WHITE && board.castling_rights.white_queen_side ||
           to_move == Color::BLACK && board.castling_rights.black_queen_side {
            let positions = [king, king.delta(-1, 0).unwrap(), king.delta(-2, 0).unwrap(), king.delta(-3, 0).unwrap()];

            let is_no_threat_for_castling = positions[1..=2].iter().all(|pos| (pos.to_bit_mask() & opponent_threats) == 0); 
            let is_room_for_castling = positions[1..=3].iter().all(|pos| board.get_piece(*pos).is_none());
            
            if is_no_threat_for_castling && is_room_for_castling {
                moves.push(
                    Move { 
                        move_type: MoveType::Castling,
                        moving_piece: Piece::KING,
                        from: king,
                        to: positions[2],
                        promotes_to: None,
                        last_en_passant: board.en_passant(),
                        last_castling_rights: board.castling_rights, 
                });
            }
        }

        moves
    }

    pub fn generate_queen_moves(&self, board: &GameState, to_move: Color) -> Vec<Move> {
        self.generate_moves_from_trace_and_piece_positions(
            board,
            to_move,
            board.get_piece_position(Piece::QUEEN, to_move),
            Piece::QUEEN,
            &self.queen_trace)
    }

    pub fn generate_pawn_moves(&self, board: &GameState, to_move: Color) -> Vec<Move> {
        let mut moves = vec![];
        moves.append(&mut self.generate_pawn_steps(board, to_move));
        moves.append(&mut self.generate_pawn_captures(board, to_move));
        moves.append(&mut self.generate_en_passant_captures(board));
        moves
    }

    pub fn generate_king_moves(&self, board: &GameState, to_move: Color) -> Vec<Move> {
        let mut moves = vec![];
        let king = board.get_piece_position(Piece::KING, to_move)[0];
        let target_squares = vec![
            king.delta(0, 1),
            king.delta(0, -1),
            king.delta(1, 0),
            king.delta(-1, 0),
            king.delta(1, 1),
            king.delta(-1, -1),
            king.delta(-1, 1),
            king.delta(1, -1),
        ].into_iter().filter_map(|x| x).collect::<Vec<Position>>();

        for square in target_squares {
            match board.collide(square) {
                None => moves.push(
                    Move {
                        move_type: MoveType::Step,
                        moving_piece: Piece::KING, 
                        from: king,
                        to: square,
                        promotes_to: None,
                        last_en_passant: board.en_passant(),
                        last_castling_rights: board.castling_rights, 
                    }),
                Some(color) if color == to_move.opposite() => { 
                    moves.push(
                        Move { 
                            move_type: MoveType::Capture(board.get_piece(square).unwrap().0),
                            moving_piece: Piece::KING,
                            from: king,
                            to: square,
                            promotes_to: None,
                            last_en_passant: board.en_passant(),
                            last_castling_rights: board.castling_rights,
                        });  
                },
                Some(color) if color == to_move => continue,
                _ => panic!("Not possible"),
            }
        }

        moves
    }

    fn generate_moves_from_trace_and_piece_positions(
        &self, board: &GameState,
        to_move: Color, 
        piece_positions: Vec<Position>, 
        piece: Piece,
        trace: &Vec<Vec<Vec<Position>>>) -> Vec<Move> {

        let mut moves = vec![];
        for piece_position in piece_positions {
            let mut moves_from_position = self.generate_moves_from_trace_and_position(board, to_move, piece_position, piece, trace);
            moves.append(&mut moves_from_position);
        }
        moves
    }

    fn generate_moves_from_target_squares(
        &self,
        board: &GameState,
        to_move: Color,
        position: Position,
        piece: Piece,
        target_squares: &Vec<Position>) -> Vec<Move> {

        let opposite_color = to_move.opposite();
        let mut moves = vec![];

        for square in target_squares {
            match board.collide(*square) {
                None => moves.push(
                    Move {
                        move_type: MoveType::Step,
                        moving_piece: piece, 
                        from: position,
                        to: *square,
                        promotes_to: None,
                        last_en_passant: board.en_passant(),
                        last_castling_rights: board.castling_rights, 
                    }),
                Some(color) if color == opposite_color => { 
                    moves.push(
                        Move { 
                            move_type: MoveType::Capture(board.get_piece(*square).unwrap().0),
                            moving_piece: piece,
                            from: position,
                            to: *square,
                            promotes_to: None,
                            last_en_passant: board.en_passant(),
                            last_castling_rights: board.castling_rights,
                        }); 
                    break 
                },
                Some(color) if color != opposite_color => break,
                _ => panic!("Not possible"),
            }
        }

        moves
    }

    fn generate_moves_from_trace_and_position(
        &self,
        board: &GameState,
        to_move: Color,
        position: Position,
        piece: Piece,
        trace: &Vec<Vec<Vec<Position>>>) -> Vec<Move> {

        let mut moves = vec![];

        for ray in &trace[usize::from(position.to_numeric())] {
            moves.append(&mut self.generate_moves_from_target_squares(board, to_move, position, piece, ray));
        }

        moves
    }

    fn generate_pawn_steps(&self, board: &GameState, color: Color) -> Vec<Move> {
        let current_pawns = board.get_piece_mask(Piece::PAWN, color);

        fn take_step(pawns: u64, color: Color) -> u64 {
            if color == Color::WHITE {
                pawns << 8
            } else {
                pawns >> 8
            }
        }

        let pawns_one_step = take_step(*current_pawns, color);
        let valid_one_step_moves = pawns_one_step & !board.collide_mask(pawns_one_step);

        let can_take_second_step_mask = if color == Color::WHITE {
            MASK_RANK3
        } else {
            MASK_RANK6
        };

        let pawns_second_step = take_step(valid_one_step_moves & can_take_second_step_mask, color); 
        let valid_second_step_moves = pawns_second_step & !board.collide_mask(pawns_second_step); 
        let direction_multiplier = if color == Color::WHITE { 1 } else { -1 };

        let mut moves = vec![];

        let mut one_step_moves = bit_mask_to_positions(valid_one_step_moves)
            .iter()
            .flat_map(|position| {
                let rank = position.rank();
                let is_promotes_on_move = (rank == 7 && color == Color::WHITE) || (rank == 2 && color == Color::BLACK);
                let move_from = position.delta(0, -1 * direction_multiplier).unwrap();

                if is_promotes_on_move {
                    self.get_pawn_promotions(board, move_from, *position, None)
                } else {
                    vec![
                        Move { 
                            move_type: MoveType::Step,
                            moving_piece: Piece::PAWN,
                            from: move_from, 
                            to: *position,
                            promotes_to: None,
                            last_en_passant: board.en_passant(),
                            last_castling_rights: board.castling_rights,
                        }
                    ]
                }
            })
            .collect();

        let mut two_step_moves = bit_mask_to_positions(valid_second_step_moves)
            .iter()
            .map(|position| 
                Move { 
                    move_type: MoveType::Step,
                    moving_piece: Piece::PAWN,
                    from: position.delta(0, -2 * direction_multiplier).unwrap(),
                    to: *position,
                    promotes_to: None,
                    last_en_passant: board.en_passant(),
                    last_castling_rights: board.castling_rights,
                })
            .collect();
        
        moves.append(&mut one_step_moves);
        moves.append(&mut two_step_moves);

        moves
    }

    // does not include en passant captures
    fn generate_pawn_captures(&self, board: &GameState, color: Color) -> Vec<Move> {
        let current_pawns = board.get_piece_mask(Piece::PAWN, color);

        // calculate squares where pawns may attack
        // shift pawn mask by 7 and 9 to get "forward facing diagonals" except on files 1 and 8 (where it would wrap)
        let attack_mask = if color == Color::WHITE {
            ((current_pawns & !MASK_FILE1) << 7) | ((current_pawns & !MASK_FILE8) << 9)
        } else {
            ((current_pawns & !MASK_FILE1) >> 9) | ((current_pawns & !MASK_FILE8) >> 7)
        };

        let valid_captures = board.collide_mask_color(attack_mask, color.opposite());

        let mut moves = vec![];
        let direction_multiplier = if color == Color::WHITE { 1 } else { -1 };

        for square in bit_mask_to_positions(valid_captures) {
            let file = square.file();
            let rank = square.rank();
            let pawn_mask = board.get_piece_mask(Piece::PAWN, color);
            let is_promotes_on_move = (rank == 8 && color == Color::WHITE) || (rank == 1 && color == Color::BLACK);

            let left_candidate = square.delta(-1, -direction_multiplier);
            if file != 1 && left_candidate.map(|p| pawn_mask & p.to_bit_mask() > 0).unwrap_or(false) {
                let captured_piece = board.get_piece(square).unwrap().0;

                if is_promotes_on_move {
                    moves.append(&mut self.get_pawn_promotions(board, left_candidate.unwrap(), square, Some(captured_piece)));
                } else {
                    moves.push(
                        Move { 
                            move_type: MoveType::Capture(captured_piece),
                            moving_piece: Piece::PAWN,
                            from: left_candidate.unwrap(),
                            to: square,
                            promotes_to: None,
                            last_en_passant: board.en_passant(),
                            last_castling_rights: board.castling_rights,
                        }
                    );
                }
            }

            let right_candidate = square.delta(1, -direction_multiplier); 
            if file != 8 && right_candidate.map(|p| pawn_mask & p.to_bit_mask() > 0).unwrap_or(false) {
                let captured_piece = board.get_piece(square).unwrap().0;
                
                if is_promotes_on_move {
                    moves.append(&mut self.get_pawn_promotions(board, right_candidate.unwrap(), square, Some(captured_piece)));
                } else {
                    moves.push(
                        Move { 
                            move_type: MoveType::Capture(captured_piece),
                            moving_piece: Piece::PAWN,
                            from: right_candidate.unwrap(),
                            to: square,
                            promotes_to: None,
                            last_en_passant: board.en_passant(),
                            last_castling_rights: board.castling_rights,
                        }
                    ); 
                }
            }
        }

        moves
     }

     fn get_pawn_promotions(&self, board: &GameState, from: Position, to: Position, capture: Option<Piece>) -> Vec<Move> {
        let mut promotions = vec![];
        let pieces = [Piece::QUEEN, Piece::ROOK, Piece::BISHOP, Piece::KNIGHT];
        
        for piece in pieces.iter() {
            promotions.push(
                Move { 
                    move_type: capture.map(|p| MoveType::Capture(p)).unwrap_or(MoveType::Step),
                    moving_piece: Piece::PAWN,
                    from: from,
                    to: to,
                    promotes_to: Some(*piece),
                    last_en_passant: board.en_passant(),
                    last_castling_rights: board.castling_rights,
                }
            );
        }
        promotions
     }

     fn generate_en_passant_captures(&self, board: &GameState) -> Vec<Move> {
        let current_pawns = board.get_piece_mask(Piece::PAWN, board.to_move());
        let direction_multiplier = if board.to_move() == Color::WHITE { 1 } else { -1 };

        let mut moves = vec![];

        if let Some(en_passant_square) = board.en_passant() {
            let is_left_en_passant_valid = ((current_pawns & (!MASK_FILE8)) << 1) & (en_passant_square.to_bit_mask()) > 0;
            let is_right_en_passant_valid = ((current_pawns & (!MASK_FILE1)) >> 1) & (en_passant_square.to_bit_mask()) > 0;                

            if is_left_en_passant_valid {
                let en_passant = Move {
                    move_type: MoveType::EnPassant,
                    moving_piece: Piece::PAWN,
                    from: en_passant_square.delta(-1, 0).unwrap(),
                    to: en_passant_square.delta(0, direction_multiplier).unwrap(),
                    promotes_to: None,
                    last_en_passant: board.en_passant(),
                    last_castling_rights: board.castling_rights,
                };
                moves.push(en_passant);
            }

            if is_right_en_passant_valid {
                let en_passant = Move {
                    move_type: MoveType::EnPassant,
                    moving_piece: Piece::PAWN,
                    from: en_passant_square.delta(1, 0).unwrap(),
                    to: en_passant_square.delta(0, direction_multiplier).unwrap(),
                    promotes_to: None,
                    last_en_passant: board.en_passant(),
                    last_castling_rights: board.castling_rights,
                };
                moves.push(en_passant);
            }
        }

        moves
     }
 
}
