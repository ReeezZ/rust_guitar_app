use crate::music::notes::Note;
use crate::music::scales::ScaleType;
use crate::music::{heptatonic_scales::HeptaScaleType, Scale};
use crate::{
  components::{
    fret_range_selector::FretRangeSelector,
    fretboard::{FretboardModelAdapter, FretboardVisualConfigBuilder},
    music_selectors::{NoteSelector, ScaleTypeSelector},
  },
  models::fretboard::{default_tuning, FretboardModelBuilder, FretboardModelExt},
};
use leptos::{prelude::*, wasm_bindgen::JsCast};

/// Page demonstrating the SVG fretboard with scale display functionality
#[component]
pub fn FretboardScalePage() -> impl IntoView {
  let start_fret = RwSignal::new(2_usize);
  let end_fret = RwSignal::new(7_usize);

  // Extra frets for visual context
  let extra_frets = RwSignal::new(2_usize);

  let (scale, set_scale) = signal(Scale::new(
    Note::C,
    ScaleType::Hepatonic(HeptaScaleType::Major),
  ));

  let root_note = Memo::new(move |_| scale.get().root_note().unwrap_or(Note::C));

  let model = RwSignal::new(
    FretboardModelBuilder::new()
      .start_fret(start_fret.into())
      .end_fret(end_fret.into())
      .tuning(default_tuning())
      .config(Signal::derive(move || {
        FretboardVisualConfigBuilder::new()
          .extra_frets(extra_frets.into())
          .build()
      }))
      .build(),
  );

  model.with_untracked(move |model| {
    model.update_from_scale(scale.get_untracked());
  });

  let update_scale = move |scale: Scale| {
    model.with_untracked(move |model| {
      model.update_from_scale(scale);
    });
    set_scale.set(scale);
  };

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
            {move || format!("{:?}", scale.get())}
          </p>
          <p>
            <strong>"Range:"</strong>
            " Frets "
            {move || format!("{}-{}", start_fret.get(), end_fret.get())}
          </p>
        </div>
      </div>

      // Quick scale presets
      <div class="p-4 bg-yellow-50 rounded-lg border-2 border-yellow-200">
        <h3 class="mb-3 text-lg font-semibold">"Quick Presets"</h3>
        <div class="flex flex-wrap gap-2">
          <button
            class="py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600"
            on:click=move |_| {
              start_fret.set(3);
              end_fret.set(7);
              update_scale(Scale::new(Note::G, ScaleType::Hepatonic(HeptaScaleType::Major)));
            }
          >
            "G Major (3-7)"
          </button>
          <button
            class="py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600"
            on:click=move |_| {
              start_fret.set(5);
              end_fret.set(8);
              update_scale(Scale::new(Note::A, ScaleType::Hepatonic(HeptaScaleType::Minor)));
            }
          >
            "A Minor (5-8)"
          </button>
          <button
            class="py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600"
            on:click=move |_| {
              start_fret.set(0);
              end_fret.set(5);
              update_scale(Scale::new(Note::E, ScaleType::Hepatonic(HeptaScaleType::Minor)));
            }
          >
            "E Minor (0-5)"
          </button>
          <button
            class="py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600"
            on:click=move |_| {
              start_fret.set(7);
              end_fret.set(10);
              update_scale(Scale::new(Note::C, ScaleType::Hepatonic(HeptaScaleType::Major)));
            }
          >
            "C Major (7-10)"
          </button>
          <button
            class="py-2 px-4 text-white bg-green-500 rounded hover:bg-green-600"
            on:click=move |_| {
              start_fret.set(1);
              end_fret.set(4);
              update_scale(Scale::new(Note::D, ScaleType::Hepatonic(HeptaScaleType::Minor)));
            }
          >
            "A Minor (1-4, no opens)"
          </button>
          <button
            class="py-2 px-4 text-white bg-purple-500 rounded hover:bg-purple-600"
            on:click=move |_| {
              start_fret.set(0);
              end_fret.set(10);
              update_scale(Scale::new(Note::E, ScaleType::Hepatonic(HeptaScaleType::Major)));
            }
          >
            "Open strings only"
          </button>
        </div>
      </div>

      // Main fretboard display
      //
      <FretboardModelAdapter model />
      // />
      // Show 2 extra frets beyond the end fret
      <div class="p-4 bg-gray-50 rounded-lg border-2 border-gray-200"></div>

      // Scale configuration controls
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
        // Root note selector
        <NoteSelector
          value=root_note.into()
          on_note_changed=Callback::new(move |note| {
            update_scale(Scale::new(note, scale.get_untracked().scale_type()));
          })
          label="Root Note"
        />

        // Scale type selector
        <ScaleTypeSelector
          value=Signal::derive(move || scale.get().scale_type())
          on_scale_changed=Callback::new(move |scale_type| {
            update_scale(Scale::new(root_note.get_untracked(), scale_type));
          })
          label="Scale Type"
        />

        // Fret range control with dual sliders
        <FretRangeSelector
          on_start_fret_change=Callback::new(move |new_start| {
            start_fret.set(new_start);
            update_scale(scale.get_untracked());
          })
          on_end_fret_change=Callback::new(move |new_end| {
            end_fret.set(new_end);
            update_scale(scale.get_untracked());
          })
          start_fret
          end_fret
          label="Playable Range"
        />

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
