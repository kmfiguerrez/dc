use crate::Route;
use super::Sidebar;
use dioxus::prelude::*;

// This component is a layout.
#[component]
pub fn Components() -> Element {
    rsx! {
        div { class: "grid grid-cols-2 border border-red-500",
            Sidebar {}
            div { Outlet::<Route> {} }
        
        }
    }
}

