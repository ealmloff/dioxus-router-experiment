use dioxus::prelude::*;

#[inline_props]
pub fn Route1(cx: Scope, dynamic: String) -> Element {
    render! {div {
        "Route1 {{
            dynamic: {dynamic}
        }}"
    }}
}
