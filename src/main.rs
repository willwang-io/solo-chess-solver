use dioxus::prelude::*;

mod board;
mod piece;
mod solver;
mod step;
mod ui;

use board::Board;
use piece::{Piece, PieceType};
use solver::solo_chess_solver;
use ui::chessboard::Chessboard;
use ui::piece_selection::PieceSelectionBoard;
use ui::solution::Solution;
use ui::step_arrows::StepArrows;

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

    let mut solver_board = board_state();
    let steps = solo_chess_solver(&mut solver_board);
    let selected_step = use_signal(|| Option::<usize>::None);

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Meta { name: "author", content: "Jiaye Will Wang" }
        document::Meta { name: "description", content: "Solve Solo-Chess puzzles from chess.com. Place pieces, run the solver, and step through the capture sequence that leaves one piece." }
        document::Meta { property: "og:site_name", content: "Solo-Chess Solver" }
        document::Meta { property: "og:title", content: "Solo-Chess Solver" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:image", content: asset!("/assets/og.png") }
        document::Meta { property: "og:image:width", content: "1200" }
        document::Meta { property: "og:image:height", content: "630" }
        document::Meta { property: "og:image:type", content: "image/png" }
        document::Link { rel: "icon", href: asset!("/assets/favicon.svg") }
        document::Link { rel: "stylesheet", href: STYLE }
        main {
            h1 { "Solo-Chess Solver" }
            div {
                class: "board-stack",
                Chessboard {
                    board: board_state,
                    on_square_click,
                    on_square_right_click,
                }
                StepArrows {
                    steps: steps.clone(),
                    selected_step,
                }
            }
            PieceSelectionBoard { selected: selected_piece }
            Solution { steps, selected_step }
            section {
                h2 { "Rules" }
                p { "From chess.com: " }
                ul {
                    li { "Capture a piece with every move until just one remains." }
                    li { "No piece may capture more than 2 times per puzzle (shown in black if cannot move)." }
                    li { "If there is a King on the board, it must be the final piece." }
                }
            }
        }
    }
}
