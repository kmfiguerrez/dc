use dioxus::prelude::*;

use crate::components::accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};

#[component]
pub fn AccordionPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-y-8",
            div {
                h1 { class: "text-3xl font-bold", "Accordion" }
                p { "An accordion component for displaying collapsible content sections." }
            }

            Accordion {
                AccordionItem { value: "dioxus",

                    AccordionTrigger { "What is Dioxus?" }

                    AccordionContent {
                        p { "Dioxus is a Rust framework for building user interfaces." }
                    }
                }

                AccordionItem { value: "react",

                    AccordionTrigger { "Is it similar to React?" }

                    AccordionContent {
                        p { "Yes. Both use component-based UI concepts." }
                    }
                }
            }
        
        }
    }
}