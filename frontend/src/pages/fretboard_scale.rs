// Currently refactoring fretboard base
// TODO reenable warnings
#![allow(warnings)]
use crate::{
  components::{
    fret_range_selector::FretRangeSelector,
    music_selectors::{NoteSelector, ScaleTypeSelector},
  },
  fretboard::{
    components::{
      base::{Fretboard, FretboardViewModel},
      visual_config::{self, FretboardVisualConfig},
    },
    fretboard_model::{default_tuning, FretClickEvent, FretboardModel},
  },
};
use leptos::{logging::log, prelude::*, wasm_bindgen::JsCast};
use shared::music::scales::ScaleType;
use shared::{music::heptatonic_scales::HeptaScaleType, Scale};
use shared::{music::notes::Note, ScaleTrait};

/// Page demonstrating the SVG fretboard with scale display functionality
#[component]
pub fn FretboardScalePage() -> impl IntoView {
  let start_fret = RwSignal::new(2_usize);
  let end_fret = RwSignal::new(7_usize);

  // Extra frets for visual context
  let extra_frets = RwSignal::new(2_usize);

  // Scale configuration
  let root_note = RwSignal::new(Note::C);
  let scale_type = RwSignal::new(ScaleType::Hepatonic(HeptaScaleType::Major));

  // Track clicked note for testing
  let (clicked_note_event, set_clicked_note_event) = signal::<Option<FretClickEvent>>(None);

  let on_note_clicked = Callback::new(move |event: FretClickEvent| {
    log!(
      "🎵 Scale Display - Note: {}, String: {} (1-indexed: {}), Fret: {}",
      event.note,
      event.coord.string_idx,
      event.coord.string_idx + 1,
      event.coord.fret_idx
    );
    set_clicked_note_event.set(Some(event));
  });

  let model = RwSignal::new(FretboardModel::default());

  

  Effect::new(move || {
    model.update(move |model| {
      model.update_from_scale(Scale::new(root_note.get(), scale_type.get()));
    });
  });

  view! {
    <div class="p-6 space-y-6">
      <h1 class="text-3xl font-bold">"SVG Fretboard with Scale Display"</h1>

      // Current scale info display
      <div class="p-4 bg-blue-50 rounded-lg border-2 border-blue-200">
        <h3 class="mb-2 text-lg font-semibold">"Current Scale"</h3>
        <div class="text-sm text-gray-700">
          <p>
            <strong>"Root:"</strong>
            {move || root_note.get().to_string()}
          </p>
          <p>
            <strong>"Scale:"</strong>
            {move || format!("{:?}", scale_type.get())}
          </p>
          <p>
            <strong>"Range:"</strong>
            " Frets "
            {move || format!("{}-{}", start_fret.get(), end_fret.get())}
          </p>
        </div>
      </div>

      // Click feedback
      <div class="p-4 bg-green-50 rounded-lg border-2 border-green-200">
        <strong>"Clicked Note: "</strong>
        {move || match clicked_note_event.get() {
          Some(event) => {
            format!(
              "{} - String {} - Fret {}",
              event.note,
              event.coord.string_idx + 1,
              event.coord.fret_idx,
            )
          }
          None => "Click on the fretboard to test interaction".to_string(),
        }}
      </div>

      // Quick scale presets
      <div class="p-4 bg-yellow-50 rounded-lg border-2 border-yellow-200">
        <h3 class="mb-3 text-lg font-semibold">"Quick Presets"</h3>
        <div class="flex flex-wrap gap-2">
          <button
            class="py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600"
            on:click=move |_| {
              root_note.set(Note::G);
              scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Major));
              start_fret.set(3);
              end_fret.set(7);
            }
          >
            "G Major (3-7)"
          </button>
          <button
            class="py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600"
            on:click=move |_| {
              root_note.set(Note::A);
              scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Minor));
              start_fret.set(5);
              end_fret.set(8);
            }
          >
            "A Minor (5-8)"
          </button>
          <button
            class="py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600"
            on:click=move |_| {
              root_note.set(Note::E);
              scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Minor));
              start_fret.set(0);
              end_fret.set(5);
            }
          >
            "E Minor (0-5)"
          </button>
          <button
            class="py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600"
            on:click=move |_| {
              root_note.set(Note::C);
              scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Major));
              start_fret.set(7);
              end_fret.set(10);
            }
          >
            "C Major (7-10)"
          </button>
          <button
            class="py-2 px-4 text-white bg-green-500 rounded hover:bg-green-600"
            on:click=move |_| {
              root_note.set(Note::A);
              scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Minor));
              start_fret.set(1);
              end_fret.set(4);
            }
          >
            "A Minor (1-4, no opens)"
          </button>
          <button
            class="py-2 px-4 text-white bg-purple-500 rounded hover:bg-purple-600"
            on:click=move |_| {
              root_note.set(Note::E);
              scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Major));
              start_fret.set(0);
              end_fret.set(0);
            }
          >
            "Open strings only"
          </button>
        </div>
      </div>

      // Main fretboard display
      //
      <FretboardViewModel model />
      // />
      // Show 2 extra frets beyond the end fret
      <div class="p-4 bg-gray-50 rounded-lg border-2 border-gray-200"></div>

      // Scale configuration controls
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
        // Root note selector
        <NoteSelector value=root_note label="Root Note" />

        // Scale type selector
        <ScaleTypeSelector value=scale_type label="Scale Type" />

        // Fret range control with dual sliders
        <FretRangeSelector start_fret end_fret label="Playable Range" />

        // Extra frets control
        <div class="space-y-2">
          <label class="block text-sm font-medium text-gray-700">"Extra Frets"</label>
          <div class="text-sm text-gray-600">
            "Context: " {move || extra_frets.get()} " frets" <br />
            <span class="text-xs text-gray-500">"(Visual context beyond playable range)"</span>
          </div>
          <input
            type="range"
            min="0"
            max="5"
            prop:value=move || extra_frets.get()
            on:input=move |ev| {
              let target = ev.target().unwrap();
              let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
              if let Ok(val) = input.value().parse::<usize>() {
                extra_frets.set(val);
              }
            }
            class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
          />
        </div>
      </div>

      // Legend
      <div class="p-4 bg-gray-50 rounded-lg border-2 border-gray-200">
        <h3 class="mb-3 text-lg font-semibold">"Legend"</h3>
        <div class="flex flex-wrap gap-6">
          <div class="flex gap-2 items-center">
            <div class="w-6 h-6 bg-red-500 rounded-full border-2 border-red-700"></div>
            <span>"Root Note"</span>
          </div>
          <div class="flex gap-2 items-center">
            <div class="w-5 h-5 bg-blue-500 rounded-full border-2 border-blue-700"></div>
            <span>"Scale Notes"</span>
          </div>
          <div class="flex gap-2 items-center">
            <div class="w-4 h-4 bg-gray-400 rounded-full opacity-50"></div>
            <span>"Fret Markers"</span>
          </div>
        </div>
      </div>
    </div>
  }
}
