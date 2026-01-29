use dioxus::prelude::*;

use crate::step::Step;

#[component]
pub fn Solution(steps: Vec<Step>) -> Element {
    rsx! {
        h2 { "Steps" }
        p { {format!("{:?}", steps)} }
    }
}
