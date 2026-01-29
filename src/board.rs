use crate::piece::Piece;

const N: usize = 8;
const SIZE: usize = N * N;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    cells: [Option<Piece>; SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [None; SIZE],
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
            .filter_map(|(i, p)| p.as_ref().map(|p| (i / N, i % N, p)))
    }

    pub fn count_pieces(&self) -> usize {
        self.pieces().count()
    }

    pub fn sum_move_left(&self) -> usize {
        self.pieces().map(|(_, _, p)| p.move_left()).sum()
    }

    pub fn has_king(&self) -> bool {
        self.pieces().any(|(_, _, p)| p.is_king())
    }

    pub fn single_is_king(&self) -> bool {
        let mut it = self.pieces();
        let Some((_, _, p)) = it.next() else { return false; };
        it.next().is_none() && p.is_king()
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
        assert!(board.get_cell(2, 6).is_some());
        assert_eq!(1, board.get_cell(2, 6).unwrap().move_left());
        assert_eq!(PieceType::Bishop, board.get_cell(2, 6).unwrap().piece_type);
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
