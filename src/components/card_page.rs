use dioxus::prelude::*;

use crate::components::{
    button::{Button, ButtonVariant},
    stack::{VStack, HStack,},
    card::Card
};

#[component]
pub fn CardPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-y-5",

            div {
                h1 { class: "text-3xl font-bold", "Card" }
                p { "A simple card component" }
            }

            VStack { gap: 24,

                Card {
                    title: Some("Account settings".to_string()),
                    class: "max-w-fit",

                    VStack { gap: 12,

                        p { "Manage your account preferences." }

                        HStack { gap: 8,

                            Button { variant: ButtonVariant::Primary, "Save changes" }

                            Button { variant: ButtonVariant::Secondary, "Cancel" }
                        }
                    }
                }

                Card {
                    title: Some("Danger zone".to_string()),
                    class: "max-w-fit",

                    VStack { gap: 12,

                        p { "Deleting your account is permanent." }

                        HStack {
                            Button { variant: ButtonVariant::Destructive, "Delete account" }
                        }
                    }
                }
            }
        }
    }
}