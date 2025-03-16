use leptos::logging::log;
use leptos::prelude::*;

use crate::components::fretboard::{FretClickEvent, Fretboard};
use crate::models::fretboard_model::{FretState, FretboardModel};
use crate::models::fretboard_trainer::FretboardTrainerTrait;
use crate::music::intervals::Interval;
use crate::music::notes::Note;

#[component]
pub fn FretboardTrainer() -> impl IntoView {
  let fretboard_model = RwSignal::new(FretboardModel::new(6, 5, FretboardModel::standard_tuning()));

  let (note, set_note) = signal(Note::C);
  let (interval, set_interval) = signal(Interval::MajorSecond);

  let on_fret_clicked = Callback::new(move |evt: FretClickEvent| {
    fretboard_model.with(|model| {
      let note_of_clicked_fret = model.note_from_fret(evt.coord);
      if interval.get().of(note.get()) == note_of_clicked_fret {
        log!("Correct!");
      } else {
        log!("Incorrect!");
      }
    });
  });

  fretboard_model.update(move |model| {
    let random_fret = model.get_random_fret();
    set_note.set(model.note_from_fret(random_fret));
    model.set_fret_state(random_fret, FretState::Root);
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
    </div>
  }
}
