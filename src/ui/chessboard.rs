use dioxus::prelude::*;

use crate::board::Board;

#[component]
pub fn Chessboard(
    board: Signal<Board>,
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
    board: Signal<Board>,
    r: usize,
    c: usize,
    on_square_click: EventHandler<(usize, usize)>,
    on_square_right_click: EventHandler<(usize, usize)>,
) -> Element {
    let cell = board.read().get_cell(r, c);
    let rank_label = 8 - r;
    let file_label = (b'a' + c as u8) as char;

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

                let fr = f_idx / 8;
                let fc = f_idx % 8;
                if b.get_cell(fr, fc).is_some() {
                    b.move_piece(fr, fc, r, c);
                }
            },

            if c == 0 {
                span { class: "square-label rank-label", "{rank_label}" }
            }

            if r == 7 {
                span { class: "square-label file-label", "{file_label}" }
            }

            if let Some(piece) = cell {
                img {
                    src: piece.get_icon(),
                    alt: piece.to_string(),
                    class: "piece",
                    draggable: "true",
                    ondragstart: move |e| {
                        let _ = e.data_transfer().set_data("text/plain", &(r * 8 + c).to_string());
                    },
                }
            }
        }
    }
}
