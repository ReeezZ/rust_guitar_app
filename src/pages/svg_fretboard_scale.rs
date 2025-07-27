use crate::components::{
  fretboard::FretClickEvent, svg_fretboard_scale_display::SvgFretboardScaleDisplay,
};
use crate::music::heptatonic_scales::HeptaScaleType;
use crate::music::notes::Note;
use crate::music::scales::ScaleType;
use leptos::{ev, logging::log, prelude::*};

/// Extracts the value from an input or select event.
fn event_target_value(ev: &ev::Event) -> String {
  use leptos::wasm_bindgen::JsCast;
  let target = ev.target().unwrap();
  
  // Try as HtmlSelectElement first (for <select> elements)
  if let Ok(select) = target.clone().dyn_into::<web_sys::HtmlSelectElement>() {
    return select.value();
  }
  
  // Fall back to HtmlInputElement (for <input> elements)
  if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
    return input.value();
  }
  
  // If neither works, return empty string instead of panicking
  String::new()
}

/// Page demonstrating the SVG fretboard with scale display functionality
#[component]
pub fn SvgFretboardScalePage() -> impl IntoView {
  // Fret range controls
  let start_fret = RwSignal::new(0_usize);
  let end_fret = RwSignal::new(7_usize);

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

  view! {
    <div class="p-6 space-y-6">
      <h1 class="text-3xl font-bold">"SVG Fretboard with Scale Display"</h1>

      // Scale configuration controls
      <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
        // Root note selector
        <div class="space-y-2">
          <label class="block text-sm font-medium">"Root Note"</label>
          <select
            class="w-full p-2 border rounded-md"
            on:change=move |ev| {
              let value = event_target_value(&ev);
              if let Ok(note) = value.parse::<Note>() {
                root_note.set(note);
              }
            }
          >
            <option value="C" selected=move || root_note.get() == Note::C>"C"</option>
            <option value="C♯/D♭" selected=move || root_note.get() == Note::CisOrDes>"C#/Db"</option>
            <option value="D" selected=move || root_note.get() == Note::D>"D"</option>
            <option value="D♯/E♭" selected=move || root_note.get() == Note::DisOrEs>"D#/Eb"</option>
            <option value="E" selected=move || root_note.get() == Note::E>"E"</option>
            <option value="F" selected=move || root_note.get() == Note::F>"F"</option>
            <option value="F♯/G♭" selected=move || root_note.get() == Note::FisOrGes>"F#/Gb"</option>
            <option value="G" selected=move || root_note.get() == Note::G>"G"</option>
            <option value="G♯/A♭" selected=move || root_note.get() == Note::GisOrAs>"G#/Ab"</option>
            <option value="A" selected=move || root_note.get() == Note::A>"A"</option>
            <option value="A♯/B♭" selected=move || root_note.get() == Note::AisOrB>"A#/Bb"</option>
            <option value="H" selected=move || root_note.get() == Note::H>"B"</option>
          </select>
        </div>

        // Scale type selector
        <div class="space-y-2">
          <label class="block text-sm font-medium">"Scale Type"</label>
          <select
            class="w-full p-2 border rounded-md"
            on:change=move |ev| {
              let value = event_target_value(&ev);
              match value.as_str() {
                "Major" => scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Major)),
                "Minor" => scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Minor)),
                "Chromatic" => scale_type.set(ScaleType::Chromatic),
                _ => {}
              }
            }
          >
            <option value="Major" selected=move || matches!(scale_type.get(), ScaleType::Hepatonic(HeptaScaleType::Major))>"Major"</option>
            <option value="Minor" selected=move || matches!(scale_type.get(), ScaleType::Hepatonic(HeptaScaleType::Minor))>"Natural Minor"</option>
            <option value="Chromatic" selected=move || matches!(scale_type.get(), ScaleType::Chromatic)>"Chromatic"</option>
          </select>
        </div>

        // Fret range controls
        <div class="space-y-2">
          <label class="block text-sm font-medium">
            "Start Fret: " {move || start_fret.get()}
          </label>
          <input
            type="range"
            min="0"
            max=move || end_fret.get().saturating_sub(1)
            prop:value=move || start_fret.get()
            class="w-full"
            on:input=move |ev| {
              let val = event_target_value(&ev);
              if let Ok(val) = val.parse::<usize>() {
                if val < end_fret.get_untracked() {
                  start_fret.set(val);
                }
              }
            }
          />
        </div>

        <div class="space-y-2">
          <label class="block text-sm font-medium">
            "End Fret: " {move || end_fret.get()}
          </label>
          <input
            type="range"
            min=move || start_fret.get().saturating_add(1)
            max="22"
            prop:value=move || end_fret.get()
            class="w-full"
            on:input=move |ev| {
              let val = event_target_value(&ev);
              if let Ok(val) = val.parse::<usize>() {
                if val > start_fret.get_untracked() && val <= 22 {
                  end_fret.set(val);
                }
              }
            }
          />
        </div>
      </div>

      // Current scale info display
      <div class="p-4 rounded-lg bg-blue-50 border-2 border-blue-200">
        <h3 class="mb-2 text-lg font-semibold">"Current Scale"</h3>
        <div class="text-sm text-gray-700">
          <p><strong>"Root:"</strong> {move || root_note.get().to_string()}</p>
          <p><strong>"Scale:"</strong> {move || format!("{:?}", scale_type.get())}</p>
          <p><strong>"Range:"</strong> " Frets " {move || start_fret.get()} " - " {move || end_fret.get()}</p>
        </div>
      </div>

      // Click feedback
      <div class="p-4 rounded-lg bg-green-50 border-2 border-green-200">
        <strong>"Clicked Note: "</strong>
        {move || match clicked_note_event.get() {
          Some(event) => {
            format!(
              "{} - String {} - Fret {}",
              event.note.to_string(),
              event.coord.string_idx + 1,
              event.coord.fret_idx,
            )
          }
          None => "Click on the fretboard to test interaction".to_string(),
        }}
      </div>

      // Quick scale presets
      <div class="p-4 rounded-lg bg-yellow-50 border-2 border-yellow-200">
        <h3 class="mb-3 text-lg font-semibold">"Quick Presets"</h3>
        <div class="flex flex-wrap gap-2">
          <button
            class="px-4 py-2 text-white bg-blue-500 rounded hover:bg-blue-600"
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
            class="px-4 py-2 text-white bg-blue-500 rounded hover:bg-blue-600"
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
            class="px-4 py-2 text-white bg-blue-500 rounded hover:bg-blue-600"
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
            class="px-4 py-2 text-white bg-blue-500 rounded hover:bg-blue-600"
            on:click=move |_| {
              root_note.set(Note::C);
              scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Major));
              start_fret.set(7);
              end_fret.set(10);
            }
          >
            "C Major (7-10)"
          </button>
        </div>
      </div>

      // Main fretboard display
      <div class="p-4 rounded-lg bg-gray-50 border-2 border-gray-200">
        <SvgFretboardScaleDisplay
          start_fret=start_fret.read_only().into()
          end_fret=end_fret.read_only().into()
          root_note=root_note.read_only().into()
          scale_type=scale_type.read_only().into()
          on_note_clicked=on_note_clicked
        />
      </div>

      // Legend
      <div class="p-4 rounded-lg bg-gray-50 border-2 border-gray-200">
        <h3 class="mb-3 text-lg font-semibold">"Legend"</h3>
        <div class="flex flex-wrap gap-6">
          <div class="flex items-center gap-2">
            <div class="w-6 h-6 bg-red-500 rounded-full border-2 border-red-700"></div>
            <span>"Root Note"</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-5 h-5 bg-blue-500 rounded-full border-2 border-blue-700"></div>
            <span>"Scale Notes"</span>
          </div>
          <div class="flex items-center gap-2">
            <div class="w-4 h-4 bg-gray-400 rounded-full opacity-50"></div>
            <span>"Fret Markers"</span>
          </div>
        </div>
      </div>
    </div>
  }
}
