use leptos::prelude::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;
use shared::Scale;
use std::time::Duration;

use super::ConfigurationHeader;
use crate::components::fretboard::FretboardModelAdapter;
use crate::components::metronome::Metronome;
use crate::models::fretboard::{FretboardModelBuilder, FretboardModelExt};
use shared::models::exercise::{Exercise, ExerciseType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerState {
  Stopped,
  Running,
  Paused,
}

#[component]
pub fn PracticeSession(
  #[prop(optional)] target_time: Option<Duration>,
  /// Optional exercise for exercise-specific features like fretboard display
  exercise: Signal<Exercise>,
  /// Optional callback for when exercise is updated
  #[prop(optional)]
  on_exercise_update: Option<Callback<Exercise>>,
  /// Optional callback when BPM changes
  #[prop(optional)]
  on_bpm_change: Option<Callback<u32>>,
) -> impl IntoView {
  let elapsed_seconds = RwSignal::new(0u64);
  let timer_state = RwSignal::new(TimerState::Stopped);

  view! {
    <div class="p-6 bg-white rounded-lg border border-gray-200">
      <h3 class="mb-4 text-lg font-semibold text-gray-800">"Practice Session"</h3>

      <ConfigurationHeader
        exercise
        on_exercise_update=on_exercise_update.unwrap_or_else(|| Callback::new(|_| {}))
      />

      <div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <TimerSection elapsed_seconds target_time timer_state />

        <MetronomeSection on_bpm_change />
      </div>

      <FretboardSection exercise />
    </div>
  }
}

#[component]
fn TimerSection(
  target_time: Option<Duration>,
  timer_state: RwSignal<TimerState>,
  elapsed_seconds: RwSignal<u64>,
) -> impl IntoView {
  // Check if target time is reached
  let is_target_reached = move || {
    if let Some(target) = target_time {
      elapsed_seconds.get() >= target.as_secs()
    } else {
      false
    }
  };

  // Set up interval for timer ticking
  let Pausable {
    pause,
    resume,
    is_active: _,
  } = use_interval_fn(
    move || {
      if timer_state.get() == TimerState::Running {
        elapsed_seconds.update(|t| *t += 1);
      }
    },
    1000, // 1 second interval
  );

  let pause_clone = pause.clone();

  let stop_timer = {
    move |_| {
      timer_state.set(TimerState::Stopped);
      elapsed_seconds.set(0);
      pause_clone(); // Stop the interval
    }
  };

  let start_timer = {
    move |_| {
      match timer_state.get() {
        TimerState::Stopped => {
          elapsed_seconds.set(0);
          timer_state.set(TimerState::Running);
          resume(); // Start the interval
        }
        TimerState::Paused => {
          timer_state.set(TimerState::Running);
          resume(); // Resume the interval
        }
        TimerState::Running => {
          timer_state.set(TimerState::Paused);
          pause(); // Pause the interval
        }
      }
    }
  };

  // Format elapsed time as MM:SS
  let formatted_time = move || {
    let seconds = elapsed_seconds.get();
    let minutes = seconds / 60;
    let secs = seconds % 60;
    format!("{minutes:02}:{secs:02}")
  };

  view! {
    <div>
      <div class="flex justify-between items-center mb-3">
        <h4 class="font-semibold text-gray-700 text-md">"Timer"</h4>
      </div>

      <div class="p-4 bg-white rounded-lg border border-gray-200">
        <div class="text-center">
          // Timer display
          <div class=move || {
            let base_classes = "text-4xl lg:text-6xl font-mono font-bold mb-4";
            if is_target_reached() {
              format!("{base_classes} text-green-600")
            } else {
              format!("{base_classes} text-gray-800")
            }
          }>{formatted_time}</div>

          // Target time display
          {move || {
            if let Some(target) = target_time {
              let target_mins = target.as_secs() / 60;
              let target_secs = target.as_secs() % 60;
              view! {
                <p class="mb-4 text-sm text-gray-600">
                  "Target: " {format!("{target_mins:02}:{target_secs:02}")}
                  {move || if is_target_reached() { " ✓" } else { "" }}
                </p>
              }
                .into_any()
            } else {
              view! { <div></div> }.into_any()
            }
          }}

          // Control buttons
          <div class="flex justify-center mb-3 space-x-3">
            <button
              class=move || {
                match timer_state.get() {
                  TimerState::Running => {
                    "bg-yellow-500 hover:bg-yellow-600 text-white font-bold py-2 px-4 rounded-lg text-sm"
                  }
                  _ => {
                    "bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded-lg text-sm"
                  }
                }
              }
              on:click=start_timer
            >
              {move || {
                match timer_state.get() {
                  TimerState::Running => "Pause",
                  TimerState::Paused => "Resume",
                  TimerState::Stopped => "Start",
                }
              }}
            </button>

            <button
              class="py-2 px-4 text-sm font-bold text-white bg-red-500 rounded-lg hover:bg-red-600 disabled:bg-gray-400"
              on:click=stop_timer
              disabled=move || timer_state.get() == TimerState::Stopped
            >
              "Stop"
            </button>
          </div>

          // Timer state indicator
          <p class="text-xs text-gray-500">
            {move || {
              match timer_state.get() {
                TimerState::Stopped => "Ready to start",
                TimerState::Running => "Timer running...",
                TimerState::Paused => "Timer paused",
              }
            }}
          </p>
        </div>
      </div>
    </div>
  }
}

#[component]
fn MetronomeSection(on_bpm_change: Option<Callback<u32>>) -> impl IntoView {
  let (bpm, set_bpm) = signal(120u32); // Default 120 BPM
  let (show_metronome, set_show_metronome) = signal(true);
  // Handle BPM changes from metronome
  let bpm_change_callback = Callback::new(move |new_bpm: u32| {
    set_bpm.set(new_bpm);
    // Propagate BPM change to parent component if callback provided
    if let Some(callback) = on_bpm_change {
      callback.run(new_bpm);
    }
  });
  view! {
    <div>
      // Toggle metronome visibility
      <div class="flex justify-between items-center mb-3">
        <h4 class="font-semibold text-gray-700 text-md">"Metronome"</h4>
        <button
          class="py-1 px-2 text-xs bg-gray-200 rounded hover:bg-gray-300"
          on:click=move |_| set_show_metronome.update(|show| *show = !*show)
        >
          {move || if show_metronome.get() { "Hide" } else { "Show" }}
        </button>
      </div>

      {move || {
        if show_metronome.get() {
          view! { <Metronome bpm=bpm on_bpm_change=bpm_change_callback /> }.into_any()
        } else {
          view! {
            <div class="py-8 text-center text-gray-500">
              <p class="text-sm">"Metronome hidden"</p>
              <p class="text-xs">"Current BPM: " {move || bpm.get().to_string()}</p>
            </div>
          }
            .into_any()
        }
      }}
    </div>
  }
}

#[component]
fn FretboardSection(exercise: Signal<Exercise>) -> impl IntoView {
  let (show_fretboard, set_show_fretboard) = signal(true);

  {
    move || {
      match exercise.get().exercise_type {
        ExerciseType::Scale {
          root_note,
          scale_type,
          fret_range,
        }
        | ExerciseType::Triad {
          root_note,
          scale_type,
          fret_range,
        } => {
          view! {
            <div class="mt-6">
              // Toggle fretboard visibility
              <div class="flex justify-between items-center mb-3">
                <h4 class="font-semibold text-gray-700 text-md">"Fretboard"</h4>
                <button
                  class="py-1 px-2 text-xs bg-gray-200 rounded hover:bg-gray-300"
                  on:click=move |_| set_show_fretboard.update(|show| *show = !*show)
                >
                  {move || if show_fretboard.get() { "Hide" } else { "Show" }}
                </button>
              </div>

              {move || {
                if show_fretboard.get() {
                  let fretboard_model = Memo::new(move |_| {
                    let model = FretboardModelBuilder::new()
                      .start_fret_val(fret_range.0 as usize)
                      .end_fret_val(fret_range.1 as usize)
                      .build();
                    let current_scale = Scale::new(root_note, scale_type);
                    model.update_from_scale(current_scale);
                    model
                  });
                  // TODO bad for performance: initialize model once and just update

                  view! {
                    <div class="p-4 bg-gray-50 rounded-lg">
                      <FretboardModelAdapter model=fretboard_model />
                    </div>
                  }
                    .into_any()
                } else {
                  ().into_any()
                }
              }}
            </div>
          }
          .into_any()
        }
        _ => ().into_any(),
      }
    }
  }
}
