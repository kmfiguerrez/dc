use dioxus::prelude::*;

use crate::components::{
    switch::Switch,
    stack::{VStack, HStack}
};

#[component]
pub fn SwitchPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-y-8",

            div {
                h1 { class: "text-3xl font-bold", "Switch" }
                p { "A togglable switch component" }
            }

            div {
                h2 { class: "text-xl font-bold mb-4", "Controlled usage" }

                Settings {}
            }

            div {
                h2 { class: "text-xl font-bold mb-4", "Uncontrolled usage" }

                VStack { gap: 16,

                    Switch { label: "Enable notifications", default_checked: true }

                    Switch { label: "Enable dark mode", default_checked: false }
                }
            }

            div {
                h2 { class: "text-xl font-bold", "Disabled" }
                p { class: " mb-4",
                    "Add the "
                    code { "disabled" }
                    " prop to the "
                    code { "Switch" }
                    " component to disable the switch."
                }

                Switch { label: "Disabled", disabled: true }
            
            }
        }
    }
}

#[component]
fn Settings() -> Element {
    let mut notifications = use_signal(|| false);

    rsx! {
        VStack { gap: 16,

            HStack { gap: 12,

                Switch {
                    label: "Enable notifications",

                    checked: Some(notifications()),

                    on_checked_change: move |value| {
                        notifications.set(value);
                    },
                }

                p {
                    "Notifications: "
                    if notifications() {
                        "Enabled"
                    } else {
                        "Disabled"
                    }
                }
            }
        }
    }
}