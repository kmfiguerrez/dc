use dioxus::prelude::*;

// Temporary CSS loading.
const ACCORDION_CSS: Asset = asset!("/assets/styling/accordion.css");

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AccordionMode {
    #[default]
    Single,
    Multiple,
}

#[derive(Clone, PartialEq)]
struct AccordionContext {
    mode: AccordionMode,
    open_items: Signal<Vec<String>>,
}

#[component]
pub fn Accordion(
    #[props(default)]
    mode: AccordionMode,

    children: Element,
) -> Element {
    let open_items = use_signal(|| Vec::<String>::new());

    use_context_provider(|| AccordionContext {
        mode,
        open_items,
    });

    rsx! {
        // Temporary CSS loading.
        document::Link { rel: "stylesheet", href: ACCORDION_CSS }

        div { class: "accordion", {children} }
    }
}

#[derive(Clone, PartialEq)]
struct AccordionItemContext {
    // value is supplied by the developer using the component.
    value: String,
}

#[component]
pub fn AccordionItem(
    value: String,
    children: Element,
) -> Element {
    use_context_provider(|| AccordionItemContext {
        value: value.clone(),
    });

    rsx! {
        div { class: "accordion-item", {children} }
    }
}

#[component]
pub fn AccordionTrigger(children: Element) -> Element {
    let accordion = use_context::<AccordionContext>();
    let item = use_context::<AccordionItemContext>();

    let is_open = accordion
        .open_items
        .read()
        .contains(&item.value);

    // item.value is cloned because it is captured in the closure.
    let item_value = item.value.clone();
    // accordion.mode is reassigned because it is captured in the closure.
    let mode = accordion.mode;

    let toggle = move |_| {
        // Signal<T> is a handle that implements Copy trait.
        // accordion.open_items is assigned to open_items to avoid capturing the accordion variable.
        let mut open_items = accordion.open_items;
        // write() obtains mutable access to the vector stored inside that signal.        
        let mut items = open_items.write();

        let existing_index = items
            .iter()
            .position(|value| value == &item_value);

        match mode {
            AccordionMode::Single => {
                if existing_index.is_some() {
                    items.clear();
                } else {
                    items.clear();
                    items.push(item_value.clone());
                }
            }

            AccordionMode::Multiple => {
                if let Some(index) = existing_index {
                    items.remove(index);
                } else {
                    items.push(item_value.clone());
                }
            }
        }
    };

    rsx! {
        button {
            class: "accordion-trigger",
            r#type: "button",
            "aria-expanded": "{is_open}",
            onclick: toggle,
            {children}
        }
    }
}

#[component]
pub fn AccordionContent(children: Element) -> Element {
    let accordion = use_context::<AccordionContext>();
    let item = use_context::<AccordionItemContext>();

    let is_open = accordion
        .open_items
        .read()
        .contains(&item.value);

    rsx! {
        if is_open {
            div { class: "accordion-content", {children} }
        }
    }
}