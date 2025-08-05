use crate::music::heptatonic_scales::HeptaScaleType;
use crate::music::notes::Note;
use crate::music::scales::ScaleType;
use leptos::prelude::*;

/// Extracts the value from an HTML input or select event.
/// This is a helper function used by all selector components.
fn event_target_value(ev: &leptos::ev::Event) -> String {
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

/// A reusable note selector dropdown component.
///
/// Automatically generates all 12 chromatic notes from the Note enum,
/// eliminating code duplication and reactive performance issues.
#[component]
pub fn NoteSelector(
  /// The reactive signal containing the selected note
  value: RwSignal<Note>,
  /// Optional label for the selector (defaults to "Note")
  #[prop(optional)]
  label: Option<&'static str>,
  /// Optional CSS classes for styling
  #[prop(optional, into)]
  class: Option<String>,
) -> impl IntoView {
  let label_text = label.unwrap_or("Note");
  let css_class = class.unwrap_or_else(|| "w-full p-2 border rounded-md".to_string());

  view! {
    <div class="space-y-2">
      <label class="block text-sm font-medium">{label_text}</label>
      <select
        class=css_class
        on:change=move |ev| {
          let note_str = event_target_value(&ev);
          if let Ok(note) = note_str.parse::<Note>() {
            value.set(note);
          }
        }
      >
        // Generate options dynamically from Note::mapping()
        {Note::mapping()
          .iter()
          .map(|(note, display_str)| {
            let current_note = *note;
            let display_text = *display_str;

            view! {
              <option value=display_text selected=move || value.get() == current_note>
                {display_text}
              </option>
            }
          })
          .collect_view()}
      </select>
    </div>
  }
}

/// A reusable scale type selector dropdown component.
///
/// Currently supports Major, Minor, and Chromatic scales.
/// Can be easily extended to support more scale types.
#[component]
pub fn ScaleTypeSelector(
  /// The reactive signal containing the selected scale type
  value: RwSignal<ScaleType>,
  /// Optional label for the selector (defaults to "Scale Type")
  #[prop(optional)]
  label: Option<&'static str>,
  /// Optional CSS classes for styling
  #[prop(optional, into)]
  class: Option<String>,
) -> impl IntoView {
  let label_text = label.unwrap_or("Scale Type");
  let css_class = class.unwrap_or_else(|| "w-full p-2 border rounded-md".to_string());

  // Define available scale types
  // TODO: This could be made more dynamic in the future
  let scale_options = [
    (
      "Major",
      "Major",
      ScaleType::Hepatonic(HeptaScaleType::Major),
    ),
    (
      "Minor",
      "Natural Minor",
      ScaleType::Hepatonic(HeptaScaleType::Minor),
    ),
    ("Chromatic", "Chromatic", ScaleType::Chromatic),
  ];

  view! {
    <div class="space-y-2">
      <label class="block text-sm font-medium">{label_text}</label>
      <select
        class=css_class
        on:change=move |ev| {
          let scale_str = event_target_value(&ev);
          match scale_str.as_str() {
            "Major" => value.set(ScaleType::Hepatonic(HeptaScaleType::Major)),
            "Minor" => value.set(ScaleType::Hepatonic(HeptaScaleType::Minor)),
            "Chromatic" => value.set(ScaleType::Chromatic),
            _ => {}
          }
        }
      >
        {scale_options
          .iter()
          .map(|(option_value, display_text, scale_type)| {
            let current_scale_type = *scale_type;
            let option_val = *option_value;
            let display_txt = *display_text;

            view! {
              <option
                value=option_val
                selected=move || {
                  match (value.get(), current_scale_type) {
                    (
                      ScaleType::Hepatonic(HeptaScaleType::Major),
                      ScaleType::Hepatonic(HeptaScaleType::Major),
                    ) => true,
                    (
                      ScaleType::Hepatonic(HeptaScaleType::Minor),
                      ScaleType::Hepatonic(HeptaScaleType::Minor),
                    ) => true,
                    (ScaleType::Chromatic, ScaleType::Chromatic) => true,
                    _ => false,
                  }
                }
              >
                {display_txt}
              </option>
            }
          })
          .collect_view()}
      </select>
    </div>
  }
}

/// A reusable numeric range selector component.
///
/// Provides a labeled range input with dynamic min/max values.
/// Commonly used for fret range selection.
#[component]
pub fn NumericRangeSelector(
  /// The reactive signal containing the selected value
  value: RwSignal<usize>,
  /// Label for the selector
  label: &'static str,
  /// Minimum value (can be static or reactive)
  #[prop(into)]
  min: Signal<usize>,
  /// Maximum value (can be static or reactive)
  #[prop(into)]
  max: Signal<usize>,
  /// Optional CSS classes for styling
  #[prop(optional, into)]
  class: Option<String>,
) -> impl IntoView {
  let css_class = class.unwrap_or_else(|| "w-full".to_string());

  view! {
    <div class="space-y-2">
      <label class="block text-sm font-medium">{label} ": " {move || value.get()}</label>
      <input
        type="range"
        min=move || min.get()
        max=move || max.get()
        prop:value=move || value.get()
        class=css_class
        on:input=move |ev| {
          let val = event_target_value(&ev);
          if let Ok(val) = val.parse::<usize>() {
            let min_val = min.get();
            let max_val = max.get();
            if val >= min_val && val <= max_val {
              value.set(val);
            }
          }
        }
      />
    </div>
  }
}
