use leptos::prelude::*;

use super::helper::get_fret_positions;
use crate::components::fretboard::{FretStateColor, FretStateSignals};

#[component]
pub(super) fn FretsEditor(
  frets: RwSignal<FretStateSignals>,
  #[prop(into)] label: RwSignal<String>,
  #[prop(into)] color: RwSignal<FretStateColor>,
  #[prop(into)] hidden: RwSignal<bool>,
) -> impl IntoView {
  let reset_sample = move |_| frets.set(get_fret_positions());

  view! {
    <div class="p-4 mt-4 space-y-3 bg-gray-50 rounded border">
      <h2 class="font-semibold">"Edit Fret State"</h2>
      <div class="flex flex-wrap gap-4 items-end">
        <label class="flex flex-col text-sm">
          <span>"Label"</span>
          <input
            r#type="text"
            class="p-1 rounded border"
            prop:value=move || label.get()
            on:input=move |ev| label.set(event_target_value(&ev))
          />
        </label>
        <label class="flex flex-col text-sm">
          <span>"Color"</span>
          <select
            class="p-1 rounded border"
            on:change=move |ev| {
              match event_target_value(&ev).as_str() {
                "Red" => color.set(FretStateColor::Red),
                "Blue" => color.set(FretStateColor::Blue),
                _ => color.set(FretStateColor::Green),
              }
            }
          >
            <option value="Green" selected=move || color.get() == FretStateColor::Green>
              "Green"
            </option>
            <option value="Red" selected=move || color.get() == FretStateColor::Red>
              "Red"
            </option>
            <option value="Blue" selected=move || color.get() == FretStateColor::Blue>
              "Blue"
            </option>
          </select>
        </label>
        <label class="flex gap-2 items-center mt-4 text-sm">
          <input
            r#type="checkbox"
            prop:checked=move || hidden.get()
            on:change=move |ev| {
              use leptos::wasm_bindgen::JsCast;
              if let Some(t) = ev.target() {
                if let Ok(input) = t.dyn_into::<web_sys::HtmlInputElement>() {
                  hidden.set(input.checked());
                }
              }
            }
          />
          <span>"Hidden"</span>
        </label>
        <button class="py-1 px-3 text-gray-800 bg-gray-300 rounded" on:click=reset_sample>
          "Reset Sample"
        </button>
      </div>
    </div>
  }
}
