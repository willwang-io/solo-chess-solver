use dioxus::prelude::*;

use crate::cell::Cell;

#[component]
pub fn Chessboard(
    board: Signal<Vec<Vec<Cell>>>,
    // on_place: EventHandler<(usize, usize)>,
) -> Element {
    let mut row_selected = use_signal(|| 0usize);
    let mut col_selected = use_signal(|| 0usize);
    let board = board.read();

    rsx! {
        div {
            style: "display:grid;grid-template-columns:repeat(8,48px);grid-template-rows:repeat(8,48px);gap:0;border:2px solid #222;width:fit-content;",
            for (r, row) in board.iter().enumerate() {
                for (c, cell) in row.iter().enumerate() {
                    button {
                        key: "{r}-{c}",
                        r#type: "button",
                        onclick: move |_| {
                            *row_selected.write() = r;
                            *col_selected.write() = c;
                            println!("Cell {r} {c} is clicked");
                        },
                    },
                }
            }
        }
        p { "Cell selected: {board[row_selected()][col_selected()].to_string()}"}
    }
}
