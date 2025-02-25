use leptos::prelude::*;

use crate::{components::fretboard::Fretboard, music::notes::Note};

fn is_note_visible(note: Note) -> bool {
  true
}

fn note_to_string(note: Note) -> String {
  note.to_string()
}

#[component]
pub fn FretboardScaleSelection() -> impl IntoView {
  let (is_note_visible_signal, set_is_note_visible_signal) = signal(is_note_visible);
  let (note_to_string_signal, set_note_to_string_signal) = signal(note_to_string);
  view! {
    <div class="">
      <Fretboard
        num_frets=24
        num_strings=6
        is_note_visible_signal=is_note_visible_signal
        note_to_string_signal
      />
    </div>
  }
}
