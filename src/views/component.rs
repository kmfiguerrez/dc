use dioxus::prelude::*;

use crate::components::ButtonPage;

#[component]
pub fn Component(name: String) -> Element {
    rsx! {
        if name == "button" {
            ButtonPage {}
        } else {
            "{name}"
        }
    }
}
