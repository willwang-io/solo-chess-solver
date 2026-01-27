use core::fmt;
use dioxus::prelude::*;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Rook,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub move_left: usize,
    pub piece_type: PieceType,
}

impl Piece {
    const DEFAULT_MOVE_LEFT: usize = 2; 

    pub fn new(piece_type: PieceType) -> Self {
        Self {
            move_left: Self::DEFAULT_MOVE_LEFT,
            piece_type,
        }
    }

    pub fn get_icon(&self) -> Asset {
        self.piece_type.get_icon()
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_name = match self {
            PieceType::King => "King",
            PieceType::Queen => "Queen",
            PieceType::Bishop => "Bishop",
            PieceType::Rook => "Rook",
            PieceType::Knight => "Knight",
            PieceType::Pawn => "Pawn",
        };
        write!(f, "{}", display_name)
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.piece_type.fmt(f)
    }
}

impl PieceType {
    pub const ALL: [PieceType; 6] = [
        PieceType::King,
        PieceType::Queen,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Knight,
        PieceType::Pawn,
    ];

    pub fn get_icon(&self) -> Asset {
        match self {
            PieceType::King => asset!("/assets/img/white/king.png"),
            PieceType::Queen => asset!("/assets/img/white/queen.png"),
            PieceType::Bishop => asset!("/assets/img/white/bishop.png"),
            PieceType::Rook => asset!("/assets/img/white/rook.png"),
            PieceType::Knight => asset!("/assets/img/white/knight.png"),
            PieceType::Pawn => asset!("/assets/img/white/pawn.png"),
        }
    }
}
