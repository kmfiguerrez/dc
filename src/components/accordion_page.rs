use dioxus::prelude::*;

use crate::components::accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent, AccordionMode};

#[component]
pub fn AccordionPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-y-8",
            div {
                h1 { class: "text-3xl font-bold", "Accordion" }
                p { "An accordion component for displaying collapsible content sections." }
            }

            div {
                h2 { class: "text-xl font-bold", "Single-open" }
                p { class: "mb-4",
                    "Use the "
                    code { "mode" }
                    " prop to change the mode of the Accordion."
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

            div {
                h2 { class: "text-xl font-bold", "Multiple-open" }
                p { class: "mb-4",
                    "Use the "
                    code { "mode" }
                    " prop to change the mode of the Accordion."
                }

                Accordion { mode: AccordionMode::Multiple,

                    AccordionItem { value: "rust",

                        AccordionTrigger { "What is Rust?" }

                        AccordionContent {
                            p { "Rust is a systems programming language." }
                        }
                    }

                    AccordionItem { value: "dioxus",

                        AccordionTrigger { "What is Dioxus?" }

                        AccordionContent {
                            p { "Dioxus is a Rust UI framework." }
                        }
                    }
                }
            }
        
        }
    }
}