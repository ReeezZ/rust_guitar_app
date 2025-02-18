use leptos::{
  prelude::ElementChild,
  *,
};

use crate::fretboard::Fretboard;

#[component]
pub fn App() -> impl IntoView {
  view! {
      <div>
        <h1 class="py-6 text-6xl font-bold text-center text-cyan-700">
          "Gitarren Griffbrett"
        </h1>
        <Fretboard num_frets=24 num_strings=6 />
      </div>
  }
}
