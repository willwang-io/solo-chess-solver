use dioxus::prelude::*;

use crate::step::Step;
use crate::ui::step_colors::STEP_COLORS;

#[component]
pub fn Solution(steps: Vec<Step>, selected_step: Signal<Option<usize>>) -> Element {
    let sans = to_sans(&steps);
    let selected = selected_step();
    rsx! {
        p {
            strong { "Steps: " }
            for (idx, step) in sans.iter().enumerate() {
                span {
                    key: "{idx}",
                    class: "{step_class(selected, idx)}",
                    style: "color: {STEP_COLORS[idx % STEP_COLORS.len()]};",
                    onmouseenter: move |_| selected_step.set(Some(idx)),
                    onmouseleave: move |_| selected_step.set(None),
                    "{step} "
                }
            }
        }
        p { class: "step-hint", "Tip: hover a step to highlight its arrow on the board." }
    }
}

fn to_sans(steps: &[Step]) -> Vec<String> {
    let mut sans_step = vec![];

    let numeric_to_sans = |r: usize, c: usize| -> String {
        format!("{}{}", (c as u8 + b'a') as char, 8 - r)
    };

    for Step { from: (fr, fc), to: (tr, tc), piece_type } in steps {
        let tmp = format!("{}{}x{}", piece_type.to_string(), numeric_to_sans(*fr, *fc), numeric_to_sans(*tr, *tc));
        sans_step.push(tmp);
    }
    sans_step
}

fn step_class(selected: Option<usize>, idx: usize) -> &'static str {
    match selected {
        Some(selected_idx) if selected_idx == idx => "step-token active",
        Some(_) => "step-token dim",
        None => "step-token",
    }
}
