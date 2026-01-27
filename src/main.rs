use dioxus::prelude::*;

mod board;
mod piece;
mod solver;
mod ui;

use board::Board;
use piece::{Piece, PieceType};
use ui::chessboard::Chessboard;
use ui::piece_selection_board::PieceSelectionBoard;

const STYLE: Asset = asset!("/assets/style.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let board = Board::new();

    let mut board_state = use_signal(|| board);
    let mut selected_square = use_signal(|| Option::<(usize, usize)>::None);
    let selected_piece = use_signal(|| Option::<usize>::None);

    let on_square_click = move |(r, c): (usize, usize)| {
        selected_square.set(Some((r, c)));
        if let Some(p) = selected_piece() {
            board_state.with_mut(|b| {
                b.set_cell(r, c, Piece::new(PieceType::ALL[p]));
            });
        }
    };

    let on_square_right_click = move |(r, c): (usize, usize)| {
        board_state.with_mut(|b| {
            b.clear_cell(r, c);
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
