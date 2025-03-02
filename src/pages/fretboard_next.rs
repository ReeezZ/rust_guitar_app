//! The next version of the fretboard viewer.
//! WIP

use leptos::prelude::*;

use crate::components::fretboard_rework::{FretboardModel, FretboardRework};
use crate::music::heptatonic_scales::HeptaScaleType::Major;
use crate::music::heptatonic_scales::{HeptaScaleImpl, HeptaScaleType};
use crate::music::notes::Note;
use crate::music::scales::{Scale, ScaleType};

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
          if let Some(note) = Note::from_str(&event_target_value(&ev)) {
            set_root_note.set(note);
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

  // Create a signal to hold the fretboard model
  let fretboard_model = RwSignal::new(FretboardModel::new(6, 12));

  // String tunings
  let string_tunings = vec![Note::E, Note::H, Note::G, Note::D, Note::A, Note::E];

  // Create an effect to update the fretboard whenever signals change
  Effect::new(move |_| {
    let current_root = root_note.get();
    let current_scale = scale_type.get();
    let current_scale = match current_scale {
      ScaleType::Hepatonic(scale) => scale,
      _ => HeptaScaleType::Major,
    };
    let scale = Scale::Heptatonic(HeptaScaleImpl::new(current_root, current_scale));

    // Update the model by creating a new one
    // This assumes FretboardModel implements Clone
    fretboard_model.update(|model| {
      model.update_from_scale(&string_tunings, &scale);
    });
  });

  view! {
    <div class="flex-row y-4">
      <FretboardRework fretboard=fretboard_model />
      <div class="flex flex-row justify-center items-center text-center">
        <RootNoteSelection set_root_note root_note />
        <ScaleSelection set_scale_type />
      </div>
    </div>
  }
}
