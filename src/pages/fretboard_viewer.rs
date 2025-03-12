use std::str::FromStr;

use leptos::logging::debug_warn;
use leptos::prelude::*;

use crate::components::fretboard_scale_display::FretboardScaleDisplay;
use crate::music::heptatonic_scales::HeptaScaleType::Major;
use crate::music::notes::Note;
use crate::music::scales::ScaleType;

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
            debug_warn!("Failed to parse note from this value: {}", &event_value);
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

const MAX_NUM_FRETS: u8 = 24;

#[component]
fn NumFretsSelection(num_frets: ReadSignal<u8>, set_num_frets: WriteSignal<u8>) -> impl IntoView {
  let min_frets = 5;

  view! {
    <div class="flex flex-row items-center m-4 text-center align-middle">
      <label class="mr-2">"Number of Frets"</label>
      <div class="flex items-center">
        <button
          class="py-1 px-2 rounded border border-gray-300 hover:bg-gray-100"
          on:click=move |_| {
            let current = num_frets.get();
            if current > min_frets {
              set_num_frets.set(current - 1);
            }
          }
        >
          "-"
        </button>
        <span class="px-3 text-center min-w-[2rem]">{move || num_frets.get()}</span>
        <button
          class="py-1 px-2 rounded border border-gray-300 hover:bg-gray-100"
          on:click=move |_| {
            let current = num_frets.get();
            if current < MAX_NUM_FRETS {
              set_num_frets.set(current + 1);
            }
          }
        >
          "+"
        </button>
      </div>
    </div>
  }
}

#[component]
pub fn FretboardViewer() -> impl IntoView {
  let (root_note, set_root_note) = signal(Note::C);
  let (scale_type, set_scale_type) = signal(ScaleType::Hepatonic(Major));

  // Using max num frets for now. https://gitlab.com/ReeeZ/leptos_stuff/-/issues/10
  let (num_frets, set_num_frets) = signal(MAX_NUM_FRETS);

  view! {
    <div class="flex-row y-4">
      <FretboardScaleDisplay num_frets=num_frets root_note=root_note scale_type=scale_type />
      <div class="flex flex-row flex-wrap justify-center items-center text-center">
        <RootNoteSelection set_root_note=set_root_note root_note=root_note />
        <ScaleSelection set_scale_type=set_scale_type />
        <NumFretsSelection num_frets=num_frets set_num_frets=set_num_frets />
      </div>
    </div>
  }
}
