use core::fmt;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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
    move_left: usize,
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
        if self.move_left == 0 {
            self.piece_type.get_black_icon()
        } else {
            self.piece_type.get_white_icon()
        }
    }

    pub fn use_move(&mut self) {
        self.move_left = self.move_left.saturating_sub(1);
    }

    pub fn move_left(&self) -> usize {
        self.move_left
    }

    pub fn is_king(&self) -> bool {
        self.piece_type == PieceType::King
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_name = match self {
            PieceType::King => "K",
            PieceType::Queen => "Q",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Knight => "N",
            PieceType::Pawn => "",
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

    pub fn get_white_icon(&self) -> Asset {
        match self {
            PieceType::King => asset!("/assets/img/white/king.png"),
            PieceType::Queen => asset!("/assets/img/white/queen.png"),
            PieceType::Bishop => asset!("/assets/img/white/bishop.png"),
            PieceType::Rook => asset!("/assets/img/white/rook.png"),
            PieceType::Knight => asset!("/assets/img/white/knight.png"),
            PieceType::Pawn => asset!("/assets/img/white/pawn.png"),
        }
    }

    pub fn get_black_icon(&self) -> Asset {
        match self {
            PieceType::King => asset!("/assets/img/black/king.png"),
            PieceType::Queen => asset!("/assets/img/black/queen.png"),
            PieceType::Bishop => asset!("/assets/img/black/bishop.png"),
            PieceType::Rook => asset!("/assets/img/black/rook.png"),
            PieceType::Knight => asset!("/assets/img/black/knight.png"),
            PieceType::Pawn => asset!("/assets/img/black/pawn.png"),
        }
    }
}
