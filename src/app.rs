use leptos::{
  prelude::{ClassAttribute, ElementChild},
  *,
};

use crate::fretboard::Fretboard;

#[component]
pub fn App() -> impl IntoView {
  view! {
    <div class="flex flex-col items-center px-20 min-h-screen bg-gray-900 max-h-fit">
      <h1 class="py-6 text-6xl font-bold text-center text-cyan-700 border border-black">
        "Gitarren Griffbrett"
      </h1>
      <Fretboard num_frets=21 num_strings=6 />
    </div>
  }
}
