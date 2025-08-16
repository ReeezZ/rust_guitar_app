use leptos::prelude::*;
use leptos_use::use_interval_fn;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetronomeState {
  Stopped,
  Running,
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[component]
pub fn Metronome(
  /// BPM (beats per minute) signal
  bpm: ReadSignal<u32>,
  /// Optional callback when BPM changes
  #[prop(optional)]
  on_bpm_change: Option<Callback<u32>>,
) -> impl IntoView {
  let (metronome_state, set_metronome_state) = signal(MetronomeState::Stopped);
  let (current_beat, set_current_beat) = signal(1u8);

  // Calculate interval from BPM (60000ms / BPM = ms per beat)
  let interval_ms = move || {
    let bpm_val = bpm.get();
    if bpm_val > 0 {
      (60000.0 / bpm_val as f64) as u64
    } else {
      1000 // fallback to 60 BPM
    }
  };

  // Metronome tick function
  let tick = move || {
    if metronome_state.get() == MetronomeState::Running {
      // Update beat counter (1-4 for 4/4 time) FIRST
      set_current_beat.update(|beat| {
        *beat = if *beat >= 4 { 1 } else { *beat + 1 };
      });
      
      // Then play sound for the NEW beat
      play_click(current_beat.get() == 1);
    }
  };

  // Set up interval for metronome ticking
  let metronome_interval = use_interval_fn(tick, interval_ms());

  // Start/stop metronome
  let toggle_metronome = move |_| {
    match metronome_state.get() {
      MetronomeState::Stopped => {
        set_current_beat.set(1);
        set_metronome_state.set(MetronomeState::Running);
        // Play the first beat immediately
        play_click(true); // Beat 1 is always the accent
        (metronome_interval.resume)();
      }
      MetronomeState::Running => {
        set_metronome_state.set(MetronomeState::Stopped);
        (metronome_interval.pause)();
      }
    }
  };

  // Handle BPM changes
  let handle_bpm_change = move |new_bpm: u32| {
    if let Some(callback) = on_bpm_change {
      callback.run(new_bpm);
    }
  };

  view! {
      <div class="bg-white p-4 rounded-lg border border-gray-200">
          <h4 class="text-md font-semibold text-gray-800 mb-3">"Metronome"</h4>

          <div class="text-center mb-4">
              // BPM Display and Control
              <div class="flex items-center justify-center space-x-4 mb-4">
                  <button 
                      class="w-8 h-8 bg-gray-200 hover:bg-gray-300 rounded-full flex items-center justify-center text-sm font-bold"
                      on:click=move |_| {
                          let new_bpm = (bpm.get().saturating_sub(5)).max(30);
                          handle_bpm_change(new_bpm);
                      }
                  >
                      "−"
                  </button>
                  
                  <div class="text-center">
                      <div class="text-2xl font-bold text-gray-800">
                          {move || bpm.get().to_string()}
                      </div>
                      <div class="text-xs text-gray-500">"BPM"</div>
                  </div>
                  
                  <button 
                      class="w-8 h-8 bg-gray-200 hover:bg-gray-300 rounded-full flex items-center justify-center text-sm font-bold"
                      on:click=move |_| {
                          let new_bpm = (bpm.get() + 5).min(250);
                          handle_bpm_change(new_bpm);
                      }
                  >
                      "+"
                  </button>
              </div>

              // Beat indicator (4 dots for 4/4 time)
              <div class="flex justify-center space-x-2 mb-4">
                  {(1..=4).map(|beat_num| {
                      view! {
                          <div class={move || {
                              let base_classes = "w-3 h-3 rounded-full";
                              if current_beat.get() == beat_num && metronome_state.get() == MetronomeState::Running {
                                  if beat_num == 1 {
                                      format!("{base_classes} bg-red-500") // Accent on beat 1
                                  } else {
                                      format!("{base_classes} bg-blue-500")
                                  }
                              } else {
                                  format!("{base_classes} bg-gray-300")
                              }
                          }}></div>
                      }
                  }).collect_view()}
              </div>

              // Start/Stop button
              <button
                  class={move || {
                      match metronome_state.get() {
                          MetronomeState::Running => "bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-4 rounded-lg",
                          MetronomeState::Stopped => "bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded-lg"
                      }
                  }}
                  on:click=toggle_metronome
              >
                  {move || {
                      match metronome_state.get() {
                          MetronomeState::Running => "Stop",
                          MetronomeState::Stopped => "Start"
                      }
                  }}
              </button>

              // State indicator
              <p class="text-xs text-gray-500 mt-2">
                  {move || {
                      match metronome_state.get() {
                          MetronomeState::Stopped => "Click start to begin metronome".to_string(),
                          MetronomeState::Running => format!("Beat {}/4", current_beat.get())
                      }
                  }}
              </p>
          </div>
      </div>
  }
}

/// Play a click sound using Web Audio API
fn play_click(is_accent: bool) {
  let _ = (|| -> Result<(), JsValue> {
    // Create fresh audio context for each click to avoid storing it
    let ctx = AudioContext::new()?;
    
    // Create oscillator for the click sound
    let oscillator = ctx.create_oscillator()?;
    let gain = ctx.create_gain()?;

    // Connect audio nodes: oscillator -> gain -> destination
    oscillator.connect_with_audio_node(&gain)?;
    gain.connect_with_audio_node(&ctx.destination())?;

    // Configure the click sound
    if is_accent {
      // Accent beat (beat 1) - lower pitch, longer duration
      oscillator.frequency().set_value(800.0);
      gain.gain().set_value(0.3);
    } else {
      // Regular beats - higher pitch, shorter duration
      oscillator.frequency().set_value(1200.0);
      gain.gain().set_value(0.2);
    }

    oscillator.set_type(OscillatorType::Square);

    // Start and stop the oscillator for a short click
    let current_time = ctx.current_time();
    oscillator.start_with_when(current_time)?;
    oscillator.stop_with_when(current_time + 0.1)?; // 100ms click

    // Fade out to avoid clicking
    gain.gain().exponential_ramp_to_value_at_time(0.01, current_time + 0.1)?;

    Ok(())
  })();
}
