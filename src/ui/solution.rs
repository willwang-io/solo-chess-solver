use dioxus::prelude::*;

use crate::step::Step;
use crate::ui::step_colors::STEP_COLORS;

#[component]
pub fn Solution(steps: Vec<Step>) -> Element {
    let sans = to_sans(&steps);
    rsx! {
        p {
            strong { "Steps: " }
            for (idx, step) in sans.iter().enumerate() {
                span {
                    key: "{idx}",
                    style: "color: {STEP_COLORS[idx % STEP_COLORS.len()]};",
                    "{step} "
                }
            }
        }
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
