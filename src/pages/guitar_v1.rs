use crate::{
  components::fretboard::Fretboard,
  music::{notes::Note, scales::ScaleType},
};
use leptos::prelude::*;

#[component]
pub fn GuitarV1() -> impl IntoView {
  let (root_note, _) = signal(Note::C);
  let (scale_type, _) = signal(ScaleType::Chromatic);

  view! {
    <ErrorBoundary fallback=|errors| {
      view! {
        <h1>"Uh oh! Something went wrong!"</h1>

        <p>"Errors: "</p>
        // Render a list of errors as strings - good for development purposes
        <ul>
          {move || {
            errors.get().into_iter().map(|(_, e)| view! { <li>{e.to_string()}</li> }).collect_view()
          }}

        </ul>
      }
    }>

      <div>
        <h1 class="py-12 text-6xl font-bold text-center text-primary-rev trans">
          "Gitarren Griffbrett"
        </h1>
        <Fretboard num_frets=24 num_strings=6 root_note scale_type />
      </div>
    </ErrorBoundary>
  }
}
