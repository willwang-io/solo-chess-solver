use core::fmt;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

    pub fn get_icon(&self) -> Asset {
        match self {
            Piece::King => asset!("/assets/img/white/king.png"),
            Piece::Queen => asset!("/assets/img/white/queen.png"),
            Piece::Bishop => asset!("/assets/img/white/bishop.png"),
            Piece::Rook => asset!("/assets/img/white/rook.png"),
            Piece::Knight => asset!("/assets/img/white/knight.png"),
            Piece::Pawn => asset!("/assets/img/white/pawn.png"),
        }
    }
}
