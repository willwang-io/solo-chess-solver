use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Bishop,
    Rook,
    Knight,
    Pawn,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_name = match self {
            Piece::King => "King",
            Piece::Queen => "Queen",
            Piece::Bishop => "Bishop",
            Piece::Rook => "Rook",
            Piece::Knight => "Knight",
            Piece::Pawn => "Pawn",
        };
        write!(f, "{}", display_name)
    }
}

impl Piece {
    pub const ALL: [Piece; 6] = [
        Piece::King,
        Piece::Queen,
        Piece::Bishop,
        Piece::Rook,
        Piece::Knight,
        Piece::Pawn,
    ];
}
