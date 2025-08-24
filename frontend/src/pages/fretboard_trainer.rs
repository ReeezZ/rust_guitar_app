use leptos::prelude::*;
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;

use crate::fretboard::fretboard_model::{FretClickEvent, FretboardModel};
// use crate::components::fretboard::trainer::FretboardTrainer;
use crate::fretboard::FretCoord;
use crate::models::fretboard_trainer::FretboardTrainerTrait;
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
  // Initialize fretboard model for note calculations
  let fretboard_model = RwSignal::new(FretboardModel::default());

  // Game state
  let (num_correct, set_num_correct) = signal(0);
  let (num_incorrect, set_num_incorrect) = signal(0);
  let (current_note, set_current_note) = signal(Note::C);
  let (current_interval, set_current_interval) = signal(random_interval());
  let (error_text, set_error_text) = signal("".to_string());

  // Visual state for SVG overlays
  let (reference_note_coord, set_reference_note_coord) = signal(None::<FretCoord>);
  let (reference_note_name, set_reference_note_name) = signal(None::<Note>);
  let (error_coords, set_error_coords) = signal(Vec::<FretCoord>::new());
  let (error_note_names, set_error_note_names) = signal(Vec::<Note>::new());

  // Initialize with first question
  Effect::new(move |_| {
    fretboard_model.with(|model| {
      let random_fret = model.get_random_fret();
      let note = model.note_from_fret(random_fret);
      set_current_note.set(note);
      set_reference_note_coord.set(Some(random_fret));
      set_reference_note_name.set(Some(note));
    });
  });

  // Handle fret clicks
  let on_note_clicked = Callback::new(move |evt: FretClickEvent| {
    fretboard_model.with(|model| {
      let clicked_note = model.note_from_fret(evt.coord);
      let target_note = current_interval.get().of(current_note.get());

      if clicked_note == target_note {
        // Correct answer!
        set_num_correct.update(|n| *n += 1);
        set_current_interval.set(random_interval());
        set_error_text.set("".to_string());

        // Clear error highlights and set new reference note
        set_error_coords.set(vec![]);
        set_error_note_names.set(vec![]);
        let new_fret = model.get_random_fret();
        let new_note = model.note_from_fret(new_fret);
        set_current_note.set(new_note);
        set_reference_note_coord.set(Some(new_fret));
        set_reference_note_name.set(Some(new_note));
      } else {
        // Incorrect answer
        if error_text.get().is_empty() {
          set_error_text.set("Incorrect!".to_string());
        }
        set_num_incorrect.update(|n| *n += 1);

        // Add to error highlights
        set_error_coords.update(|coords| {
          if !coords.contains(&evt.coord) {
            coords.push(evt.coord);
          }
        });
        set_error_note_names.update(|names| {
          // Only add the note name if we added a new coordinate
          let coords = error_coords.get_untracked();
          if coords.len() > names.len() {
            names.push(clicked_note);
          }
        });
      }
    });
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

      // <FretboardTrainer
      // reference_note=reference_note_coord.into()
      // reference_note_name=reference_note_name.into()
      // error_notes=error_coords.into()
      // error_note_names=error_note_names.into()
      // on_note_clicked=on_note_clicked
      // />

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
