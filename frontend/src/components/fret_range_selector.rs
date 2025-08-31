use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// Component for selecting a fret range with two sliders
#[component]
pub fn FretRangeSelector(
  #[prop(into)] start_fret: Signal<usize>,
  #[prop(into)] end_fret: Signal<usize>,
  /// Label for the control
  label: &'static str,
  /// Minimum possible fret value
  #[prop(optional)]
  min: Option<usize>,
  /// Maximum possible fret value  
  #[prop(optional)]
  max: Option<usize>,
  #[prop(into)] on_start_fret_change: Callback<usize>,
  #[prop(into)] on_end_fret_change: Callback<usize>,
) -> impl IntoView {
  let min_fret = min.unwrap_or(0);
  let max_fret = max.unwrap_or(22);

  // Sync individual signals with the range signal changes from outside
  Effect::new(move |_| {
    let new_start = start_fret.get();
    let new_end = end_fret.get();

    if start_fret.get_untracked() != new_start {
      on_start_fret_change.run(new_start);
    }
    if end_fret.get_untracked() != new_end {
      on_end_fret_change.run(new_end);
    }
  });

  view! {
    <div class="space-y-2">
      <label class="block text-sm font-medium text-gray-700">{label}</label>

      // Current range display
      <div class="text-sm text-gray-600">
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
        <label class="text-xs text-gray-500">"Start Fret: " {move || start_fret.get()}</label>
        <input
          type="range"
          min=min_fret
          max=move || end_fret.get()
          prop:value=move || start_fret.get()
          on:input=move |ev| {
            let target = ev.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            if let Ok(val) = input.value().parse::<usize>() {
              if val <= end_fret.get_untracked() {
                on_start_fret_change.run(val);
              }
            }
          }
          class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
        />
      </div>

      // End fret slider
      <div class="space-y-1">
        <label class="text-xs text-gray-500">"End Fret: " {move || end_fret.get()}</label>
        <input
          type="range"
          min=move || start_fret.get()
          max=max_fret
          prop:value=move || end_fret.get()
          on:input=move |ev| {
            let target = ev.target().unwrap();
            let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
            if let Ok(val) = input.value().parse::<usize>() {
              if val >= start_fret.get_untracked() {
                on_end_fret_change.run(val);
              }
            }
          }
          class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
        />
      </div>
    </div>
  }
}
