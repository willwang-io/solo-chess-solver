use dioxus::prelude::*;

use crate::piece::Piece;

#[component]
pub fn PieceSelectionBoard(selected: Signal<Option<usize>>) -> Element {
    rsx! {
        form {
            p { "Select the piece to place on the board" }
            for (idx, piece) in Piece::ALL.iter().enumerate() {
                label {
                    input {
                        type: "radio",
                        name: "chess_piece",
                        value: "{piece}",
                        checked: selected() == Some(idx),
                        onclick: move |_| {
                            selected.set(if selected() == Some(idx) { None } else { Some(idx) })
                        }
                    }
                    img {
                        class: "piece",
                        src: piece.get_icon(),
                        alt: piece.to_string(),
                        draggable: "false",
                    }
                }
            }
        }
    }
}
