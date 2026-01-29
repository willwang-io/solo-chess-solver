use crate::piece::PieceType;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Step {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub piece_type: PieceType,
}
