use crate::components::{
  fretboard_config_examples::FretboardConfigExamples, svg_fretboard::SvgFretboard,
};
use leptos::{ev, prelude::*};

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
    <div style="margin: 5em;">
      <SvgFretboard start_fret=start_fret.read_only().into() end_fret=end_fret.read_only().into() />
    </div>
    <FretboardConfigExamples />
  }
}
