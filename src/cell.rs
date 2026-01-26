use crate::piece::Piece;
use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    // Note: Coordinates are 0-indexed with (0,0) at the bottom-left (a1) and (7,7) at the top-right (h8).
    pub row: usize,
    pub col: usize,
    piece: Option<Piece>,
    used_capture: usize,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_assert!(self.row < 8 && self.col < 8);
        let file = (b'a' + self.col as u8) as char;
        let rank = self.row + 1;
        write!(f, "{}{}", file, rank)
    }
}

impl Cell {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row, 
            col, 
            piece: None,
            used_capture: 2,
        }
    }

    pub fn clear_cell(&mut self) {
        self.piece = None;
    }

    pub fn set_cell(&mut self, piece: Piece) {
        self.piece = Some(piece);
    }

    pub fn piece(&self) -> Option<Piece> {
        self.piece
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chessboard_coordinates_conversion() {
        let a1 = Cell::new(0, 0);
        let h8 = Cell::new(7, 7);
        let e4 = Cell::new(3, 4);

        assert_eq!("a1", a1.to_string());
        assert_eq!("h8", h8.to_string());
        assert_eq!("e4", e4.to_string());
    }
}
