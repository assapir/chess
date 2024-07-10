pub use board::Board;
pub use piece::Piece;

pub mod board;
pub mod piece;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub struct Square {
    pub piece: Piece,
    pub color: Option<Color>, // Use Option to represent empty squares
}

impl Square {
    pub fn new(piece: Piece, color: Option<Color>) -> Self {
        Square { piece, color }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub piece: Piece,
    pub captured: Option<Piece>,
    pub score: i32,
}
