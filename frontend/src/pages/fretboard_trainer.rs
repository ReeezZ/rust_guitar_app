use leptos::prelude::*;
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;

use crate::components::fretboard::{
  FretClickEvent, FretCoord, FretState, FretStateColor, FretboardModelAdapter,
};
use crate::models::fretboard::model::FretboardModel;
use crate::models::fretboard::FretboardModelExt;
use shared::music::intervals::Interval;
use shared::music::notes::Note;

/// Random interval except Unison
fn random_interval() -> Interval {
  Interval::iter()
    .filter(|i| i != &Interval::Unison)
    .choose(&mut rand::rng())
    .unwrap()
}

/// Fretboard trainer page for interval training using SVG components.
///
/// This trainer helps users learn intervals by showing a reference note and asking
/// them to find the specified interval. Uses the modern SVG overlay approach for
/// clean separation between game logic and visual presentation.
#[component]
pub fn FretboardTrainerPage() -> impl IntoView {
  // Initialize fretboard model for note calculations (pure data, no callbacks)
  let fretboard_model = RwSignal::new(FretboardModel::default());

  // Game state
  let (num_correct, set_num_correct) = signal(0);
  let (num_incorrect, set_num_incorrect) = signal(0);
  let (current_note, set_current_note) = signal(Note::C);
  let (current_interval, set_current_interval) = signal(random_interval());
  let (error_text, set_error_text) = signal("".to_string());

  // Visual state for SVG overlays
  let (reference_note_coord, set_reference_note_coord) = signal(None::<FretCoord>);
  let (error_coords, set_error_coords) = signal(Vec::<FretCoord>::new());

  // Handle fret clicks - this is pure UI logic, not mixed with data model
  let on_note_clicked = Callback::new(move |evt: FretClickEvent| {
    fretboard_model.with_untracked(move |model| {
      let clicked_note = model.note_from_fret(evt.coord);
      let target_note = current_interval
        .get_untracked()
        .of(current_note.get_untracked());

      leptos::logging::log!(
        "coord {:?} - target note: {:?} - clicked note: {} - expected interval: {}, got interval: {:?}",
        evt.coord,
        target_note,
        clicked_note,
        current_interval.get_untracked(),
        Interval::from_notes(current_note.get_untracked(), clicked_note)
      );

      if clicked_note == target_note {
        // Correct answer!
        set_num_correct.update(|n| *n += 1);
        set_current_interval.set(random_interval());
        set_error_text.set("".to_string());

        // Clear error highlights and set new reference note
        set_error_coords.set(vec![]);
        let new_fret = model.get_random_fret();
        let new_note = model.note_from_fret(new_fret);
        set_current_note.set(new_note);
        model.hide_all_frets();
        model.set_fret_state(
          evt.coord,
          FretState::Normal(FretStateColor::Green, new_note.to_string()),
        );
      } else {
        // Incorrect answer
        if error_text.get_untracked().is_empty() {
          set_error_text.set("Incorrect!".to_string());
        }
        set_num_incorrect.update(|n| *n += 1);

        if !error_coords.get_untracked().contains(&evt.coord) {
          // Add to error highlights
          set_error_coords.update(|coords| {
            coords.push(evt.coord);
          });
        }

        model.set_fret_state(
          evt.coord,
          FretState::Normal(FretStateColor::Red, clicked_note.to_string()),
        );
      }
    });
  });

  // Initialize the first question
  fretboard_model.with_untracked(move |model| {
    let random_fret = model.get_random_fret();
    let note = model.note_from_fret(random_fret);
    set_current_note.set(note);
    set_reference_note_coord.set(Some(random_fret));
    set_error_coords.set(vec![]);
    leptos::logging::log!("Initial note: {} at {:?}", note, random_fret);
    model.set_fret_state(
      random_fret,
      FretState::Normal(FretStateColor::Green, note.to_string()),
    );
  });

  // Computed strings for display
  let interval_str = move || current_interval.get().to_string();
  let note_str = move || current_note.get().to_string();
  let success_rate = move || {
    let correct = num_correct.get();
    let incorrect = num_incorrect.get();
    let total = correct + incorrect;
    if total > 0 {
      (correct as f32 / total as f32 * 100.0).round() as u32
    } else {
      0
    }
  };

  view! {
    <div class="flex flex-col space-y-4">
      <div class="flex flex-col items-center space-y-4">
        <h1 class="text-2xl font-bold">"Fretboard Trainer"</h1>
        <p>"Train intervals of notes"</p>
      </div>

      <FretboardModelAdapter model=fretboard_model on_note_clicked=on_note_clicked />

      <div class="text-center">
        <p class="text-lg">
          "Looking for " <b>{move || format!("{} ", interval_str())}</b> "of "
          <b>{move || format!("{} ", note_str())}</b>
        </p>
      </div>

      <div class="flex flex-col items-center space-y-2">
        <div class="grid grid-cols-2 gap-4 text-center">
          <div>
            <p class="font-semibold text-green-600">
              {move || format!("Correct: {}", num_correct.get())}
            </p>
          </div>
          <div>
            <p class="font-semibold text-red-600">
              {move || format!("Incorrect: {}", num_incorrect.get())}
            </p>
          </div>
        </div>

        <p class="text-sm text-gray-600">
          {move || format!("Total answers: {}", num_correct.get() + num_incorrect.get())}
        </p>

        <p class="font-semibold">{move || format!("Success rate: {}%", success_rate())}</p>
      </div>

      <div class="flex overflow-hidden justify-center min-h-[2rem]">
        {move || {
          if !error_text.get().is_empty() {
            Some(
              view! {
                <p class="text-center text-red-600 animate-shake animate-thrice animate-duration-[160ms] animate-ease-linear">
                  {error_text}
                </p>
              },
            )
          } else {
            None
          }
        }}
      </div>

      <div class="text-center">
        <p class="text-sm text-gray-500">"Click on the fretboard to answer"</p>
      </div>
    </div>
  }
}
