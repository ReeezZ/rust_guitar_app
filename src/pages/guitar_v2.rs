use leptos::prelude::*;

use crate::music::scales::{Scale, ScaleType};
use crate::{components::fretboard::Fretboard, music::notes::Note};

fn is_note_visible(note: Note) -> bool {
  Scale::new(Note::C, ScaleType::Major).contains_note(note)
}

fn note_to_string(note: Note) -> String {
  note.to_string()
}

#[component]
pub fn GuitarV2() -> impl IntoView {
  let (is_note_visible_signal, _) = signal::<fn(Note) -> bool>(is_note_visible);
  let (note_to_string_signal, _) = signal::<fn(Note) -> String>(note_to_string);
  view! {
    <div class="">
      <Fretboard num_frets=24 num_strings=6 is_note_visible_signal note_to_string_signal />
    </div>
  }
}
