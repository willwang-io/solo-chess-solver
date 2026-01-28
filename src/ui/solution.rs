use dioxus::prelude::*;

#[component]
pub fn Solution(steps: Vec<(usize, usize, usize, usize)>) -> Element {
    rsx! {
        h2 { "Steps" }
        p { {format!("{:?}", steps)} }
    }
}
