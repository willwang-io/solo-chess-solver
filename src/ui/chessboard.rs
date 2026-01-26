use dioxus::prelude::*;

use crate::cell::Cell;

#[component]
pub fn Chessboard(
    board: Signal<Vec<Vec<Cell>>>,
    on_square_click: EventHandler<(usize, usize)>,
    on_square_right_click: EventHandler<(usize, usize)>,
) -> Element {
    rsx! {
        div {
            class: "chessboard",
            for r in 0..8 {
                for c in 0..8 {
                    Square {
                        key: "{r}-{c}",
                        board,
                        r, c,
                        on_square_click,
                        on_square_right_click,
                    }
                }
            }
        }
    }
}

#[component]
fn Square(
    board: Signal<Vec<Vec<Cell>>>,
    r: usize,
    c: usize,
    on_square_click: EventHandler<(usize, usize)>,
    on_square_right_click: EventHandler<(usize, usize)>,
) -> Element {
    let cell = board.read()[r][c];

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
                let Some((fr, fc)) = e
                    .data_transfer()
                    .get_data("text/plain")
                    .and_then(|s| {
                        let (r, c) = s.split_once(',')?;
                        Some((r.parse::<usize>().ok()?, c.parse::<usize>().ok()?))
                    })
                else {
                    return;
                };

                let mut b = board.write();
                if fr >= b.len() || fc >= b[0].len() { return; }

                if let Some(p) = b[fr][fc].piece() {
                    b[fr][fc].clear_cell();
                    b[r][c].set_cell(p);
                }
            },

            if let Some(piece) = cell.piece() {
                img {
                    src: piece.get_icon(),
                    alt: piece.to_string(),
                    class: "piece",
                    draggable: "true",
                    ondragstart: move |e| {
                        let _ = e.data_transfer().set_data("text/plain", &format!("{r},{c}"));
                    },
                }
            }
        }
    }
}
