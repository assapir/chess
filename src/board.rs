use std::collections::HashMap;
use std::fmt;

use crate::{piece::Piece, Color, Move, Position, Square};

const MAX_DEPTH: usize = 4;

pub struct Board {
    pub squares: [[Square; 8]; 8],
    transposition_table: HashMap<u64, i32>,
    pub turn: Color,
}

impl Board {
    pub fn new() -> Self {
        let empty_square = Square::new(Piece::Empty, None);
        let mut squares = [[empty_square; 8]; 8];

        // Initialize pieces for both players
        for i in 0..8 {
            squares[1][i] = Square::new(Piece::Pawn, Some(Color::White));
            squares[6][i] = Square::new(Piece::Pawn, Some(Color::Black));
        }

        squares[0][0] = Square::new(Piece::Rook, Some(Color::White));
        squares[0][7] = Square::new(Piece::Rook, Some(Color::White));
        squares[7][0] = Square::new(Piece::Rook, Some(Color::Black));
        squares[7][7] = Square::new(Piece::Rook, Some(Color::Black));

        squares[0][1] = Square::new(Piece::Knight, Some(Color::White));
        squares[0][6] = Square::new(Piece::Knight, Some(Color::White));
        squares[7][1] = Square::new(Piece::Knight, Some(Color::Black));
        squares[7][6] = Square::new(Piece::Knight, Some(Color::Black));

        squares[0][2] = Square::new(Piece::Bishop, Some(Color::White));
        squares[0][5] = Square::new(Piece::Bishop, Some(Color::White));
        squares[7][2] = Square::new(Piece::Bishop, Some(Color::Black));
        squares[7][5] = Square::new(Piece::Bishop, Some(Color::Black));

        squares[0][3] = Square::new(Piece::Queen, Some(Color::White));
        squares[0][4] = Square::new(Piece::King, Some(Color::White));
        squares[7][3] = Square::new(Piece::Queen, Some(Color::Black));
        squares[7][4] = Square::new(Piece::King, Some(Color::Black));

        Board {
            squares,
            transposition_table: HashMap::new(),
            turn: Color::White,
        }
    }

    fn get_valid_moves(&self, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for (i, row) in self.squares.iter().enumerate() {
            for (j, square) in row.iter().enumerate() {
                if square.color == Some(color) {
                    match square.piece {
                        Piece::Pawn => {
                            let direction = if color == Color::White { 1 } else { -1 };
                            let new_i = (i as isize + direction) as usize;
                            if new_i < 8 && self.squares[new_i][j].piece == Piece::Empty {
                                moves.push(Move {
                                    from: Position { row: i, col: j },
                                    to: Position { row: new_i, col: j },
                                    piece: Piece::Pawn,
                                    captured: None,
                                    score: 0, // Initial score
                                });
                            }
                        }
                        Piece::King
                        | Piece::Queen
                        | Piece::Rook
                        | Piece::Bishop
                        | Piece::Knight => {
                            for &(di, dj) in &square.piece.directions() {
                                let mut new_i = i as isize;
                                let mut new_j = j as isize;
                                loop {
                                    new_i += di;
                                    new_j += dj;
                                    if new_i < 0 || new_i >= 8 || new_j < 0 || new_j >= 8 {
                                        break;
                                    }
                                    let target_square =
                                        self.squares[new_i as usize][new_j as usize];
                                    if target_square.piece == Piece::Empty {
                                        moves.push(Move {
                                            from: Position { row: i, col: j },
                                            to: Position {
                                                row: new_i as usize,
                                                col: new_j as usize,
                                            },
                                            piece: square.piece,
                                            captured: None,
                                            score: 0, // Initial score
                                        });
                                        if square.piece == Piece::King
                                            || square.piece == Piece::Knight
                                        {
                                            break; // King and Knight move only one step
                                        }
                                    } else {
                                        if target_square.color != Some(color) {
                                            moves.push(Move {
                                                from: Position { row: i, col: j },
                                                to: Position {
                                                    row: new_i as usize,
                                                    col: new_j as usize,
                                                },
                                                piece: square.piece,
                                                captured: Some(target_square.piece),
                                                score: 0, // Initial score
                                            });
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                        Piece::Empty => {}
                    }
                }
            }
        }

        // Sort moves based on a heuristic (e.g., captures and checks first)
        moves.sort_by_key(|mv| match mv.captured {
            Some(piece) => piece.value(), // Higher value pieces first
            None => 0,
        });

        moves
    }

    fn evaluate_board(&self) -> i32 {
        let mut score = 0;
        for (i, row) in self.squares.iter().enumerate() {
            for (j, square) in row.iter().enumerate() {
                let piece_value = match square.piece {
                    Piece::King => 900,
                    Piece::Queen => 90,
                    Piece::Rook => 50,
                    Piece::Bishop | Piece::Knight => 30,
                    Piece::Pawn => 10,
                    Piece::Empty => 0,
                };

                let position_value = square.piece.table()[i][j];

                score += (piece_value + position_value)
                    * match square.color {
                        Some(Color::White) => 1,
                        Some(Color::Black) => -1,
                        None => 0,
                    };
            }
        }

        // Add more sophisticated evaluation metrics
        score += self.evaluate_king_safety();
        score += self.evaluate_pawn_structure();
        score += self.evaluate_piece_activity();

        // Add mobility score
        let white_moves = self.get_valid_moves(Color::White).len() as i32;
        let black_moves = self.get_valid_moves(Color::Black).len() as i32;
        score += white_moves - black_moves;

        score
    }

    fn evaluate_king_safety(&self) -> i32 {
        let mut score = 0;

        for (i, row) in self.squares.iter().enumerate() {
            for (j, square) in row.iter().enumerate() {
                if square.piece == Piece::King {
                    let king_safety_value = if square.color == Some(Color::White) {
                        // Example heuristic: King safety is better in the corners
                        if i == 0 && (j == 0 || j == 7) {
                            20
                        } else {
                            10
                        }
                    } else {
                        if i == 7 && (j == 0 || j == 7) {
                            20
                        } else {
                            10
                        }
                    };
                    score += king_safety_value
                        * match square.color {
                            Some(Color::White) => 1,
                            Some(Color::Black) => -1,
                            None => 0,
                        };
                }
            }
        }

        score
    }

    fn evaluate_pawn_structure(&self) -> i32 {
        let mut score = 0;

        for (_i, row) in self.squares.iter().enumerate() {
            for (j, square) in row.iter().enumerate() {
                if square.piece == Piece::Pawn {
                    let pawn_structure_value = if square.color == Some(Color::White) {
                        // Example heuristic: Pawns are better in the center
                        if j == 3 || j == 4 {
                            5
                        } else {
                            2
                        }
                    } else {
                        if j == 3 || j == 4 {
                            5
                        } else {
                            2
                        }
                    };
                    score += pawn_structure_value
                        * match square.color {
                            Some(Color::White) => 1,
                            Some(Color::Black) => -1,
                            None => 0,
                        };
                }
            }
        }

        score
    }

    fn evaluate_piece_activity(&self) -> i32 {
        let mut score = 0;

        for (_i, row) in self.squares.iter().enumerate() {
            for (_j, square) in row.iter().enumerate() {
                if square.piece != Piece::Empty {
                    let piece_activity_value = match square.piece {
                        Piece::King => 0, // King activity is not usually considered
                        Piece::Queen => 10,
                        Piece::Rook => 5,
                        Piece::Bishop => 3,
                        Piece::Knight => 3,
                        Piece::Pawn => 1,
                        Piece::Empty => 0,
                    };
                    score += piece_activity_value
                        * match square.color {
                            Some(Color::White) => 1,
                            Some(Color::Black) => -1,
                            None => 0,
                        };
                }
            }
        }

        score
    }

    fn quiescence_search(&self, mut alpha: i32, beta: i32) -> i32 {
        let stand_pat = self.evaluate_board();
        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }

        let mut valid_moves = self.get_valid_moves(self.current_turn());
        valid_moves.retain(|mv| {
            mv.captured.is_some() || {
                let target_square = self.squares[mv.to.row][mv.to.col];
                if let Some(color) = target_square.color {
                    self.is_in_check(color)
                } else {
                    false
                }
            }
        }); // Consider captures and checks

        for mv in valid_moves.iter_mut() {
            let mut new_board = self.clone();
            new_board.make_move(mv.from, mv.to);
            let score = -new_board.quiescence_search(-beta, -alpha);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn minimax(&mut self, depth: usize, is_maximizing: bool, alpha: i32, beta: i32) -> i32 {
        let board_hash = self.hash();
        if let Some(&cached_eval) = self.transposition_table.get(&board_hash) {
            return cached_eval;
        }

        if depth == 0 {
            let eval = self.quiescence_search(alpha, beta);
            self.transposition_table.insert(board_hash, eval);
            return eval;
        }

        let color = if is_maximizing {
            Color::Black
        } else {
            Color::White
        };
        let mut valid_moves = self.get_valid_moves(color);

        let mut alpha = alpha;
        let mut beta = beta;
        let mut best_eval = if is_maximizing { i32::MIN } else { i32::MAX };

        for mv in valid_moves.iter_mut() {
            let mut new_board = self.clone();
            new_board.make_move(mv.from, mv.to);
            let eval = new_board.minimax(depth - 1, !is_maximizing, alpha, beta);
            if is_maximizing {
                best_eval = best_eval.max(eval);
                alpha = alpha.max(eval);
            } else {
                best_eval = best_eval.min(eval);
                beta = beta.min(eval);
            }
            if beta <= alpha {
                break;
            }
        }

        self.transposition_table.insert(board_hash, best_eval);
        best_eval
    }

    pub fn find_best_move(&mut self) -> Option<Move> {
        let color = self.turn;
        let mut best_move = None;
        let mut best_score = if color == Color::Black {
            i32::MIN
        } else {
            i32::MAX
        };

        for depth in 1..=MAX_DEPTH {
            let mut valid_moves = self.get_valid_moves(color);
            for mv in valid_moves.iter_mut() {
                let mut new_board = self.clone();
                new_board.make_move(mv.from, mv.to);
                let score = new_board.minimax(depth, color == Color::White, i32::MIN, i32::MAX);
                if (color == Color::Black && score > best_score)
                    || (color == Color::White && score < best_score)
                {
                    best_score = score;
                    best_move = Some(*mv);
                } else if score == best_score {
                    // Apply secondary criteria
                    if self.more_criteria(&mv, &best_move.unwrap()) {
                        best_move = Some(*mv);
                    }
                }
            }
        }

        best_move
    }

    fn more_criteria(&self, mv1: &Move, mv2: &Move) -> bool {
        // Example secondary criteria: prefer moves that control the center
        let center_squares = vec![
            Position { row: 3, col: 3 },
            Position { row: 3, col: 4 },
            Position { row: 4, col: 3 },
            Position { row: 4, col: 4 },
        ];

        let mv1_controls_center = center_squares.contains(&mv1.to);
        let mv2_controls_center = center_squares.contains(&mv2.to);

        if mv1_controls_center && !mv2_controls_center {
            return true;
        } else if !mv1_controls_center && mv2_controls_center {
            return false;
        }

        // Additional secondary criteria: prefer moves that increase mobility
        let mv1_mobility = self.get_valid_moves_after_move(mv1).len();
        let mv2_mobility = self.get_valid_moves_after_move(mv2).len();

        if mv1_mobility > mv2_mobility {
            return true;
        } else if mv1_mobility < mv2_mobility {
            return false;
        }

        // Additional secondary criteria can be added here

        false
    }

    fn get_valid_moves_after_move(&self, mv: &Move) -> Vec<Move> {
        let mut new_board = self.clone();
        new_board.make_move(mv.from, mv.to);
        new_board.get_valid_moves(new_board.turn)
    }

    pub fn make_move(&mut self, from: Position, to: Position) {
        let piece = self.squares[from.row][from.col].piece;
        let color = self.squares[from.row][from.col].color;
        self.squares[to.row][to.col] = Square::new(piece, color);
        self.squares[from.row][from.col] = Square::new(Piece::Empty, None);
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    pub fn is_checkmate(&self, color: Color) -> bool {
        // Check if the current player is in checkmate
        let valid_moves = self.get_valid_moves(color);
        valid_moves.is_empty() && self.is_in_check(color)
    }

    fn is_in_check(&self, color: Color) -> bool {
        // Find the king's position
        let king_pos = self.find_king(color);
        if let Some((king_i, king_j)) = king_pos {
            // Check if any opponent piece can move to the king's position
            let opponent_color = if color == Color::White {
                Color::Black
            } else {
                Color::White
            };
            let opponent_moves = self.get_valid_moves(opponent_color);
            for mv in opponent_moves {
                if mv.to
                    == (Position {
                        row: king_i,
                        col: king_j,
                    })
                {
                    return true;
                }
            }
        }
        false
    }

    fn find_king(&self, color: Color) -> Option<(usize, usize)> {
        for (i, row) in self.squares.iter().enumerate() {
            for (j, square) in row.iter().enumerate() {
                if square.piece == Piece::King && square.color == Some(color) {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn hash(&self) -> u64 {
        // Implement a hashing function for the board state
        0 // Placeholder
    }

    fn current_turn(&self) -> Color {
        // Implement logic to determine the current turn
        Color::White // Placeholder
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        let mut new_squares = [[Square::new(Piece::Empty, None); 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                new_squares[i][j] = self.squares[i][j];
            }
        }
        Board {
            squares: new_squares,
            transposition_table: self.transposition_table.clone(),
            turn: self.turn,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.squares.iter() {
            for square in row.iter() {
                let symbol = match (square.piece, square.color) {
                    (_, None) | (Piece::Empty, _) => ".",
                    (Piece::King, Some(Color::White)) => "♔",
                    (Piece::Queen, Some(Color::White)) => "♕",
                    (Piece::Rook, Some(Color::White)) => "♖",
                    (Piece::Bishop, Some(Color::White)) => "♗",
                    (Piece::Knight, Some(Color::White)) => "♘",
                    (Piece::Pawn, Some(Color::White)) => "♙",
                    (Piece::King, Some(Color::Black)) => "♚",
                    (Piece::Queen, Some(Color::Black)) => "♛",
                    (Piece::Rook, Some(Color::Black)) => "♜",
                    (Piece::Bishop, Some(Color::Black)) => "♝",
                    (Piece::Knight, Some(Color::Black)) => "♞",
                    (Piece::Pawn, Some(Color::Black)) => "♟︎",
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Board, Color, Move, Piece, Position};

    #[test]
    fn test_get_valid_moves_white_pawn_initial_position() {
        let board = Board::new();
        let moves = board.get_valid_moves(Color::White);

        let expected_moves = vec![
            Move {
                from: Position { row: 1, col: 0 },
                to: Position { row: 2, col: 0 },
                piece: Piece::Pawn,
                captured: None,
                score: 0,
            },
            Move {
                from: Position { row: 1, col: 1 },
                to: Position { row: 2, col: 1 },
                piece: Piece::Pawn,
                captured: None,
                score: 0,
            },
            Move {
                from: Position { row: 1, col: 2 },
                to: Position { row: 2, col: 2 },
                piece: Piece::Pawn,
                captured: None,
                score: 0,
            },
            Move {
                from: Position { row: 1, col: 3 },
                to: Position { row: 2, col: 3 },
                piece: Piece::Pawn,
                captured: None,
                score: 0,
            },
            Move {
                from: Position { row: 1, col: 4 },
                to: Position { row: 2, col: 4 },
                piece: Piece::Pawn,
                captured: None,
                score: 0,
            },
            Move {
                from: Position { row: 1, col: 5 },
                to: Position { row: 2, col: 5 },
                piece: Piece::Pawn,
                captured: None,
                score: 0,
            },
            Move {
                from: Position { row: 1, col: 6 },
                to: Position { row: 2, col: 6 },
                piece: Piece::Pawn,
                captured: None,
                score: 0,
            },
            Move {
                from: Position { row: 1, col: 7 },
                to: Position { row: 2, col: 7 },
                piece: Piece::Pawn,
                captured: None,
                score: 0,
            },
        ];

        for expected_move in expected_moves {
            assert!(moves.contains(&expected_move));
        }
    }
}
