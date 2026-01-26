use dioxus::prelude::*;

mod cell;
mod piece;
mod solver;
mod ui;

use cell::Cell;
use piece::Piece;
use ui::chessboard::Chessboard;
use ui::piece_selection_board::PieceSelectionBoard;

const STYLE: Asset = asset!("/assets/style.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut board: Vec<Vec<Cell>> = (0..8)
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
    board[4][5] = Cell {
        row: 4,
        col: 5,
        piece: Some(Piece::Pawn),
    };
    let board_state = use_signal(|| board);
    let selected_piece = use_signal(|| Option::<usize>::None);

    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
        div {
            Chessboard {
                board: board_state,
            }
            PieceSelectionBoard { selected: selected_piece }
        }
    }
}
