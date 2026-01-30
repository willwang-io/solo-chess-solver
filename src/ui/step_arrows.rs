use dioxus::prelude::*;

use crate::step::Step;
use crate::ui::step_colors::STEP_COLORS;

const ARROW_HEAD_LEN: f32 = 0.35;
const ARROW_HEAD_WIDTH: f32 = 0.45;

#[component]
pub fn StepArrows(steps: Vec<Step>, selected_step: Signal<Option<usize>>) -> Element {
    let selected = selected_step();
    let arrow_lines: Vec<(usize, f32, f32, f32, f32)> = steps
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(idx, Step { from: (fr, fc), to: (tr, tc), .. })| {
            if fr == tr && fc == tc {
                return None;
            }
            let x1 = (fc as f32) + 0.5;
            let y1 = (fr as f32) + 0.5;
            let x2 = (tc as f32) + 0.5;
            let y2 = (tr as f32) + 0.5;
            let (x2, y2) = shorten_line(x1, y1, x2, y2, ARROW_HEAD_LEN);
            Some((idx, x1, y1, x2, y2))
        })
        .collect();
    rsx! {
        svg {
            class: "chessboard-overlay",
            view_box: "0 0 8 8",
            preserve_aspect_ratio: "none",
            defs {
                for (idx, color) in STEP_COLORS.iter().enumerate() {
                    marker {
                        id: "arrowhead-{idx}",
                        marker_units: "userSpaceOnUse",
                        marker_width: "{ARROW_HEAD_LEN}",
                        marker_height: "{ARROW_HEAD_WIDTH}",
                        ref_x: "0",
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
            for (idx, x1, y1, x2, y2) in arrow_lines {
                line {
                    key: "{idx}",
                    class: "{arrow_class(selected, idx)}",
                    style: "stroke: {step_color(idx)};",
                    x1: "{x1}",
                    y1: "{y1}",
                    x2: "{x2}",
                    y2: "{y2}",
                    marker_end: "url(#arrowhead-{color_index(idx)})",
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

fn shorten_line(x1: f32, y1: f32, x2: f32, y2: f32, head_len: f32) -> (f32, f32) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt();
    if len <= 0.0001 {
        return (x2, y2);
    }
    let scale = head_len / len;
    (x2 - dx * scale, y2 - dy * scale)
}

fn arrow_class(selected: Option<usize>, idx: usize) -> &'static str {
    match selected {
        Some(selected_idx) if selected_idx == idx => "step-arrow active",
        Some(_) => "step-arrow dim",
        None => "step-arrow",
    }
}
