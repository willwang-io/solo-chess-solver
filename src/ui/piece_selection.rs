use dioxus::prelude::*;

use crate::piece::PieceType;

#[component]
pub fn PieceSelectionBoard(selected: Signal<Option<usize>>) -> Element {
    rsx! {
        div {
            class: "piece-selection",
            p { "Select the piece to place on the board. Right-click to clear it." }
            div {
                class: "piece-toggle-group",
                for (idx, piece) in PieceType::ALL.iter().enumerate() {
                    button {
                        class: if selected() == Some(idx) { "piece-toggle selected" } else { "piece-toggle" },
                        r#type: "button",
                        aria_pressed: selected() == Some(idx),
                        onclick: move |_| {
                            selected.set(if selected() == Some(idx) { None } else { Some(idx) })
                        },
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
}
