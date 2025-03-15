//! The next version of the fretboard viewer.
//! WIP

use leptos::prelude::*;

use crate::components::fretboard::{FretClickEvent, Fretboard};
use crate::models::fretboard_model::{FretState, FretboardModel};
use crate::models::fretboard_trainer::FretboardTrainerTrait;

#[component]
pub fn FretboardNext() -> impl IntoView {
  let fretboard_model = RwSignal::new(FretboardModel::new(
    6,
    12,
    FretboardModel::standard_tuning(),
  ));

  let on_click_random_note = move |_| {
    fretboard_model.with(|model| {
      let random_fret = model.get_random_fret();
      model.set_fret_state(random_fret, FretState::Root);
    });
  };

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
    <div class="flex-row y-4">
      <Fretboard fretboard=fretboard_model on_fret_clicked />
      <button
        class="bg-blue-200 rounded-md border-4 border-slate-500"
        on:click=on_click_random_note
      >
        "Random note"
      </button>
    </div>
  }
}
