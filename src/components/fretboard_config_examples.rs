/// Interactive fretboard configuration playground
///
/// This component provides a single fretboard with all configuration parameters
/// exposed as interactive controls, allowing users to experiment with different
/// settings in real-time and understand their effects.
use crate::components::{
  fretboard::FretClickEvent, svg_fretboard_with_notes::SvgFretboardWithNotes,
};
use leptos::{logging::log, prelude::*};

#[component]
pub fn FretboardConfigExamples() -> impl IntoView {
  // Fret range controls
  let start_fret = RwSignal::new(3_usize);
  let end_fret = RwSignal::new(7_usize);

  // Configuration controls
  let num_strings = RwSignal::new(6_u8);
  let max_frets = RwSignal::new(22_usize);
  let svg_aspect_ratio = RwSignal::new(3.0_f64);
  let fret_margin_percentage = RwSignal::new(0.05_f64);
  let nut_width = RwSignal::new(14.0_f64);
  let extra_frets = RwSignal::new(1_usize);
  let marker_preset = RwSignal::new("standard".to_string());

  // Convert marker preset to actual marker positions
  let marker_positions = Memo::new(move |_| match marker_preset.get().as_str() {
    "standard" => vec![3_u8, 5, 7, 9, 12, 15, 17, 19, 21, 24],
    "octaves" => vec![12_u8, 24],
    "pentatonic" => vec![3_u8, 5, 7, 12, 15, 17, 24],
    "none" => vec![],
    _ => vec![3_u8, 5, 7, 9, 12, 15, 17, 19, 21, 24],
  });

  view! {
    <div class="space-y-3">
      <h2 class="text-2xl font-bold">"Interactive Fretboard Configuration"</h2>

      // Quick presets
      <div class="p-2 bg-green-50 rounded-lg border">
        <h3 class="mb-4 text-lg font-semibold">"⚡ Quick Presets"</h3>
        <div class="flex flex-wrap gap-2 justify-center items-center">
          <button
            class="py-2 px-4 text-white bg-green-500 rounded hover:bg-green-600"
            on:click=move |_| {
              num_strings.set(6);
              svg_aspect_ratio.set(3.0);
              max_frets.set(22);
              marker_preset.set("standard".to_string());
            }
          >
            "🎸 Standard Guitar"
          </button>
          <button
            class="py-2 px-4 text-white bg-green-500 rounded hover:bg-green-600"
            on:click=move |_| {
              num_strings.set(4);
              svg_aspect_ratio.set(4.0);
              max_frets.set(20);
              marker_preset.set("standard".to_string());
            }
          >
            "🎵 Bass Guitar"
          </button>
          <button
            class="py-2 px-4 text-white bg-green-500 rounded hover:bg-green-600"
            on:click=move |_| {
              num_strings.set(7);
              svg_aspect_ratio.set(2.8);
              max_frets.set(24);
              extra_frets.set(2);
              marker_preset.set("standard".to_string());
            }
          >
            "🎸 7-String"
          </button>
          <button
            class="py-2 px-4 text-white bg-green-500 rounded hover:bg-green-600"
            on:click=move |_| {
              svg_aspect_ratio.set(2.2);
              fret_margin_percentage.set(0.02);
              nut_width.set(10.0);
            }
          >
            "� Compact"
          </button>
          <button
            class="py-2 px-4 text-white bg-green-500 rounded hover:bg-green-600"
            on:click=move |_| {
              svg_aspect_ratio.set(5.0);
              extra_frets.set(3);
              fret_margin_percentage.set(0.08);
              marker_preset.set("standard".to_string());
            }
          >
            "📺 Ultra-wide"
          </button>
        </div>
      </div>

      // Main layout: fretboard and controls side by side on large screens
      <div class="flex flex-col gap-6 xl:flex-row">
        // Main fretboard display - responsive sizing, not too constrained
        <div class="p-4 bg-blue-50 rounded-lg border-2 border-blue-200 xl:min-w-0 xl:flex-[2]">
          <SvgFretboardWithNotes
            start_fret=start_fret.read_only().into()
            end_fret=end_fret.read_only().into()
            num_strings=num_strings.read_only()
            max_frets=max_frets.read_only()
            svg_aspect_ratio=svg_aspect_ratio.read_only()
            fret_margin_percentage=fret_margin_percentage.read_only()
            nut_width=nut_width.read_only()
            extra_frets=extra_frets.read_only()
            marker_positions=Signal::derive(move || marker_positions.get())
            on_note_clicked=Callback::new(|event: FretClickEvent| {
              log!(
                "Fret clicked: String {}, Fret {}, Note {}", event.coord.string_idx, event.coord.fret_idx, event.note
              );
            })
          />
        </div>

        // Configuration controls panel - compact and efficient use of space
        <div class="xl:flex-1 xl:max-w-lg">
          // Configuration controls organized in compact sections
          <div class="space-y-4">

            // Fret Range Controls - compact horizontal layout
            <div class="p-1 rounded-lg border">
              <h3 class="mb-3 text-base font-semibold">"🎯 Fret Range"</h3>
              <div class="grid grid-cols-2 gap-3">
                <div>
                  <label class="block mb-1 text-xs font-medium">
                    "Start: " <span class="font-bold">{move || start_fret.get()}</span>
                  </label>
                  <input
                    type="range"
                    min="0"
                    max="20"
                    class="w-full"
                    prop:value=move || start_fret.get()
                    on:input=move |ev| {
                      if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                        if val < end_fret.get() {
                          start_fret.set(val);
                        }
                      }
                    }
                  />
                </div>
                <div>
                  <label class="block mb-1 text-xs font-medium">
                    "End: " <span class="font-bold">{move || end_fret.get()}</span>
                  </label>
                  <input
                    type="range"
                    min="1"
                    max="24"
                    class="w-full"
                    prop:value=move || end_fret.get()
                    on:input=move |ev| {
                      if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                        if val > start_fret.get() {
                          end_fret.set(val);
                        }
                      }
                    }
                  />
                </div>
              </div>
            </div>

            // Instrument Configuration - compact horizontal layout
            <div class="p-1 rounded-lg border">
              <h3 class="mb-3 text-base font-semibold">"🎸 Instrument"</h3>
              <div class="grid grid-cols-2 gap-3">
                <div>
                  <label class="block mb-1 text-xs font-medium">
                    "Strings: " <span class="font-bold">{move || num_strings.get()}</span>
                  </label>
                  <input
                    type="range"
                    min="4"
                    max="8"
                    class="w-full"
                    prop:value=move || num_strings.get()
                    on:input=move |ev| {
                      if let Ok(val) = event_target_value(&ev).parse::<u8>() {
                        num_strings.set(val);
                      }
                    }
                  />
                  <div class="mt-1 text-xs text-gray-600">
                    {move || match num_strings.get() {
                      4 => "Bass",
                      6 => "Guitar",
                      7 => "7-String",
                      8 => "8-String",
                      _ => "Custom",
                    }}
                  </div>
                </div>
                <div>
                  <label class="block mb-1 text-xs font-medium">
                    "Max Frets: " <span class="font-bold">{move || max_frets.get()}</span>
                  </label>
                  <input
                    type="range"
                    min="12"
                    max="27"
                    class="w-full"
                    prop:value=move || max_frets.get()
                    on:input=move |ev| {
                      if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                        max_frets.set(val);
                      }
                    }
                  />
                </div>
              </div>
            </div>

            // Visual Layout & Fine Tuning - combined compact layout
            <div class="p-1 rounded-lg border">
              <h3 class="mb-3 text-base font-semibold">"📐 Layout & Tuning"</h3>
              <div class="grid grid-cols-2 gap-3">
                <div>
                  <label class="block mb-1 text-xs font-medium">
                    "Aspect: "
                    <span class="font-bold">
                      {move || format!("{:.1}:1", svg_aspect_ratio.get())}
                    </span>
                  </label>
                  <input
                    type="range"
                    min="1.5"
                    max="6.0"
                    step="0.1"
                    class="w-full"
                    prop:value=move || svg_aspect_ratio.get()
                    on:input=move |ev| {
                      if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                        svg_aspect_ratio.set(val);
                      }
                    }
                  />
                  <div class="mt-1 text-xs text-gray-600">
                    {move || {
                      if svg_aspect_ratio.get() < 2.5 {
                        "Compact"
                      } else if svg_aspect_ratio.get() > 4.0 {
                        "Ultra-wide"
                      } else {
                        "Standard"
                      }
                    }}
                  </div>
                </div>
                <div>
                  <label class="block mb-1 text-xs font-medium">
                    "Margin: "
                    <span class="font-bold">
                      {move || format!("{:.1}%", fret_margin_percentage.get() * 100.0)}
                    </span>
                  </label>
                  <input
                    type="range"
                    min="0.01"
                    max="0.15"
                    step="0.01"
                    class="w-full"
                    prop:value=move || fret_margin_percentage.get()
                    on:input=move |ev| {
                      if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                        fret_margin_percentage.set(val);
                      }
                    }
                  />
                </div>
              </div>
              <div class="grid grid-cols-2 gap-3 mt-3">
                <div>
                  <label class="block mb-1 text-xs font-medium">
                    "Nut Width: "
                    <span class="font-bold">{move || format!("{:.0}px", nut_width.get())}</span>
                  </label>
                  <input
                    type="range"
                    min="8"
                    max="25"
                    class="w-full"
                    prop:value=move || nut_width.get()
                    on:input=move |ev| {
                      if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                        nut_width.set(val);
                      }
                    }
                  />
                </div>
                <div>
                  <label class="block mb-1 text-xs font-medium">
                    "Extra Frets: " <span class="font-bold">{move || extra_frets.get()}</span>
                  </label>
                  <input
                    type="range"
                    min="0"
                    max="5"
                    class="w-full"
                    prop:value=move || extra_frets.get()
                    on:input=move |ev| {
                      if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                        extra_frets.set(val);
                      }
                    }
                  />
                </div>
              </div>
            </div>

            // Markers - compact layout
            <div class="p-1 rounded-lg border">
              <h3 class="mb-3 text-base font-semibold">"🎯 Fret Markers"</h3>
              <div class="flex gap-2 justify-center items-center">
                <button
                  class=move || {
                    format!(
                      "inline-block px-2 py-1 rounded text-xs m-1 {}",
                      if marker_preset.get() == "standard" {
                        "bg-blue-500 text-white"
                      } else {
                        "bg-gray-200 hover:bg-gray-300"
                      },
                    )
                  }
                  on:click=move |_| marker_preset.set("standard".to_string())
                >
                  "Standard"
                </button>
                <button
                  class=move || {
                    format!(
                      "inline-block px-2 py-1 rounded text-xs m-1 {}",
                      if marker_preset.get() == "octaves" {
                        "bg-blue-500 text-white"
                      } else {
                        "bg-gray-200 hover:bg-gray-300"
                      },
                    )
                  }
                  on:click=move |_| marker_preset.set("octaves".to_string())
                >
                  "Octaves"
                </button>
                <button
                  class=move || {
                    format!(
                      "inline-block px-2 py-1 rounded text-xs m-1 {}",
                      if marker_preset.get() == "pentatonic" {
                        "bg-blue-500 text-white"
                      } else {
                        "bg-gray-200 hover:bg-gray-300"
                      },
                    )
                  }
                  on:click=move |_| marker_preset.set("pentatonic".to_string())
                >
                  "Pentatonic"
                </button>
                <button
                  class=move || {
                    format!(
                      "inline-block px-2 py-1 rounded text-xs m-1 {}",
                      if marker_preset.get() == "none" {
                        "bg-blue-500 text-white"
                      } else {
                        "bg-gray-200 hover:bg-gray-300"
                      },
                    )
                  }
                  on:click=move |_| marker_preset.set("none".to_string())
                >
                  "None"
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

    </div>
  }
}

fn event_target_value(ev: &leptos::ev::Event) -> String {
  use leptos::wasm_bindgen::JsCast;
  let target = ev.target().unwrap();
  let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
  input.value()
}
