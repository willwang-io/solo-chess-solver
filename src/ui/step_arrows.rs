use dioxus::prelude::*;

use crate::step::Step;

#[component]
pub fn StepArrows(steps: Vec<Step>) -> Element {
    rsx! {
        svg {
            class: "chessboard-overlay",
            view_box: "0 0 8 8",
            preserve_aspect_ratio: "none",
            defs {
                marker {
                    id: "arrowhead",
                    marker_units: "strokeWidth",
                    marker_width: "4",
                    marker_height: "4",
                    ref_x: "9",
                    ref_y: "5",
                    orient: "auto",
                    view_box: "0 0 10 10",
                    path {
                        d: "M 0 0 L 10 5 L 0 10 z",
                        style: "fill: var(--step-arrow); fill-opacity: var(--step-arrow-opacity);",
                    }
                }
            }
            for (idx, Step { from: (fr, fc), to: (tr, tc), .. }) in
                steps.iter().copied().enumerate()
            {
                if fr != tr || fc != tc {
                    line {
                        key: "{idx}",
                        class: "step-arrow",
                        x1: "{(fc as f32) + 0.5}",
                        y1: "{(fr as f32) + 0.5}",
                        x2: "{(tc as f32) + 0.5}",
                        y2: "{(tr as f32) + 0.5}",
                        marker_end: "url(#arrowhead)",
                    }
                }
            }
        }
    }
}
