use crate::components::{
  fretboard_config_examples::FretboardConfigExamples,
  svg_fretboard::{SvgFretClickEvent, SvgFretboard},
};
use leptos::{ev, logging::log, prelude::*};

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

  // Track clicked fret for testing interactivity
  let (clicked_fret, set_clicked_fret) = signal::<Option<SvgFretClickEvent>>(None);

  let on_fret_clicked = Callback::new(move |event: SvgFretClickEvent| {
    log!(
      "Fret clicked: String {}, Fret {}",
      event.coord.string_idx,
      event.coord.fret_idx
    );
    set_clicked_fret.set(Some(event));
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
            if val < end_fret.get() {
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
            if val > start_fret.get() && val <= MAX_FRETS {
              end_fret.set(val);
            }
          }
        }
      />
    </div>

    // Display clicked fret info for testing
    <div style="margin-bottom: 1em; padding: 1em; background: #f0f0f0; border-radius: 5px;">
      <strong>"Click Test: "</strong>
      {move || match clicked_fret.get() {
        Some(event) => format!("String {} - Fret {}", event.coord.string_idx + 1, event.coord.fret_idx),
        None => "Click on a fret to test interactivity".to_string(),
      }}
    </div>

    <div style="margin: 5em;">
      <SvgFretboard
        start_fret=start_fret.read_only().into()
        end_fret=end_fret.read_only().into()
        on_fret_clicked=on_fret_clicked
      />
    </div>
    <FretboardConfigExamples />
  }
}
