use leptos::prelude::*;
use leptos_use::use_interval_fn;
use std::time::Duration;

use crate::components::fretboard::scale_display::FretboardScaleDisplay;
use crate::components::metronome::Metronome;
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
  #[prop(optional)]
  exercise: Option<Exercise>,
  /// Optional callback when BPM changes
  #[prop(optional)]
  on_bpm_change: Option<Callback<u32>>,
) -> impl IntoView {
  let (elapsed_seconds, set_elapsed_seconds) = signal(0u64);
  let (timer_state, set_timer_state) = signal(TimerState::Stopped);
  let (bpm, set_bpm) = signal(120u32); // Default 120 BPM
  let (show_metronome, set_show_metronome) = signal(true);
  let (show_fretboard, set_show_fretboard) = signal(true);

  // Clone exercise for closures to avoid move issues
  let exercise_clone = exercise.clone();
  let exercise_clone2 = exercise.clone();

  // Set up interval for timer ticking
  let interval_controls = use_interval_fn(
    move || {
      if timer_state.get() == TimerState::Running {
        set_elapsed_seconds.update(|t| *t += 1);
      }
    },
    1000, // 1 second interval
  );

  // Format elapsed time as MM:SS
  let formatted_time = move || {
    let seconds = elapsed_seconds.get();
    let minutes = seconds / 60;
    let secs = seconds % 60;
    format!("{minutes:02}:{secs:02}")
  };

  // Check if target time is reached
  let is_target_reached = move || {
    if let Some(target) = target_time {
      elapsed_seconds.get() >= target.as_secs()
    } else {
      false
    }
  };

  let start_timer = {
    let pause = interval_controls.pause.clone();
    let resume = interval_controls.resume.clone();
    move |_| {
      match timer_state.get() {
        TimerState::Stopped => {
          set_elapsed_seconds.set(0);
          set_timer_state.set(TimerState::Running);
          resume(); // Start the interval
        }
        TimerState::Paused => {
          set_timer_state.set(TimerState::Running);
          resume(); // Resume the interval
        }
        TimerState::Running => {
          set_timer_state.set(TimerState::Paused);
          pause(); // Pause the interval
        }
      }
    }
  };

  let stop_timer = {
    let pause = interval_controls.pause.clone();
    move |_| {
      set_timer_state.set(TimerState::Stopped);
      set_elapsed_seconds.set(0);
      pause(); // Stop the interval
    }
  };

  // Handle BPM changes from metronome
  let bpm_change_callback = Callback::new(move |new_bpm: u32| {
    set_bpm.set(new_bpm);
    // Propagate BPM change to parent component if callback provided
    if let Some(callback) = on_bpm_change {
      callback.run(new_bpm);
    }
  });

  view! {
    <div class="p-6 bg-white rounded-lg border border-gray-200">
      <h3 class="mb-4 text-lg font-semibold text-gray-800">"Practice Session"</h3>

      // Exercise info section - compact display
      {move || {
        if let Some(ref ex) = exercise_clone {
          view! {
            <div class="p-3 mb-6 bg-gray-50 rounded-lg">
              <div class="flex flex-wrap gap-4 items-center text-sm">
                <div class="flex gap-2 items-center">
                  <span class="font-medium text-gray-700">"Type:"</span>
                  <span class="py-1 px-2 text-xs font-medium text-blue-800 bg-blue-100 rounded">
                    {ex.exercise_type.type_name()}
                  </span>
                </div>

                // Split root note and scale type into separate boxes
                {match &ex.exercise_type {
                  ExerciseType::Scale { root_note, scale_type, .. }
                  | ExerciseType::Triad { root_note, scale_type, .. } => {
                    view! {
                      <>
                        <div class="flex gap-2 items-center">
                          <span class="font-medium text-gray-700">"Root:"</span>
                          <span class="py-1 px-2 text-xs font-medium text-indigo-800 bg-indigo-100 rounded">
                            {root_note.to_string()}
                          </span>
                        </div>
                        <div class="flex gap-2 items-center">
                          <span class="font-medium text-gray-700">"Scale:"</span>
                          <span class="py-1 px-2 text-xs font-medium text-purple-800 bg-purple-100 rounded">
                            {scale_type.to_string()}
                          </span>
                        </div>
                      </>
                    }
                      .into_any()
                  }
                  _ => view! { <div></div> }.into_any(),
                }}

                {ex
                  .exercise_type
                  .get_fret_range()
                  .map(|(min, max)| {
                    view! {
                      <div class="flex gap-2 items-center">
                        <span class="font-medium text-gray-700">"Frets:"</span>
                        <span class="py-1 px-2 text-xs font-medium text-orange-800 bg-orange-100 rounded">
                          {format!("{min}-{max}")}
                        </span>
                      </div>
                    }
                  })}

                <div class="flex gap-2 items-center">
                  <span class="font-medium text-gray-700">"Details:"</span>
                  <span class="text-xs text-gray-600">{ex.exercise_type.to_string()}</span>
                </div>
              </div>
            </div>
          }
            .into_any()
        } else {
          view! { <div></div> }.into_any()
        }
      }}

      <div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
        // Timer Section
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

        // Metronome Section
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
      </div>

      // Fretboard Section (only show if exercise has fretboard content)
      {move || {
        if let Some(ref ex) = exercise_clone2 {
          match &ex.exercise_type {
            ExerciseType::Scale { root_note, scale_type, fret_range }
            | ExerciseType::Triad { root_note, scale_type, fret_range } => {
              let root_note = *root_note;
              let scale_type = *scale_type;
              let fret_range = *fret_range;

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
                      view! {
                        <div class="p-4 bg-gray-50 rounded-lg">
                          <FretboardScaleDisplay
                            fret_range=Signal::derive(move || {
                              fret_range.0 as usize..=fret_range.1 as usize
                            })
                            root_note=Signal::derive(move || root_note)
                            scale_type=Signal::derive(move || scale_type)
                          />
                        </div>
                      }
                        .into_any()
                    } else {
                      view! {
                        <div class="py-8 text-center text-gray-500">
                          <p class="text-sm">"Fretboard hidden"</p>
                          <p class="text-xs">
                            {format!(
                              "{} {} (frets {}-{})",
                              root_note,
                              scale_type,
                              fret_range.0,
                              fret_range.1,
                            )}
                          </p>
                        </div>
                      }
                        .into_any()
                    }
                  }}
                </div>
              }
                .into_any()
            }
            _ => view! { <div></div> }.into_any(),
          }
        } else {
          view! { <div></div> }.into_any()
        }
      }}

      // Session info
      <div class="pt-4 mt-4 text-center border-t border-gray-200">
        <p class="text-xs text-gray-500">
          "Session BPM: " {move || bpm.get().to_string()} " • Duration: " {formatted_time}
        </p>
      </div>
    </div>
  }
}
