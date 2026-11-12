use dioxus::prelude::*;

// Temporary CSS loading.
const ACCORDION_CSS: Asset = asset!("/assets/styling/accordion.css");

#[derive(Clone, PartialEq)]
struct AccordionContext {
    open_item: Signal<Option<String>>,
}

#[component]
pub fn Accordion(children: Element) -> Element {
    // The None::<String> tells Rust that the type inside Option is String.
    let open_item = use_signal(|| None::<String>);

    use_context_provider(|| AccordionContext {
        open_item,
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

    // The accordion.open_item is inside a parantheses because rust complains
    // when accordion.open_item() is used, saying open_item is not a method.
    // Workaround: put open_item inside () or dereference *open_item.read().
    let is_open = (accordion.open_item)() == Some(item.value.clone());

    let toggle = move |_| {
        // Signal<T> is a handle that implements the Copy trait.
        // When assigned to another variable, the handle is copied not the underlying data.
        // A handle is a value used to identify or access a resource (actual data).
        let mut open_item = accordion.open_item;

        if is_open {
            open_item.set(None);
        } else {
            open_item.set(Some(item.value.clone()));
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

    let is_open = (accordion.open_item)() == Some(item.value.clone());

    rsx! {
        if is_open {
            div { class: "accordion-content", {children} }
        }
    }
}