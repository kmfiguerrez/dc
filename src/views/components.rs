use crate::Route;
use super::Sidebar;
use dioxus::prelude::*;

// This component is a layout.
#[component]
pub fn Components() -> Element {
    rsx! {
        div { class: "flex border border-red-500",
            div { class: "basis-[20vw]",
                Sidebar {}
                {}
            }
            div { Outlet::<Route> {} }
        
        }
    }
}

