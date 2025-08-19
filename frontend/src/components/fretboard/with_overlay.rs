use leptos::prelude::*;

use crate::components::fretboard::with_notes::FretboardWithNotes;

#[component]
pub fn FretboardWithOverlay() -> impl IntoView {
  view! {
    <div class="fretboard-overlay">
      <FretboardWithNotes start_fret=5.into() end_fret=10.into() />
    </div>
  }
}
