use dioxus::prelude::*;

#[inline_props]
pub fn Route3(cx: Scope, dynamic: u32) -> Element {
    render! {div {
        "Route3 {{
            dynamic: {dynamic}
        }}"
    }}
}
