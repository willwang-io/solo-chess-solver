use dioxus::prelude::*;

use crate::piece::PieceType;

#[component]
pub fn PieceSelectionBoard(selected: Signal<Option<usize>>) -> Element {
    rsx! {
        form {
            p { "Select the piece to place on the board. Right-click to clear it." }
            for (idx, piece) in PieceType::ALL.iter().enumerate() {
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
                        src: piece.get_white_icon(),
                        alt: piece.to_string(),
                        draggable: "false",
                    }
                }
            }
        }
    }
}
