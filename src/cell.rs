use crate::piece::Piece;
use core::fmt;

pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub piece: Option<Piece>,
}

impl fmt::Display for Cell {
    // Note: Coordinates are 0-indexed with (0,0) at the bottom-left (a1) and (7,7) at the top-right (h8).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_assert!(self.row < 8 && self.col < 8);
        let file = (b'a' + self.col as u8) as char;
        let rank = self.row + 1;
        write!(f, "{}{}", file, rank)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chessboard_coordinates_conversion() {
        let a1 = Cell {
            row: 0,
            col: 0,
            piece: None,
        };
        let h8 = Cell {
            row: 7,
            col: 7,
            piece: None,
        };
        let e4 = Cell {
            row: 3,
            col: 4,
            piece: None,
        };

        assert_eq!("a1", a1.to_string());
        assert_eq!("h8", h8.to_string());
        assert_eq!("e4", e4.to_string());
    }
}
