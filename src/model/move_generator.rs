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

    // TODO: test performance with smallvec: https://github.com/servo/rust-smallvec
    pub fn generate_moves(&self, board: &GameState) -> Vec<Move> {
        let mut moves = vec![];
        moves.append(&mut self.generate_queen_moves(board));
        moves.append(&mut self.generate_rook_moves(board));
        moves.append(&mut self.generate_bishop_moves(board));
        moves.append(&mut self.generate_knight_moves(board));
        moves.append(&mut self.generate_pawn_moves(board));
        moves
    }

    pub fn generate_rook_moves(&self, board: &GameState) -> Vec<Move> {
        self.generate_moves_from_trace_and_piece_positions(
            board,
            board.get_piece_position(Piece::ROOK, board.to_move()),
            Piece::ROOK,
            &self.rook_trace)
    }

    pub fn generate_bishop_moves(&self, board: &GameState) -> Vec<Move> {
        self.generate_moves_from_trace_and_piece_positions(
            board,
            board.get_piece_position(Piece::BISHOP, board.to_move()),
            Piece::BISHOP,
            &self.bishop_trace)
    }

    pub fn generate_knight_moves(&self, board: &GameState) -> Vec<Move> {
        self.generate_moves_from_trace_and_piece_positions(
            board,
            board.get_piece_position(Piece::KNIGHT, board.to_move()),
            Piece::KNIGHT,
            &self.knight_trace)
    }

    pub fn generate_queen_moves(&self, board: &GameState) -> Vec<Move> {
        self.generate_moves_from_trace_and_piece_positions(
            board,
            board.get_piece_position(Piece::QUEEN, board.to_move()),
            Piece::QUEEN,
            &self.queen_trace)
    }

    pub fn generate_pawn_moves(&self, board: &GameState) -> Vec<Move> {
        let mut moves = vec![];
        moves.append(&mut self.generate_pawn_steps(board, board.to_move()));
        moves.append(&mut self.generate_pawn_captures(board, board.to_move()));
        moves.append(&mut self.generate_en_passant_captures(board));
        moves
    }

    fn generate_moves_from_trace_and_piece_positions(
        &self, board: &GameState, 
        piece_positions: Vec<Position>, 
        piece: Piece,
        trace: &Vec<Vec<Vec<Position>>>) -> Vec<Move> {

        let mut moves = vec![];
        for piece_position in piece_positions {
            let mut moves_from_position = self.generate_moves_from_trace_and_position(board, piece_position, piece, trace);
            moves.append(&mut moves_from_position);
        }
        moves
    }

    fn generate_moves_from_trace_and_position(
        &self,
        board: &GameState,
        position: Position,
        piece: Piece,
        trace: &Vec<Vec<Vec<Position>>>) -> Vec<Move> {

        let opposite_color = board.to_move().opposite();
        let mut moves = vec![];

        for ray in &trace[usize::from(position.to_numeric())] {
            for to_move in ray {
                match board.collide(*to_move) {
                    None => moves.push(
                        Move {
                            move_type: MoveType::Step,
                            moving_piece: piece, 
                            from: position,
                            to: *to_move,
                            last_en_passant: board.en_passant(),
                            last_castling_rights: board.castling_rights, 
                        }),
                    Some(color) if color == opposite_color => { 
                        moves.push(
                            Move { 
                                move_type: MoveType::Capture(board.get_piece(*to_move).unwrap().0),
                                moving_piece: piece,
                                from: position,
                                to: *to_move,
                                last_en_passant: board.en_passant(),
                                last_castling_rights: board.castling_rights,
                            }); 
                        break 
                    },
                    Some(color) if color != opposite_color => break,
                    _ => panic!("Not possible"),
                }
            }
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
            .map(|position| Move { 
                move_type: MoveType::Step,
                moving_piece: Piece::PAWN,
                from: position.delta(0, -1 * direction_multiplier).unwrap(), 
                to: *position,
                last_en_passant: board.en_passant(),
                last_castling_rights: board.castling_rights,
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
                    last_en_passant: board.en_passant(),
                    last_castling_rights: board.castling_rights,
                })
            .collect();
        
        moves.append(&mut one_step_moves);
        moves.append(&mut two_step_moves);

        moves
    }

    // no en passant
    fn generate_pawn_captures(&self, board: &GameState, color: Color) -> Vec<Move> {
        let current_pawns = board.get_piece_mask(Piece::PAWN, color);

        // calculate squares where pawns may attack
        // shift pawn mask by 7 and 9 to get "forward facing diagonals" except on files 1 and 8 (where it would wrap)
        let attack_mask = if board.to_move() == Color::WHITE {
            ((current_pawns & !MASK_FILE1) << 7) | ((current_pawns & !MASK_FILE8) << 9)
        } else {
            ((current_pawns & !MASK_FILE1) >> 9) | ((current_pawns & !MASK_FILE8) >> 7)
        };

        let valid_captures = board.collide_mask_color(attack_mask , color.opposite());

        let mut moves = vec![];
        let direction_multiplier = if color == Color::WHITE { 1 } else { -1 };

        for square in bit_mask_to_positions(valid_captures) {
            let file = square.file();
            let pawn_mask = board.get_piece_mask(Piece::PAWN, color);

            let left_candidate = square.delta(-1, -direction_multiplier);
            if file != 1 && pawn_mask & left_candidate.unwrap().to_bit_mask() > 0 {
                moves.push(
                    Move { 
                        move_type: MoveType::Capture(board.get_piece(square).unwrap().0),
                        moving_piece: Piece::PAWN,
                        from: left_candidate.unwrap(),
                        to: square,
                        last_en_passant: board.en_passant(),
                        last_castling_rights: board.castling_rights,
                    });
            }
            let right_candidate = square.delta(1, -direction_multiplier); 
            if file != 8 && pawn_mask & right_candidate.unwrap().to_bit_mask() > 0 {
                moves.push(
                    Move { 
                        move_type: MoveType::Capture(board.get_piece(square).unwrap().0),
                        moving_piece: Piece::PAWN,
                        from: right_candidate.unwrap(),
                        to: square,
                        last_en_passant: board.en_passant(),
                        last_castling_rights: board.castling_rights,
                    });
            }
        }

        moves
     }

     // not supported yet, add an en passant mask to game state and update on each apply_move call
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
                    last_en_passant: board.en_passant(),
                    last_castling_rights: board.castling_rights,
                };
                moves.push(en_passant);
            }
        }

        moves
     }
 
}
