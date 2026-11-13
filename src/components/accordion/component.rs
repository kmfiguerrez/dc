use dioxus::prelude::*;

// Temporary CSS loading.
const ACCORDION_CSS: Asset = asset!("/assets/styling/accordion.css");

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AccordionMode {
    #[default]
    Single,
    Multiple,
}

// #[derive(Clone, PartialEq)]
#[derive(Clone)]
struct AccordionContext {
    mode: AccordionMode,

    open_items: ReadSignal<Vec<String>>,

    internal_open: Signal<Vec<String>>,

    is_controlled: bool,

    on_open_change: EventHandler<Vec<String>>,
}

#[component]
pub fn Accordion(
    #[props(default)]
    mode: AccordionMode,

    // open is option to determine whether the component is controlled (Some) or not (None).
    // open is ReadSignal rather than signal because we want one-way data flow:
    // The parent owns the writable signal.
    // The Accordion receives read-only access.
    // The Accordion asks the parent to change the state through a callback.
    #[props(default)]
    open: Option<ReadSignal<Vec<String>>>,

    // initial state when the parent doesn't control it.
    #[props(default)]
    default_open: Vec<String>,

    // callback that reports changes.
    #[props(default)]
    on_open_change: EventHandler<Vec<String>>,

    #[props(default)]
    class: String,

    children: Element,
) -> Element {
    let internal_open = use_signal(|| default_open.clone());

    let is_controlled = open.is_some();

    // open_items observes the parent's signal (thru open prop) if controlled.
    // open_items observes internal_open if uncontrolled.
    let open_items = open.unwrap_or_else(|| internal_open.into());

    use_context_provider(|| AccordionContext {
        mode,
        open_items,
        internal_open,
        is_controlled,
        on_open_change,
    });

    rsx! {
        // Temporary CSS loading.
        document::Link { rel: "stylesheet", href: ACCORDION_CSS }

        div { class: "accordion {class}", {children} }
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

    // Accordion properties are reassigned because they are captured in the toggle closure.
    let open_items = accordion.open_items;
    let mut internal_open = accordion.internal_open;
    let is_controlled = accordion.is_controlled;
    let on_open_change = accordion.on_open_change;
    let mode = accordion.mode;

    let item_value = item.value.clone();

    let toggle = move |_| {
        // We clone open_items because we want to calculate the next state before deciding where to store it.
        // This is particularly important for controlled components: the Accordion must not directly modify the parent's state.
        let mut next_items = open_items.read().clone();

        let existing_index = next_items
            .iter()
            .position(|value| value == &item_value);

        match mode {
            AccordionMode::Single => {
                if existing_index.is_some() {
                    next_items.clear();
                } else {
                    next_items.clear();
                    next_items.push(item_value.clone());
                }
            }

            AccordionMode::Multiple => {
                if let Some(index) = existing_index {
                    next_items.remove(index);
                } else {
                    next_items.push(item_value.clone());
                }
            }
        }

        if !is_controlled {
            internal_open.set(next_items.clone());
        }

        // Notice that we call the callback in both modes - controlled and uncontrolled.
        // This is useful because an uncontrolled Accordion can still notify its parent about changes.
        on_open_change.call(next_items);        
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

    // The Content component doesn't need to know where the state originated.
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