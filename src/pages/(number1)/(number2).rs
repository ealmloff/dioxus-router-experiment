use dioxus::prelude::*;

#[inline_props]
pub fn Route3(cx: Scope, number1: u32, number2: u32) -> Element {
    render! {div {
        "Route3 {{
            number1: {number1},
            number2: {number2}
        }}"
    }}
}
