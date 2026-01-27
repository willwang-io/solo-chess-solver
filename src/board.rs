use crate::piece::Piece;

const N: usize = 8;

pub struct Board {
    cells: Vec<Option<Piece>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: (0..N * N).map(|_| None).collect(),
        }
    }

    pub fn set_cell(&mut self, r: usize, c: usize, p: Piece) {
        self.cells[r * N + c] = Some(p);
    }

    pub fn get_cell(&self, r: usize, c: usize) -> Option<Piece> {
        self.cells[r * N + c]
    }

    pub fn clear_cell(&mut self, r: usize, c: usize) {
        self.cells[r * N + c] = None;
    }

    pub fn move_piece(&mut self, fr: usize, fc: usize, tr: usize, tc: usize) {
        if fr == tr && fc == tc {
            return;
        }

        if let Some(mut p) = self.get_cell(fr, fc) {
            p.use_move();
            self.set_cell(tr, tc, p);
            self.clear_cell(fr, fc);
        }
    }

    pub fn pieces(&self) -> impl Iterator<Item = (usize, usize, &Piece)> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, p)| p.as_ref().map(|p| (i / 8, i % 8, p)))
    }

    pub fn count_pieces(&self) -> usize {
        self.pieces().count()
    }

    pub fn is_empty(&self, r: usize, c: usize) -> bool {
        self.get_cell(r, c).is_none()
    }
}

#[cfg(test)]
mod test {
    use crate::board::Board;
    use crate::piece::{Piece, PieceType};

    #[test]
    fn count_pieces() {
        let mut board = Board::new();
        assert_eq!(0, board.count_pieces());

        board.set_cell(0, 4, Piece::new(PieceType::Bishop));
        board.set_cell(5, 7, Piece::new(PieceType::Queen));
        assert_eq!(2, board.count_pieces());
    }

    #[test]
    fn move_piece() {
        let mut board = Board::new();
        board.set_cell(0, 4, Piece::new(PieceType::Bishop));
        board.move_piece(0, 4, 2, 6);

        assert!(board.get_cell(0, 4).is_none());
        assert_eq!(Some(Piece::new(PieceType::Bishop)), board.get_cell(2, 6));
    }

    #[test]
    fn get_pieces_and_count() {
        let mut board = Board::new();
        board.set_cell(0, 4, Piece::new(PieceType::Bishop));
        board.set_cell(3, 5, Piece::new(PieceType::Bishop));
        board.set_cell(4, 7, Piece::new(PieceType::Pawn));

        assert_eq!(3, board.count_pieces());
        let pieces: Vec<(usize, usize, Piece)> =
            board.pieces().map(|(r, c, p)| (r, c, *p)).collect();
        assert_eq!(
            vec![
                (0, 4, Piece::new(PieceType::Bishop)),
                (3, 5, Piece::new(PieceType::Bishop)),
                (4, 7, Piece::new(PieceType::Pawn))
            ],
            pieces
        );
    }
}
