use leptos::prelude::*;
use leptos_use::use_interval_fn;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerState {
  Stopped,
  Running,
  Paused,
}

#[component]
pub fn PracticeTimer(#[prop(optional)] target_time: Option<Duration>) -> impl IntoView {
  let (elapsed_seconds, set_elapsed_seconds) = signal(0u64);
  let (timer_state, set_timer_state) = signal(TimerState::Stopped);

  // Set up interval for timer ticking
  let interval_controls = use_interval_fn(
    move || {
      if timer_state.get() == TimerState::Running {
        set_elapsed_seconds.update(|t| *t += 1);
      }
    },
    1000, // 1 second interval
  );

  // Clone the controls for use in different closures
  let pause_fn = interval_controls.pause.clone();
  let resume_fn = interval_controls.resume.clone();
  let pause_fn2 = interval_controls.pause.clone();

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

  let start_timer = move |_| {
    match timer_state.get() {
      TimerState::Stopped => {
        set_elapsed_seconds.set(0);
        set_timer_state.set(TimerState::Running);
        resume_fn(); // Start the interval
      }
      TimerState::Paused => {
        set_timer_state.set(TimerState::Running);
        resume_fn(); // Resume the interval
      }
      TimerState::Running => {
        set_timer_state.set(TimerState::Paused);
        pause_fn(); // Pause the interval
      }
    }
  };

  let stop_timer = move |_| {
    set_timer_state.set(TimerState::Stopped);
    set_elapsed_seconds.set(0);
    pause_fn2(); // Stop the interval
  };

  view! {
      <div class="bg-white p-6 rounded-lg border border-gray-200">
          <h3 class="text-lg font-semibold text-gray-800 mb-4">"Practice Timer"</h3>

          <div class="text-center">
              // Timer display
              <div class={move || {
                  let base_classes = "text-6xl font-mono font-bold mb-6";
                  if is_target_reached() {
                      format!("{base_classes} text-green-600")
                  } else {
                      format!("{base_classes} text-gray-800")
                  }
              }}>
                  {formatted_time}
              </div>

              // Target time display
              {move || {
                  if let Some(target) = target_time {
                      let target_mins = target.as_secs() / 60;
                      let target_secs = target.as_secs() % 60;
                      view! {
                          <p class="text-sm text-gray-600 mb-4">
                              "Target: " {format!("{target_mins:02}:{target_secs:02}")}
                              {move || if is_target_reached() { " ✓" } else { "" }}
                          </p>
                      }.into_any()
                  } else {
                      view! { <div></div> }.into_any()
                  }
              }}

              // Control buttons
              <div class="flex justify-center space-x-4">
                  <button
                      class={move || {
                          match timer_state.get() {
                              TimerState::Running => "bg-yellow-500 hover:bg-yellow-600 text-white font-bold py-2 px-6 rounded-lg",
                              _ => "bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-6 rounded-lg"
                          }
                      }}
                      on:click=start_timer
                  >
                      {move || {
                          match timer_state.get() {
                              TimerState::Running => "Pause",
                              TimerState::Paused => "Resume",
                              TimerState::Stopped => "Start"
                          }
                      }}
                  </button>

                  <button
                      class="bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-6 rounded-lg disabled:bg-gray-400"
                      on:click=stop_timer
                      disabled={move || timer_state.get() == TimerState::Stopped}
                  >
                      "Stop"
                  </button>
              </div>

              // Timer state indicator
              <p class="text-xs text-gray-500 mt-2">
                  {move || {
                      match timer_state.get() {
                          TimerState::Stopped => "Ready to start",
                          TimerState::Running => "Timer running...",
                          TimerState::Paused => "Timer paused"
                      }
                  }}
              </p>
          </div>
      </div>
  }
}
