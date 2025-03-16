use leptos::prelude::*;

use crate::components::fretboard::{FretClickEvent, Fretboard};
use crate::models::fretboard_model::{FretState, FretboardModel};
use crate::models::fretboard_trainer::FretboardTrainerTrait;
use crate::music::intervals::Interval;
use crate::music::notes::Note;

#[component]
pub fn FretboardTrainer() -> impl IntoView {
  let fretboard_model = RwSignal::new(FretboardModel::new(
    6,
    12,
    FretboardModel::standard_tuning(),
  ));

  let (note, set_root_note) = signal(Note::C);
  let (interval, set_interval) = signal(Interval::MajorSecond);

  let on_fret_clicked = Callback::new(move |evt: FretClickEvent| {
    let toggle_fret_state = match evt.fret_state {
      FretState::Hidden => FretState::Normal,
      FretState::Normal => FretState::Hidden,
      FretState::Root => FretState::Hidden,
    };

    fretboard_model
      .get()
      .set_fret_state(evt.coord, toggle_fret_state);
  });

  view! {
    <div class="flex flex-col space-y-4">
      <div class="flex flex-col items-center space-y-4">
        <h1 class="text-2xl font-bold">"Fretboard trainer"</h1>
        <p>"Train intrevals of notes"</p>
      </div>
      <Fretboard fretboard=fretboard_model on_fret_clicked />
      <div>
        // <label>Looking for {move || interval.get()}of {note.get()}</label>
      </div>
    </div>
  }
}
