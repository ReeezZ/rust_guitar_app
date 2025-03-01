use leptos::prelude::*;

use crate::music::scales::{Scale, ScaleType};
use crate::{components::fretboard::Fretboard, music::notes::Note};

#[component]
fn ScaleSelection() -> impl IntoView {
  view! {
    <div class="flex justify-center text-center align-middle">
      <h1 class="text-4xl font-bold text-center">"Scale Selection"</h1>

    </div>
  }
}

#[component]
fn RootNoteSelection(set_root_note: WriteSignal<Note>) -> impl IntoView {
  view! {
    <div>
      <label>"Root Note"</label>
      <select
        class="py-2 px-3 rounded border border-gray-300"
        on:change=move |ev| {
          if let Some(note) = Note::from_str(&event_target_value(&ev)) {
            set_root_note.set(note);
          }
        }
      >
        // on:change:target=
        {Note::all_notes()
          .iter()
          .map(|note| {
            view! { <option>{note.to_string()}</option> }
          })
          .collect_view()}
      </select>
    </div>
  }
}

fn is_note_visible(root_note: Note) -> impl Fn(Note) -> bool {
  move |note: Note| Scale::new(root_note, ScaleType::Major).contains_note(note)
}

fn is_note_visible(note: Note) -> bool {
  Scale::new(Note::D, ScaleType::Minor).contains_note(note)
}

fn note_to_string(note: Note) -> String {
  note.to_string()
}

#[component]
pub fn GuitarV2() -> impl IntoView {
  let (is_note_visible_signal, _) = signal::<fn(Note) -> bool>(is_note_visible);
  let (root_note, set_root_note) = signal(Note::C);

  let (is_note_visible_signal, _) = signal::<fn(Note) -> bool>(|note| is_note_visible);
  let (note_to_string_signal, _) = signal::<fn(Note) -> String>(note_to_string);

  view! {
    <div class="flex-row space-y-4">
      <Fretboard num_frets=24 num_strings=6 is_note_visible_signal note_to_string_signal />
      <div class="flex flex-row justify-center space-y-4 space-x-4">
        <RootNoteSelection set_root_note />
        <ScaleSelection />
      </div>
    </div>
  }
}
