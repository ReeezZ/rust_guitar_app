use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/// Component for selecting a fret range with two sliders
#[component]
pub fn FretRangeSelector(
  /// The range signal to update
  value: RwSignal<std::ops::RangeInclusive<usize>>,
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

  // Extract start and end from the range for individual sliders
  let start_fret = RwSignal::new(*value.get_untracked().start());
  let end_fret = RwSignal::new(*value.get_untracked().end());

  // Update the range when either slider changes
  let update_range = move || {
    let start = start_fret.get();
    let end = end_fret.get();
    if start <= end {
      value.set(start..=end);
    }
  };

  // Sync individual signals with the range signal changes from outside
  Effect::new(move |_| {
    let range = value.get();
    let new_start = *range.start();
    let new_end = *range.end();
    
    if start_fret.get_untracked() != new_start {
      start_fret.set(new_start);
    }
    if end_fret.get_untracked() != new_end {
      end_fret.set(new_end);
    }
  });

  view! {
    <div class="space-y-2">
      <label class="block text-sm font-medium text-gray-700">{label}</label>
      
      // Current range display
      <div class="text-sm text-gray-600">
        "Range: " 
        {move || format!("{}-{}", start_fret.get(), end_fret.get())}
        {move || {
          let range = value.get();
          if range.start() == range.end() {
            if *range.start() == 0 {
              " (open strings only)".to_string()
            } else {
              format!(" (fret {} only)", range.start())
            }
          } else if *range.start() == 0 {
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
                start_fret.set(val);
                update_range();
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
                end_fret.set(val);
                update_range();
              }
            }
          }
          class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
        />
      </div>
    </div>
  }
}
