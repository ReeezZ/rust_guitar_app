use leptos::prelude::*;

use crate::music::scales::{Scale, ScaleType};
use crate::{components::fretboard::Fretboard, music::notes::Note};

#[component]
fn ScaleSelection(set_scale_type: WriteSignal<ScaleType>) -> impl IntoView {
  view! {
    <div class="flex justify-center text-center align-middle">
      <h1 class="text-4xl font-bold text-center">"Scale Selection"</h1>

    </div>
  }
}

#[component]
fn RootNoteSelection(
  root_note: ReadSignal<Note>,
  set_root_note: WriteSignal<Note>,
) -> impl IntoView {
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
            view! {
              <option value=note.to_string() selected=*note == root_note.get()>
                {note.to_string()}
              </option>
            }
          })
          .collect_view()}
      </select>
    </div>
  }
}

#[component]
pub fn GuitarV2() -> impl IntoView {
  let (root_note, set_root_note) = signal(Note::C);
  let (scale_type, set_scale_type) = signal(ScaleType::Major);

  view! {
    <div class="flex-row space-y-4">
      <Fretboard num_frets=24 num_strings=6 root_note scale_type />
      <div class="flex flex-row justify-center space-y-4 space-x-4">
        <RootNoteSelection set_root_note root_note />
        <ScaleSelection set_scale_type />
      </div>
    </div>
  }
}
