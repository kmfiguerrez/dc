use dioxus::prelude::*;

const BUTTON_CSS: Asset = asset!("/assets/styling/button.css");

// Copy means that small values like this enum can be copied automatically when assigned or passed by value.
// Since our enum contains no heap-allocated data, implementing Copy is appropriate.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[component]
pub fn Button(
    #[props(default)]
    variant: ButtonVariant,

    #[props(default)]
    size: ButtonSize,

    #[props(default)]
    disabled: bool,

    #[props(default)]
    class: String,

    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,

    children: Element,
) -> Element {
    let variant_class = match variant {
        ButtonVariant::Primary => "button-primary",
        ButtonVariant::Secondary => "button-secondary",
        ButtonVariant::Destructive => "button-destructive",
    };

    let size_class = match size {
        ButtonSize::Small => "button-sm",
        ButtonSize::Medium => "button-md",
        ButtonSize::Large => "button-lg",
    };

    let class = format!(
        "button {variant_class} {size_class} {class}"
    );

    rsx! {
        // Temporary CSS loading.
        document::Link { rel: "stylesheet", href: BUTTON_CSS }

        button {
            class,
            disabled,

            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler.call(event);
                }
            },

            {children}
        }
    }
}