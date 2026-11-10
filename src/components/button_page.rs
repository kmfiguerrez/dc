use dioxus::prelude::*;

use crate::components::button::{Button, ButtonVariant, ButtonSize};

#[component]
pub fn ButtonPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-y-5",

            div {
                h1 { class: "text-3xl font-bold", "Button" }
                p { "A button component for triggering actions or events when clicked." }
            }

            div {
                h2 { class: "text-xl font-bold", "Variants" }
                p { class: "mb-4",
                    "Use the "
                    code { "variant" }
                    " prop to change the variant of the button."
                }
                div { class: "flex items-center gap-x-3",
                    Button { variant: ButtonVariant::Primary, "Primary" }
                    Button { variant: ButtonVariant::Secondary, "Secondary" }
                    Button { variant: ButtonVariant::Destructive, "Destructive" }
                
                }
            }

            div {
                h2 { class: "text-xl font-bold", "Size" }
                p { class: "mb-4",
                    "Use the "
                    code { "size" }
                    " prop to change the size of the button."
                }
                div { class: "flex items-center gap-x-3",
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Small,
                        "Small"
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Medium,
                        "Medium"
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Large,
                        "Large"
                    }
                }
            }
        
        // Button { variant: ButtonVariant::Primary, "Save" }

        // Button {
        //     variant: ButtonVariant::Destructive,
        //     size: ButtonSize::Large,
        //     on_click: move |_| {
        //         println!("Delete clicked!");
        //     },
        //     "Delete"
        // }

        // Button { disabled: true, "Disabled" }
        }
    }
}
