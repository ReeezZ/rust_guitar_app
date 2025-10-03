use crate::components::exercises::practice_timer::PracticeTimer;
use crate::components::ui::title::{HeadingLevel, Title};
use crate::components::ui::{Button, ButtonVariant};
use crate::music::Scale;
use leptos::prelude::*;
use std::time::Duration;

use super::ConfigurationHeader;
use crate::components::fretboard::FretboardModelAdapter;
use crate::components::metronome::Metronome;
use crate::models::exercise::Exercise;
use crate::models::fretboard::{FretboardModelBuilder, FretboardModelExt};

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
  #[prop(into)]
  exercise: Signal<Exercise>,
  /// Optional callback for when exercise is updated
  // TODO: maybe this should be split differently for exercise types
  #[prop(optional)]
  on_exercise_update: Option<Callback<Exercise>>,
  /// Optional callback when BPM changes
  #[prop(optional)]
  on_bpm_change: Option<Callback<u32>>,
) -> impl IntoView {
  view! {
    <div class="p-6 rounded-lg border border-gray-200 dark:border-gray-800">
      <h3 class="mb-4 text-lg font-semibold">"Practice Session"</h3>

      <ConfigurationHeader
        exercise
        on_exercise_update=on_exercise_update.unwrap_or_else(|| Callback::new(|_| {}))
      />

      <div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <PracticeTimer target_time />

        <MetronomeSection on_bpm_change />
      </div>

      <FretboardSection exercise />
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
        <Title text="Metronome" level=HeadingLevel::H4 />
        <Button
          variant=ButtonVariant::Secondary
          on_click=move || set_show_metronome.update(|show| *show = !*show)
        >
          {move || if show_metronome.get() { "Hide" } else { "Show" }}
        </Button>

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
      match exercise.get() {
        Exercise::Scale(scale) => {
          view! {
            <div class="mt-6">
              // Toggle fretboard visibility
              <div class="flex justify-between items-center mb-3">
                <h4 class="font-semibold text-md">"Fretboard"</h4>
                <Button
                  variant=ButtonVariant::Secondary
                  on_click=move || set_show_fretboard.update(|show| *show = !*show)
                >
                  {move || if show_fretboard.get() { "Hide" } else { "Show" }}
                </Button>

              </div>

              {move || {
                if show_fretboard.get() {
                  let fretboard_model = Memo::new(move |_| {
                    let model = FretboardModelBuilder::new()
                      .start_fret_val(scale.fret_range.0)
                      .end_fret_val(scale.fret_range.1)
                      .build();
                    let current_scale = Scale::new(scale.root_note, scale.scale_type);
                    model.update_from_scale(current_scale);
                    model
                  });
                  // TODO bad for performance: initialize model once and just update

                  view! {
                    <div class="p-4 rounded-lg">
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
