use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Component(name: String) -> Element {
    rsx! {
        button { "{name}" }
    }
}
