//! The next version of the fretboard viewer.
//! WIP

use std::str::FromStr;
use std::sync::{Arc, Mutex};

use leptos::logging::log;
use leptos::prelude::*;

use crate::components::fretboard::Fretboard;
use crate::components::fretboard_model::FretboardModel;
use crate::music::heptatonic_scales::HeptaScaleType::Major;
use crate::music::notes::Note;
use crate::music::scales::{Scale, ScaleTrait, ScaleType};

#[component]
fn ScaleSelection(set_scale_type: WriteSignal<ScaleType>) -> impl IntoView {
  view! {
    <div class="flex flex-row justify-center items-center m-4 text-center align-middle">
      <label>"Scale"</label>
      <select
        class="py-2 px-3 rounded border border-gray-300"
        on:change=move |ev| {
          if let Some(scale_type) = ScaleType::from_str(&event_target_value(&ev)) {
            set_scale_type.set(scale_type);
          }
        }
      >
        {ScaleType::all_scale_types()
          .iter()
          .map(|scale_type| {
            view! { <option value=scale_type.to_string()>{scale_type.to_string()}</option> }
          })
          .collect_view()}
      </select>
    </div>
  }
}

#[component]
fn RootNoteSelection(
  root_note: ReadSignal<Note>,
  set_root_note: WriteSignal<Note>,
) -> impl IntoView {
  view! {
    <div class="flex flex-row items-center m-4 text-center align-middle">
      <label>"Root Note"</label>
      <select
        class="py-2 px-3 rounded border border-gray-300"
        on:change=move |ev| {
          let event_value = event_target_value(&ev);
          if let Ok(note) = Note::from_str(&event_value) {
            set_root_note.set(note);
          } else {
            log!("Failed to parse note from this value: {}", &event_value);
          }
        }
      >
        {move || {
          Note::all_notes()
            .iter()
            .map(|note| {
              view! {
                <option value=note.to_string() selected=*note == root_note.get()>
                  {note.to_string()}
                </option>
              }
            })
            .collect_view()
        }}
      </select>
    </div>
  }
}

#[component]
pub fn FretboardNext() -> impl IntoView {
  let (root_note, set_root_note) = signal(Note::C);
  let (scale_type, set_scale_type) = signal(ScaleType::Hepatonic(Major));
  let (num_frets, set_num_frets) = signal(12);

  let fretboard_model = Arc::new(Mutex::new(FretboardModel::new(
    6,
    num_frets.get(),
    FretboardModel::standard_tuning(),
  )));

  let fb_for_num_frets = fretboard_model.clone();
  let fb_scale_effect = fretboard_model.clone();

  Effect::new(move |_| {
    fb_for_num_frets
      .lock()
      .expect("mutex lock failed")
      .update_num_frets(num_frets.get());
  });

  // Create an effect to update the fretboard whenever signals change
  Effect::new(move |_| {
    let scale = Scale::new(root_note.get(), scale_type.get());
    log!("Updating fretboard with scale: {:?}", &scale);

    fb_scale_effect
      .lock()
      .expect("mutex lock failed")
      .update_from_scale(&scale);
  });

  view! {
    <div class="flex-row y-4">
      <Fretboard
        fretboard=fretboard_model
        on_fret_clicked=Callback::new(|evt| log!("Fret clicked: {:?}", evt))
      />
      <div class="flex flex-row justify-center items-center text-center">
        <RootNoteSelection set_root_note root_note />
        <ScaleSelection set_scale_type />
      </div>
    </div>
  }
}
