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

    pub fn set_cell(&mut self, r: usize, c: usize, p: Option<Piece>) {
        self.cells[r * N + c] = p;
    }

    pub fn get_cell(&self, r: usize, c: usize) -> Option<Piece> {
        self.cells[r * N + c]
    }

    pub fn clear_cell(&mut self, r: usize, c: usize) {
        self.set_cell(r, c, None);
    }

    pub fn move_piece(&mut self, fr: usize, fc: usize, tr: usize, tc: usize) {
        assert!(!self.cells[fr * N + fc].is_none());

        self.set_cell(tr, tc, self.get_cell(fr, fc));
        self.clear_cell(fr, fc);
    }
}
