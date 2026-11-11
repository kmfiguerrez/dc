//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.

mod hero;
pub use hero::Hero;

pub mod button;
pub mod stack;
pub mod card;


// Display components
mod button_page;
pub use button_page::ButtonPage;

mod card_page;
pub use card_page::CardPage;
