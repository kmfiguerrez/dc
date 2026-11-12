use dioxus::prelude::*;

// Temporary CSS loading.
const ACCORDION_CSS: Asset = asset!("/assets/styling/accordion.css");

#[derive(Clone, PartialEq)]
struct AccordionContext {
    open_items: Signal<Vec<String>>,
}

#[component]
pub fn Accordion(children: Element) -> Element {
     let open_items = use_signal(|| Vec::<String>::new());

    use_context_provider(|| AccordionContext {
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

    // We cloned item.value because it is captured inside the toggle closure.
    let item_value = item.value.clone();

    let toggle = move |_| {
        // Signal<T> is a handle that implements Copy trait.
        let mut open_items = accordion.open_items;
        // write() obtains mutable access to the vector stored inside that signal.
        let mut items = open_items.write();

        if let Some(index) = items.iter().position(|value| value == &item_value) {
            items.remove(index);
        } else {
            items.push(item_value.clone());
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