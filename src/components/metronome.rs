use leptos::prelude::*;
use leptos_use::use_interval_fn;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MetronomeState {
  Stopped,
  Running,
}

// Thread-local AudioContext for the metronome
thread_local! {
  static AUDIO_CONTEXT: RefCell<Option<Rc<AudioContext>>> = const { RefCell::new(None) };
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

  // Calculate interval from BPM as a reactive signal
  let interval_signal = Memo::new(move |_| {
    let bpm_val = bpm.get();
    if bpm_val > 0 {
      (60000.0 / bpm_val as f64) as u64
    } else {
      1000 // fallback to 60 BPM
    }
  });

  // Metronome tick function
  let tick = move || {
    if metronome_state.get() == MetronomeState::Running {
      // Compute next beat (1-4 for 4/4 time)
      let next_beat = {
        let beat = current_beat.get();
        if beat >= 4 {
          1
        } else {
          beat + 1
        }
      };

      // Play sound for the NEW beat
      play_click(next_beat == 1);

      // Update beat counter
      set_current_beat.set(next_beat);
    }
  };

  // Set up reactive interval for metronome ticking that updates with BPM
  let metronome_interval = use_interval_fn(tick, interval_signal);

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
    <div class="p-4 bg-white rounded-lg border border-gray-200">
      <h4 class="mb-3 font-semibold text-gray-800 text-md">"Metronome"</h4>

      <div class="mb-4 text-center">
        // BPM Display and Control
        <div class="flex justify-center items-center mb-4 space-x-4">
          <button
            class="flex justify-center items-center w-8 h-8 text-sm font-bold bg-gray-200 rounded-full hover:bg-gray-300"
            on:click=move |_| {
              let new_bpm = (bpm.get().saturating_sub(5)).max(30);
              handle_bpm_change(new_bpm);
            }
          >
            "−"
          </button>

          <div class="text-center">
            <div class="text-2xl font-bold text-gray-800">{move || bpm.get().to_string()}</div>
            <div class="text-xs text-gray-500">"BPM"</div>
          </div>

          <button
            class="flex justify-center items-center w-8 h-8 text-sm font-bold bg-gray-200 rounded-full hover:bg-gray-300"
            on:click=move |_| {
              let new_bpm = (bpm.get() + 5).min(250);
              handle_bpm_change(new_bpm);
            }
          >
            "+"
          </button>
        </div>

        // Beat indicator (4 dots for 4/4 time)
        <div class="flex justify-center mb-4 space-x-2">
          {(1..=4)
            .map(|beat_num| {
              view! {
                <div class=move || {
                  let base_classes = "w-3 h-3 rounded-full";
                  if current_beat.get() == beat_num
                    && metronome_state.get() == MetronomeState::Running
                  {
                    if beat_num == 1 {
                      format!("{base_classes} bg-red-500")
                    } else {
                      format!("{base_classes} bg-blue-500")
                    }
                  } else {
                    format!("{base_classes} bg-gray-300")
                  }
                }></div>
              }
            })
            .collect_view()}
        </div>

        // Start/Stop button
        <button
          class=move || {
            match metronome_state.get() {
              MetronomeState::Running => {
                "bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-4 rounded-lg"
              }
              MetronomeState::Stopped => {
                "bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded-lg"
              }
            }
          }
          on:click=toggle_metronome
        >
          {move || {
            match metronome_state.get() {
              MetronomeState::Running => "Stop",
              MetronomeState::Stopped => "Start",
            }
          }}
        </button>

        // State indicator
        <p class="mt-2 text-xs text-gray-500">
          {move || {
            match metronome_state.get() {
              MetronomeState::Stopped => "Click start to begin metronome".to_string(),
              MetronomeState::Running => format!("Beat {}/4", current_beat.get()),
            }
          }}
        </p>
      </div>
    </div>
  }
}

/// Play a click sound using Web Audio API with shared AudioContext
fn play_click(is_accent: bool) {
  let _ = AUDIO_CONTEXT.with(|ctx| -> Result<(), JsValue> {
    // Get or create the AudioContext without any cloning
    let mut ctx_ref = ctx.borrow_mut();
    let audio_ctx = match ctx_ref.as_ref() {
      Some(context) => context.as_ref(),
      None => {
        let new_context = Rc::new(AudioContext::new()?);
        *ctx_ref = Some(new_context.clone());
        ctx_ref.as_ref().unwrap().as_ref()
      }
    };

    // Create oscillator for the click sound
    let oscillator = audio_ctx.create_oscillator()?;
    let gain = audio_ctx.create_gain()?;

    // Connect audio nodes: oscillator -> gain -> destination
    oscillator.connect_with_audio_node(&gain)?;
    gain.connect_with_audio_node(&audio_ctx.destination())?;

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
    let current_time = audio_ctx.current_time();
    oscillator.start_with_when(current_time)?;
    oscillator.stop_with_when(current_time + 0.1)?; // 100ms click

    // Fade out to avoid clicking
    gain
      .gain()
      .exponential_ramp_to_value_at_time(0.01, current_time + 0.1)?;

    Ok(())
  });
}
