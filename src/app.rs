use leptos::{
  prelude::{ClassAttribute, ElementChild},
  *,
};

use crate::fretboard::Fretboard;

#[component]
pub fn App() -> impl IntoView {
  view! {
    <div class="bg-gray-900">
      <h1>"Gitarren Griffbrett"</h1>
      <Fretboard num_frets=15 num_strings=6 />
    </div>
  }
}
