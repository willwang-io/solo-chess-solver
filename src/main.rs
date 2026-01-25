use dioxus::prelude::*;

mod cell;
mod piece;
mod ui;

use cell::Cell;
use piece::Piece;
use ui::chessboard::Chessboard;
use ui::piece_selection_board::PieceSelectionBoard;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let board: Vec<Vec<Cell>> = (0..8)
        .rev()
        .map(|row| {
            (0..8)
                .map(|col| Cell {
                    row,
                    col,
                    piece: None,
                })
                .collect()
        })
        .collect();
    let board_state = use_signal(|| board);
    let selected_piece = use_signal(|| Option::<usize>::None);

    rsx! {
        div {
            Chessboard {
                board: board_state,
            }
            PieceSelectionBoard { selected: selected_piece }
        }
    }
}
