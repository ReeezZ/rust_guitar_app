use leptos::prelude::*;
use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;

use crate::components::fretboard::{FretClickEvent, Fretboard};
use crate::models::fretboard_model::{FretState, FretStateColor, FretboardModel};
use crate::models::fretboard_trainer::FretboardTrainerTrait;
use crate::music::intervals::Interval;
use crate::music::notes::Note;

/// Random interval except Unison
fn random_interval() -> Interval {
  Interval::iter()
    .filter(|i| i != &Interval::Unison)
    .choose(&mut rand::rng())
    .unwrap()
}

#[component]
pub fn FretboardTrainer() -> impl IntoView {
  let fretboard_model = RwSignal::new(FretboardModel::new(6, 5, FretboardModel::standard_tuning()));

  let (note, set_note) = signal(Note::C);
  let (interval, set_interval) = signal(random_interval());
  let (error_text, set_error_text) = signal("".to_string());

  let on_fret_clicked = Callback::new(move |evt: FretClickEvent| {
    // TODO prevent the asked noted being valid when looking for octave
    fretboard_model.with(|model| {
      let note_of_clicked_fret = model.note_from_fret(evt.coord);
      if interval.get().of(note.get()) == note_of_clicked_fret {
        set_interval.set(random_interval());
        model.set_all(FretState::Hidden);
        let new_fret = model.get_random_fret();
        model.set_fret_state(new_fret, FretState::Colored(FretStateColor::Green));
        set_note.set(model.note_from_fret(new_fret));
        set_error_text.set("".to_string());
      } else {
        if error_text.get().is_empty() {
          set_error_text.set("Incorrect!".to_string());
        }
        model.set_fret_state(evt.coord, FretState::Colored(FretStateColor::Red));
      }
    });
  });

  fretboard_model.update(move |model| {
    let random_fret = model.get_random_fret();
    set_note.set(model.note_from_fret(random_fret));
    model.set_fret_state(random_fret, FretState::Colored(FretStateColor::Green));
  });

  let interval_str = move || format!(" {} ", interval.get().to_string());
  let note_str = move || format!(" {} ", note.get().to_string());

  view! {
    <div class="flex flex-col space-y-4">
      <div class="flex flex-col items-center space-y-4">
        <h1 class="text-2xl font-bold">"Fretboard trainer"</h1>
        <p>"Train intrevals of notes"</p>
      </div>
      <Fretboard fretboard=fretboard_model on_fret_clicked />
      <div>
        <p>Looking for <b>{interval_str}</b>of <b>{note_str}</b></p>
      </div>
      <div class="overflow-hidden">
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
    </div>
  }
}
