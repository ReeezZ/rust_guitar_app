use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// Component for selecting a fret range with two sliders
/// Checks that start_fret <= end_fret
#[component]
pub fn FretRangeSelector(
  #[prop(into)] start_fret: Signal<u8>,
  #[prop(into)] on_start_fret_changed: Callback<u8>,
  #[prop(into)] end_fret: Signal<u8>,
  #[prop(into)] on_end_fret_changed: Callback<u8>,
  /// Label for the control
  label: &'static str,
  /// Minimum possible fret value
  #[prop(optional)]
  min: Option<usize>,
  /// Maximum possible fret value  
  #[prop(optional)]
  max: Option<usize>,
) -> impl IntoView {
  let min_fret = min.unwrap_or(0);
  let max_fret = max.unwrap_or(22);

  view! {
    <div class="space-y-2">
      <label class="block text-sm font-medium">{label}</label>

      // Current range display
      <div class="text-sm">
        "Range: " {move || format!("{}-{}", start_fret.get(), end_fret.get())}
        {move || {
          let start_fret = start_fret.get();
          let end_fret = end_fret.get();
          if start_fret == end_fret {
            if start_fret == 0 {
              " (open strings only)".to_string()
            } else {
              format!(" (fret {start_fret} only)")
            }
          } else if start_fret == 0 {
            " (includes open strings)".to_string()
          } else {
            " (excludes open strings)".to_string()
          }
        }}
      </div>

      // Start fret slider
      <div class="space-y-1">
        <label class="text-xs">"Start Fret: " {move || start_fret.get()}</label>
        <input
          type="range"
          min=min_fret
          max=move || end_fret.get()
          prop:value=move || start_fret.get()
          on:input=move |ev| {
            let target = ev.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            if let Ok(val) = input.value().parse::<u8>() {
              if val <= end_fret.get_untracked() {
                on_start_fret_changed.run(val);
              }
            }
          }
          class="w-full h-2 rounded-lg cursor-pointer"
        />
      </div>

      // End fret slider
      <div class="space-y-1">
        <label class="text-xs">"End Fret: " {move || end_fret.get()}</label>
        <input
          type="range"
          min=move || start_fret.get()
          max=max_fret
          prop:value=move || end_fret.get()
          on:input=move |ev| {
            let target = ev.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            if let Ok(val) = input.value().parse::<u8>() {
              if val >= start_fret.get_untracked() {
                on_end_fret_changed.run(val);
              }
            }
          }
          class="w-full h-2 rounded-lg cursor-pointer"
        />
      </div>
    </div>
  }
}
