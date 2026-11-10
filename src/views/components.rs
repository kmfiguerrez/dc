use crate::Route;
use super::Sidebar;
use dioxus::prelude::*;

#[component]
pub fn Components() -> Element {
    rsx! {
        div { class: "grid grid-cols-2 border border-red-500",
            // Link { to: Route::Home {}, "Home" }
            // Link { to: Route::Blog { id: 1 }, "Components" }
            // div { "comlumn 1" }
            Sidebar {}
            div { Outlet::<Route> {} }
        
        }
    }
}

