use dioxus::prelude::*;

use crate::components::{ButtonPage, CardPage, SwitchPage};

#[component]
pub fn Component(name: String) -> Element {
    rsx! {
        if name == "button" {
            ButtonPage {}
        } else if name == "card" {
            CardPage {}
        } else if name == "switch" {
            SwitchPage {}
        } else {
            "{name}"
        }
    }
}
