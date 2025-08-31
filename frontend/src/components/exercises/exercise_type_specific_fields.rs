use crate::{
  components::fretboard::FretboardModelAdapter, models::fretboard::FretboardModelBuilder,
};

use super::{constants::*, PositionPresetButtons};
use leptos::prelude::*;
use shared::music::{heptatonic_scales::HeptaScaleType, notes::Note, scales::ScaleType};

#[component]
pub fn ExerciseTypeSpecificFields(
  exercise_type: ReadSignal<String>,
  root_note: ReadSignal<Note>,
  on_root_note_change: Callback<Note>,
  scale_type: ReadSignal<ScaleType>,
  on_scale_type_change: Callback<ScaleType>,
  min_fret: ReadSignal<u8>,
  on_min_fret_change: Callback<u8>,
  max_fret: ReadSignal<u8>,
  on_max_fret_change: Callback<u8>,
) -> impl IntoView {
  // Handle preset selection
  let on_preset_select = Callback::new(move |(min, max): (u8, u8)| {
    on_min_fret_change.run(min);
    on_max_fret_change.run(max);
  });

  view! {
    // Conditional fields for Scale and Triad types
    {move || {
      let ex_type = exercise_type.get();
      if has_specific_settings(&ex_type) {
        view! {
          <div class="p-4 space-y-4 bg-gray-50 rounded-md">
            <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
              // Root note selector
              <div>
                <label class="block mb-1 text-sm font-medium text-gray-700">Root Note</label>
                <select
                  class="py-2 px-3 w-full rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  prop:value=move || format!("{:?}", root_note.get())
                  on:change=move |e| {
                    let value = event_target_value(&e);
                    if let Ok(note) = value.parse::<Note>() {
                      on_root_note_change.run(note);
                    }
                  }
                >
                  <option value="C">C</option>
                  <option value="Cs">"C#"</option>
                  <option value="D">D</option>
                  <option value="Ds">"D#"</option>
                  <option value="E">E</option>
                  <option value="F">F</option>
                  <option value="Fs">"F#"</option>
                  <option value="G">G</option>
                  <option value="Gs">"G#"</option>
                  <option value="A">A</option>
                  <option value="As">"A#"</option>
                  <option value="B">B</option>
                </select>
              </div>

              // Scale type selector (simplified for now)
              <div>
                <label class="block mb-1 text-sm font-medium text-gray-700">Scale Type</label>
                <select
                  class="py-2 px-3 w-full rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  on:change=move |e| {
                    let value = event_target_value(&e);
                    match value.as_str() {
                      "Major" => {
                        on_scale_type_change.run(ScaleType::Hepatonic(HeptaScaleType::Major))
                      }
                      "Minor" => {
                        on_scale_type_change.run(ScaleType::Hepatonic(HeptaScaleType::Minor))
                      }
                      _ => on_scale_type_change.run(ScaleType::Hepatonic(HeptaScaleType::Major)),
                    }
                  }
                >
                  <option
                    value="Major"
                    selected=move || {
                      matches!(scale_type.get(), ScaleType::Hepatonic(HeptaScaleType::Major))
                    }
                  >
                    Major
                  </option>
                  <option
                    value="Minor"
                    selected=move || {
                      matches!(scale_type.get(), ScaleType::Hepatonic(HeptaScaleType::Minor))
                    }
                  >
                    Natural Minor
                  </option>
                </select>
              </div>
            </div>

            // Fret range
            <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
              <div>
                <label class="block mb-1 text-sm font-medium text-gray-700">Min Fret</label>
                <input
                  type="number"
                  min="0"
                  max="24"
                  class="py-2 px-3 w-full rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  prop:value=move || min_fret.get().to_string()
                  on:input=move |e| {
                    if let Ok(val) = event_target_value(&e).parse::<u8>() {
                      on_min_fret_change.run(val.min(24));
                    }
                  }
                />
              </div>
              <div>
                <label class="block mb-1 text-sm font-medium text-gray-700">Max Fret</label>
                <input
                  type="number"
                  min="0"
                  max="24"
                  class="py-2 px-3 w-full rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                  prop:value=move || max_fret.get().to_string()
                  on:input=move |e| {
                    if let Ok(val) = event_target_value(&e).parse::<u8>() {
                      on_max_fret_change.run(val.min(24));
                    }
                  }
                />
              </div>
            </div>

            // Position presets
            <PositionPresetButtons on_preset_select />

            // Fretboard preview for scales
            <div>
              <label class="block mb-2 text-sm font-medium text-gray-700">Preview</label>
              //
              <div class="p-4 mx-auto max-w-2xl bg-gray-50 rounded-lg">
                <FretboardModelAdapter model=Signal::derive(move || {
                  FretboardModelBuilder::new()
                    .start_fret(Signal::derive(move || min_fret.get() as usize))
                    .end_fret(Signal::derive(move || max_fret.get() as usize))
                    .build()
                }) />
              </div>
            </div>
          </div>
        }
          .into_any()
      } else {
        view! { <div></div> }.into_any()
      }
    }}
  }
}
