use crate::{
  components::{
    fretboard::FretboardModelAdapter,
    music_selectors::{NoteSelector, ScaleTypeSelector},
    ui::FretRangeSelector,
  },
  models::fretboard::FretboardModelBuilder,
};

use super::PositionPresetButtons;
use crate::music::{scales::ScaleType, Note};
use leptos::prelude::*;

#[component]
pub fn ScaleExerciseForm(
  root_note: ReadSignal<Note>,
  on_root_note_change: Callback<Note>,
  scale_type: ReadSignal<ScaleType>,
  on_scale_type_change: Callback<ScaleType>,
  min_fret: RwSignal<u8>,
  max_fret: RwSignal<u8>,
) -> impl IntoView {
  // Handle preset selection
  let on_preset_select = Callback::new(move |(min, max): (u8, u8)| {
    min_fret.set(min);
    max_fret.set(max);
  });

  view! {
    // Conditional fields for Scale and Triad types
    {move || {

      view! {
        <div class="p-4 space-y-4 bg-gray-200 rounded-md dark:bg-gray-800">
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
            // Root note selector
            <NoteSelector
              value=root_note.into()
              on_note_changed=on_root_note_change
              label="Root Note"
            />

            // Scale type selector (simplified for now)
            <ScaleTypeSelector
              value=scale_type.into()
              on_scale_changed=on_scale_type_change
              label="Scale Type"
            />

          </div>

          // Fret range
          <FretRangeSelector start_fret=min_fret end_fret=max_fret label="Playable Range" />

          // Position presets
          <PositionPresetButtons
            on_preset_select
            current_range=Signal::derive(move || (min_fret.get(), max_fret.get()))
          />

          // Fretboard preview for scales
          <div>
            <label class="block mb-2 text-sm font-medium">Preview</label>
            //
            <div class="p-4 mx-auto max-w-2xl bg-gray-50 rounded-lg">
              <FretboardModelAdapter model=Signal::derive(move || {
                FretboardModelBuilder::new()
                  .start_fret(Signal::derive(move || min_fret.get()))
                  .end_fret(Signal::derive(move || max_fret.get()))
                  .build()
              }) />
            </div>
          </div>
        </div>
      }
        .into_any()
    }}
  }
}
