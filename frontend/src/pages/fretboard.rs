use leptos::{ev, logging::log, prelude::*};

use crate::components::fretboard::{
  config_examples::FretboardConfigExamples,
  with_notes::{FretClickEventWithNote, FretboardWithNotes},
};

/// Extracts the value from an input event.
/// See: https://leptos.dev/docs/reference/events/
fn event_target_value(ev: &ev::Event) -> String {
  use leptos::wasm_bindgen::JsCast;
  let target = ev.target().unwrap();
  let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
  input.value()
}

/// Page for the SVG fretboard with a runtime-adjustable fret count slider.
/// See: https://leptos.dev/docs/reference/signals/
#[component]
pub fn SvgFretboardPage() -> impl IntoView {
  const MAX_FRETS: usize = 22;
  let start_fret = RwSignal::new(0_usize);
  let end_fret = RwSignal::new(5_usize);

  // Track clicked note for testing note-aware interactivity
  let (clicked_note_event, set_clicked_note_event) = signal::<Option<FretClickEventWithNote>>(None);

  let on_note_clicked = Callback::new(move |event: FretClickEventWithNote| {
    log!(
      "🎵 SVG Fretboard (with notes) - Note: {}, String: {} (1-indexed: {}), Fret: {}",
      event.note,
      event.coord.string_idx,
      event.coord.string_idx + 1,
      event.coord.fret_idx
    );
    set_clicked_note_event.set(Some(event));
  });

  view! {
    <div style="margin-bottom: 1em;">
      <label for="start-fret-slider">"Start Fret: " {move || start_fret.get()}</label>
      <input
        id="start-fret-slider"
        type="range"
        min="0"
        max=move || end_fret.get().saturating_sub(1)
        prop:value=move || start_fret.get()
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
    <div style="margin-bottom: 1em;">
      <label for="end-fret-slider">"End Fret: " {move || end_fret.get()}</label>
      <input
        id="end-fret-slider"
        type="range"
        min=move || start_fret.get().saturating_add(1)
        max=MAX_FRETS
        prop:value=move || end_fret.get()
        on:input=move |ev| {
          let val = event_target_value(&ev);
          if let Ok(val) = val.parse::<usize>() {
            if val > start_fret.get_untracked() && val <= MAX_FRETS {
              end_fret.set(val);
            }
          }
        }
      />
    </div>

    <div style="margin-bottom: 1em; padding: 1em; background: #e8f4fd; border-radius: 5px;">
      <strong>"SVG Fretboard (with notes): "</strong>
      {move || match clicked_note_event.get() {
        Some(event) => {
          format!(
            "Note: {} - String {} - Fret {}",
            event.note,
            event.coord.string_idx + 1,
            event.coord.fret_idx,
          )
        }
        None => "Click on the second fretboard to test note-aware interaction".to_string(),
      }}
    </div>

    <h3>"2. SVG Fretboard (With Note Information)"</h3>
    <div style="margin: 2em; border: 2px solid #4a90e2; padding: 1em; border-radius: 5px;">
      <FretboardWithNotes
        start_fret=start_fret.read_only().into()
        end_fret=end_fret.read_only().into()
        on_note_clicked=on_note_clicked
      />
    </div>
    <FretboardConfigExamples />
  }
}
