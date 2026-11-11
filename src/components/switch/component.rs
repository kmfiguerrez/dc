use dioxus::prelude::*;

// Temporary CSS loading.
const SWITCH_CSS: Asset = asset!("/assets/styling/switch.css");

// The checked prop is for controlled usage--A controlled component receives its state from its parent.
// The default_checked prop is for uncontrolled usage--The component manage its own state.
#[component]
pub fn Switch(
    label: String,

    #[props(default)]
    checked: Option<bool>,

    #[props(default)]
    default_checked: bool,

    #[props(default)]
    disabled: bool,

    #[props(default)]
    on_checked_change: EventHandler<bool>,
) -> Element {
    let mut internal_checked = use_signal(|| default_checked);

    let is_controlled = checked.is_some();

    // If controlled, the current_checked is the parent supplied value, otherwise internal_check().
    let current_checked = checked.unwrap_or_else(|| internal_checked());

    let toggle = move |_| {
        if disabled {
            return;
        }

        let next_value = !current_checked;

        // If controlled, this component will not update its own state.
        if !is_controlled {
            internal_checked.set(next_value);
        }

        // Runs the closure the parent supplied.
        on_checked_change.call(next_value);
    };

    rsx! {
        // Temporary CSS loading.
        document::Link { rel: "stylesheet", href: SWITCH_CSS }

        button {
            r#type: "button",
            role: "switch",
            "aria-checked": "{current_checked}",
            "aria-label": "{label}",
            disabled,
            class: if current_checked { "switch switch-on" } else { "switch" },
            onclick: toggle,

            span { class: "switch-thumb" }
        }
    }
}