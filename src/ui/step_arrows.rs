use dioxus::prelude::*;

use crate::step::Step;
use crate::ui::step_colors::STEP_COLORS;

#[component]
pub fn StepArrows(steps: Vec<Step>, selected_step: Signal<Option<usize>>) -> Element {
    let selected = selected_step();
    rsx! {
        svg {
            class: "chessboard-overlay",
            view_box: "0 0 8 8",
            preserve_aspect_ratio: "none",
            defs {
                for (idx, color) in STEP_COLORS.iter().enumerate() {
                    marker {
                        id: "arrowhead-{idx}",
                        marker_units: "strokeWidth",
                        marker_width: "4",
                        marker_height: "4",
                        ref_x: "9",
                        ref_y: "5",
                        orient: "auto",
                        view_box: "0 0 10 10",
                        path {
                            d: "M 0 0 L 10 5 L 0 10 z",
                            style: "fill: {color}; fill-opacity: var(--step-arrow-opacity);",
                        }
                    }
                }
            }
            for (idx, Step { from: (fr, fc), to: (tr, tc), .. }) in
                steps.iter().copied().enumerate()
            {
                if fr != tr || fc != tc {
                    line {
                        key: "{idx}",
                        class: "{arrow_class(selected, idx)}",
                        style: "stroke: {step_color(idx)};",
                        x1: "{(fc as f32) + 0.5}",
                        y1: "{(fr as f32) + 0.5}",
                        x2: "{(tc as f32) + 0.5}",
                        y2: "{(tr as f32) + 0.5}",
                        marker_end: "url(#arrowhead-{color_index(idx)})",
                    }
                }
            }
        }
    }
}

fn color_index(idx: usize) -> usize {
    idx % STEP_COLORS.len()
}

fn step_color(idx: usize) -> &'static str {
    STEP_COLORS[color_index(idx)]
}

fn arrow_class(selected: Option<usize>, idx: usize) -> &'static str {
    match selected {
        Some(selected_idx) if selected_idx == idx => "step-arrow active",
        Some(_) => "step-arrow dim",
        None => "step-arrow",
    }
}
