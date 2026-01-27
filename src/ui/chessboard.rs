use dioxus::prelude::*;

use crate::cell::Cell;

#[component]
pub fn Chessboard(
    board: Signal<Vec<Cell>>,
    on_square_click: EventHandler<(usize, usize)>,
    on_square_right_click: EventHandler<(usize, usize)>,
) -> Element {
    rsx! {
        div {
            class: "chessboard",
            for i in 0..64 {
                Square {
                    key: "{i / 8}-{i % 8}",
                    board,
                    r: i / 8,
                    c: i % 8,
                    on_square_click,
                    on_square_right_click,
                }
            }
        }
    }
}

#[component]
fn Square(
    board: Signal<Vec<Cell>>,
    r: usize,
    c: usize,
    on_square_click: EventHandler<(usize, usize)>,
    on_square_right_click: EventHandler<(usize, usize)>,
) -> Element {
    let idx = r * 8 + c;
    let cell = board.read()[idx];

    rsx! {
        div {
            class: if (r + c) % 2 == 0 { "square light" } else { "square dark" },

            onclick: move |_| on_square_click.call((r, c)),

            oncontextmenu: move |e| {
                e.prevent_default();
                on_square_right_click.call((r, c));
            },

            ondragover: move |e| e.prevent_default(),

            ondrop: move |e| {
                e.prevent_default();
                let Some(f_idx) = e
                    .data_transfer()
                    .get_data("text/plain")
                    .and_then(|s| Some(s.parse::<usize>().ok()?))
                else {
                    return;
                };

                let mut b = board.write();
                if f_idx >= 64 { return; }

                if let Some(p) = b[f_idx].piece() {
                    b[f_idx].clear_cell();
                    b[idx].set_cell(p);
                }
            },

            if let Some(piece) = cell.piece() {
                img {
                    src: piece.get_icon(),
                    alt: piece.to_string(),
                    class: "piece",
                    draggable: "true",
                    ondragstart: move |e| {
                        let _ = e.data_transfer().set_data("text/plain", &idx.to_string());
                    },
                }
            }
        }
    }
}
