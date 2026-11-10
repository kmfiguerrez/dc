use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    let components = [
        "Button", "Card", "Switch", "Accordion", "Dropdown", "Popover", "Portal", "Dialog", "Sheet", "AlertDialog"
    ];

    rsx! {
        div { class: "flex flex-col border border-black",
            // Link {
            //     to: Route::Component {
            //         name: "accordion".to_string(),
            //     },
            //     "Button"
            // }
            // Link {
            //     to: Route::Component {
            //         name: "alert".to_string(),
            //     },
            //     "Card"
            // }
            for component in components {
                {
                    let name = component.to_lowercase();
                    rsx! {
                        Link { to: Route::Component { name }, "{component}" }
                    }
                }
            }
        }
    }
}
