use leptos::prelude::*;

use crate::models::fretboard_model::FretboardModel;
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
fn RootNoteSelection() -> impl IntoView {
  // Get the write signal from context
  let set_root_note = use_context::<WriteSignal<Note>>().expect("set_root_note must be provided");

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

fn note_to_string(note: Note) -> String {
  note.to_string()
}

#[component]
pub fn GuitarV2() -> impl IntoView {
  // Create the model
  let model = FretboardModel::new();

  // Get the write signals from context
  let set_root_note = use_context::<WriteSignal<Note>>().expect("set_root_note must be provided");
  let set_scale_type =
    use_context::<WriteSignal<ScaleType>>().expect("set_scale_type must be provided");

  view! {
    <div class="flex flex-col space-y-4">
      <Fretboard num_frets=24 num_strings=6 />
      <div class="flex flex-row justify-center space-x-4">
        <RootNoteSelection />
        <ScaleSelection />
      </div>
    </div>
  }
}

// Similar for ScaleTypeSelection
