use leptos::{ev, logging::log, prelude::*};

use crate::components::fretboard::{
  with_notes::{FretClickEventWithNote, FretboardWithNotes},
  with_overlay::FretboardWithOverlay,
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
pub fn FretboardDevPage() -> impl IntoView {
  view! {
    <h1>"FretboardWithOverlay"</h1>
    <div>
      <FretboardWithOverlay />
    </div>
  }
}
