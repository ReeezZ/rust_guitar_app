use leptos::{prelude::ElementChild, *};

use crate::fretboard::Fretboard;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h1>"Gitarren Griffbrett"</h1>
        <Fretboard num_frets=5 num_strings=3 />
    }
}
