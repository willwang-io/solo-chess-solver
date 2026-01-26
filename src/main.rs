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

    let mut board_state = use_signal(|| board);
    let mut selected_square = use_signal(|| Option::<(usize, usize)>::None);
    let selected_piece = use_signal(|| Option::<usize>::None);

    let on_square_click = move |(r, c): (usize, usize)| {
        selected_square.set(Some((r, c)));
        if let Some(p) = selected_piece() {
            board_state.with_mut(|b| {
                b[r][c].piece = Some(Piece::ALL[p]);
            });
        }
    };

    let on_square_right_click = move |(r, c): (usize, usize)| {
            board_state.with_mut(|b| {
                b[r][c].piece = None;
            });
    };

    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
        div {
            Chessboard {
                board: board_state,
                on_square_click,
                on_square_right_click,
            }
            PieceSelectionBoard { selected: selected_piece }
        }
    }
}
