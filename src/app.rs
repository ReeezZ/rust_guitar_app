use leptos::{
  prelude::{ClassAttribute, ElementChild},
  *,
};

use crate::fretboard::Fretboard;

#[component]
pub fn App() -> impl IntoView {
  view! {
    <div class="flex-col justify-center items-center h-screen bg-gray-900">
      <h1 class="py-6 text-6xl font-bold text-center text-amber-500">"Gitarren Griffbrett"</h1>
      <Fretboard num_frets=19 num_strings=6 />
    </div>
  }
}
