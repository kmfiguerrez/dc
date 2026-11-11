use dioxus::prelude::*;

// Temporary CSS loading.
const STACK_CSS: Asset = asset!("/assets/styling/stack.css");

#[derive(Clone, Copy, PartialEq, Default)]
pub enum StackDirection {
    Horizontal,

    #[default]
    Vertical,
}

#[component]
pub fn Stack(
    #[props(default)]
    direction: StackDirection,

    #[props(default = 8)]
    gap: u32,

    #[props(default)]
    class: String,

    children: Element,
) -> Element {
    let flex_direction = match direction {
        StackDirection::Horizontal => "row",
        StackDirection::Vertical => "column",
    };

    let class = format!("stack {class}");

    rsx! {
        // Temporary CSS loading.
        document::Link { rel: "stylesheet", href: STACK_CSS }

        div {
            class,
            style: "display: flex; flex-direction: {flex_direction}; gap: {gap}px;",
            {children}
        }
    }
}

#[component]
pub fn VStack(
    #[props(default = 8)]
    gap: u32,

    #[props(default)]
    class: String,

    children: Element,
) -> Element {
    rsx! {
        Stack { direction: StackDirection::Vertical, gap, class, {children} }
    }
}

#[component]
pub fn HStack(
    #[props(default = 8)]
    gap: u32,

    #[props(default)]
    class: String,

    children: Element,
) -> Element {
    rsx! {
        Stack { direction: StackDirection::Horizontal, gap, class, {children} }
    }
}