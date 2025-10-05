use crate::components::exercises::practice_timer::{PracticeTimer, TimerState};
use crate::components::ui::title::{HeadingLevel, Title};
use crate::components::ui::{Button, ButtonVariant};
use crate::music::{Note, Scale, ScaleType};
use leptos::prelude::*;
use std::time::Duration;

use super::ConfigurationHeader;
use crate::components::fretboard::FretboardModelAdapter;
use crate::components::metronome::Metronome;
use crate::models::fretboard::{FretboardModelBuilder, FretboardModelExt};

#[component]
pub fn ScalePracticeSession(
  #[prop(into)] root_note: Signal<Note>,
  on_root_note_changed: Callback<Note>,
  #[prop(into)] scale_type: Signal<ScaleType>,
  on_scale_type_changed: Callback<ScaleType>,
  #[prop(into)] min_fret: Signal<u8>,
  on_min_fret_changed: Callback<u8>,
  #[prop(into)] max_fret: Signal<u8>,
  on_max_fret_changed: Callback<u8>,
  #[prop(optional)] target_time: Option<Duration>,
  #[prop(optional)] on_bpm_change: Option<Callback<u32>>,
) -> impl IntoView {
  let can_edit = RwSignal::new(true);
  view! {
    <div class="p-6 rounded-lg border border-gray-200 dark:border-gray-800">
      <h3 class="mb-4 text-lg font-semibold">"Practice Session"</h3>

      <ConfigurationHeader
        root_note
        scale_type
        min_fret
        max_fret
        can_edit
        on_root_note_changed
        on_scale_type_changed
        on_min_fret_changed
        on_max_fret_changed
      />

      <div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
        <PracticeTimer
          target_time
          on_timer_state_changed=Callback::new(move |timer_state| match timer_state {
            TimerState::Stopped => can_edit.set(true),
            TimerState::Running => can_edit.set(false),
            TimerState::Paused => can_edit.set(false),
          })
        />

        <MetronomeSection on_bpm_change />
      </div>

      <FretboardSection root_note scale_type min_fret max_fret />
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
fn FretboardSection(
  #[prop(into)] root_note: Signal<Note>,
  #[prop(into)] scale_type: Signal<ScaleType>,
  #[prop(into)] min_fret: Signal<u8>,
  #[prop(into)] max_fret: Signal<u8>,
) -> impl IntoView {
  let (show_fretboard, set_show_fretboard) = signal(true);

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
          let current_scale = Scale::new(root_note.get(), scale_type.get());
          let fretboard_model = Memo::new(move |_| {
            let model = FretboardModelBuilder::new()
              .start_fret_val(min_fret.get())
              .end_fret_val(max_fret.get())
              .build();
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
}
