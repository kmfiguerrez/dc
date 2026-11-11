use dioxus::prelude::*;

// Temporary CSS loading.
const CARD_CSS: Asset = asset!("/assets/styling/card.css");


#[component]
pub fn Card(
    #[props(default)]
    title: Option<String>,

    #[props(default)]
    class: String,

    children: Element,
) -> Element {
    rsx! {
        // Temporary CSS loading.
        document::Link { rel: "stylesheet", href: CARD_CSS }

        div { class: "card {class}",

            if let Some(title) = title {
                h2 { class: "card-title", "{title}" }
            }

            div { class: "card-content", {children} }
        }
    }
}