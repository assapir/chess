#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty,
}

impl Piece {
    pub const PAWN_TABLE: [[i32; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [5, 5, 5, 5, 5, 5, 5, 5],
        [1, 1, 2, 3, 3, 2, 1, 1],
        [0, 0, 0, 2, 2, 0, 0, 0],
        [0, 0, 0, 1, 1, 0, 0, 0],
        [1, 1, 1, -1, -1, 1, 1, 1],
        [1, 2, 2, -2, -2, 2, 2, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];

    pub const KNIGHT_TABLE: [[i32; 8]; 8] = [
        [-5, -4, -3, -3, -3, -3, -4, -5],
        [-4, -2, 0, 0, 0, 0, -2, -4],
        [-3, 0, 1, 1, 1, 1, 0, -3],
        [-3, 0, 1, 2, 2, 1, 0, -3],
        [-3, 0, 1, 2, 2, 1, 0, -3],
        [-3, 0, 1, 1, 1, 1, 0, -3],
        [-4, -2, 0, 0, 0, 0, -2, -4],
        [-5, -4, -3, -3, -3, -3, -4, -5],
    ];

    pub const BISHOP_TABLE: [[i32; 8]; 8] = [
        [-2, -1, -1, -1, -1, -1, -1, -2],
        [-1, 0, 0, 0, 0, 0, 0, -1],
        [-1, 0, 0, 1, 1, 0, 0, -1],
        [-1, 0, 1, 1, 1, 1, 0, -1],
        [-1, 0, 1, 1, 1, 1, 0, -1],
        [-1, 0, 0, 1, 1, 0, 0, -1],
        [-1, 0, 0, 0, 0, 0, 0, -1],
        [-2, -1, -1, -1, -1, -1, -1, -2],
    ];

    pub const ROOK_TABLE: [[i32; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 2, 2, 2, 2, 2, 2, 1],
        [-1, 0, 0, 0, 0, 0, 0, -1],
        [-1, 0, 0, 0, 0, 0, 0, -1],
        [-1, 0, 0, 0, 0, 0, 0, -1],
        [-1, 0, 0, 0, 0, 0, 0, -1],
        [-1, 0, 0, 0, 0, 0, 0, -1],
        [0, 0, 0, 1, 1, 0, 0, 0],
    ];

    pub const QUEEN_TABLE: [[i32; 8]; 8] = [
        [-2, -1, -1, 0, 0, -1, -1, -2],
        [-1, 0, 0, 0, 0, 0, 0, -1],
        [-1, 0, 1, 1, 1, 1, 0, -1],
        [0, 0, 1, 1, 1, 1, 0, 0],
        [0, 0, 1, 1, 1, 1, 0, 0],
        [-1, 1, 1, 1, 1, 1, 0, -1],
        [-1, 0, 1, 0, 0, 0, 0, -1],
        [-2, -1, -1, 0, 0, -1, -1, -2],
    ];

    pub const KING_TABLE: [[i32; 8]; 8] = [
        [-3, -4, -4, -5, -5, -4, -4, -3],
        [-3, -4, -4, -5, -5, -4, -4, -3],
        [-3, -4, -4, -5, -5, -4, -4, -3],
        [-3, -4, -4, -5, -5, -4, -4, -3],
        [-2, -3, -3, -4, -4, -3, -3, -2],
        [-1, -2, -2, -2, -2, -2, -2, -1],
        [2, 2, 0, 0, 0, 0, 2, 2],
        [2, 3, 1, 0, 0, 1, 3, 2],
    ];

    pub fn directions(&self) -> Vec<(isize, isize)> {
        match self {
            Piece::King => vec![
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ],
            Piece::Queen => vec![
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1), // Rook-like moves
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1), // Bishop-like moves
            ],
            Piece::Rook => vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
            Piece::Bishop => vec![(1, 1), (1, -1), (-1, 1), (-1, -1)],
            Piece::Knight => vec![
                (2, 1),
                (2, -1),
                (-2, 1),
                (-2, -1),
                (1, 2),
                (1, -2),
                (-1, 2),
                (-1, -2),
            ],
            Piece::Pawn => vec![], // Pawn moves are handled separately
            Piece::Empty => vec![],
        }
    }

    pub fn table(&self) -> &'static [[i32; 8]; 8] {
        match self {
            Piece::King => &Piece::KING_TABLE,
            Piece::Queen => &Piece::QUEEN_TABLE,
            Piece::Rook => &Piece::ROOK_TABLE,
            Piece::Bishop => &Piece::BISHOP_TABLE,
            Piece::Knight => &Piece::KNIGHT_TABLE,
            Piece::Pawn => &Piece::PAWN_TABLE,
            Piece::Empty => &[[0; 8]; 8],
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            Piece::King => 900,
            Piece::Queen => 90,
            Piece::Rook => 50,
            Piece::Bishop | Piece::Knight => 30,
            Piece::Pawn => 10,
            Piece::Empty => 0,
        }
    }
}
