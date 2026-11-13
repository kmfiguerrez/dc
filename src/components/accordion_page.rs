use dioxus::prelude::*;

use crate::components::{
    button::{Button, ButtonVariant},
    stack::HStack,
    accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent, AccordionMode}
};

#[component]
pub fn AccordionPage() -> Element {
    rsx! {
        div { class: "flex flex-col gap-y-8",
            div {
                h1 { class: "text-3xl font-bold", "Accordion" }
                p { "An accordion component for displaying collapsible content sections." }
            }

            ControlledExample {}
            UncontrolledExample {}
            Mode {}
        
        }
    }
}

#[component]
fn ControlledExample() -> Element {
    let mut open_items = use_signal(|| {
        vec!["rust".to_string()]
    });

    rsx! {
        div {
            h2 { class: "text-2xl font-bold", "Controlled" }
            p { class: "mb-4", "The parent component controls the Accordion." }
            HStack { class: "mb-4",
                Button {
                    onclick: move |_| {
                        open_items.set(vec![
                            "dioxus".to_string(),
                        ]);
                    },
                    "Open Dioxus item"
                }

                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| {
                        open_items.set(vec![]);
                    },
                    "Close all items"
                }
            }

            Accordion {
                mode: AccordionMode::Multiple,

                // into() converts open_items as Signal into ReadSignal.
                open: Some(open_items.into()),

                on_open_change: move |next_items| {
                    open_items.set(next_items);
                },

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

#[component]
fn UncontrolledExample() -> Element {
    rsx! {
        div {
            h2 { class: "text-2xl font-bold", "Uncontrolled" }
            p { class: "mb-4",
                "The component manages it's own state. By default, the Accordion is uncontrolled."
            }

            div {
                // h3 { class: "text-xl font-bold", "Single-open" }
                // p { class: "mb-4",
                //     "Use the "
                //     code { "mode" }
                //     " prop to change the mode of the Accordion."
                // }
                Accordion { class: "mb-8",
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
}

fn Mode() -> Element {
    rsx! {
        div {
            h2 { class: "text-2xl font-bold", "Mode" }
            p { class: "mb-4",
                "Use the "
                code { "mode" }
                " property to change the mode of Accordion"
            }

            div {
                h3 { class: "text-xl font-bold", "Single-open" }
                p { class: "mb-4",
                    "Use the "
                    code { "mode" }
                    " prop to change the mode of the Accordion."
                }

                Accordion { class: "mb-8",
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
                h3 { class: "text-xl font-bold", "Multiple-open" }
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