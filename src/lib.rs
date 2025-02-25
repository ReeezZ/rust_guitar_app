use leptos::{prelude::ElementChild, *};

mod components;
mod music;

use crate::components::fretboard_scale_selection::FretboardScaleSelection;

#[component]
pub fn App() -> impl IntoView {
  view! {
    <div>
      <h1 class="py-6 text-6xl font-bold text-center text-cyan-700">"Gitarren Griffbrett"</h1>
      <FretboardScaleSelection />
    </div>
  }
}
