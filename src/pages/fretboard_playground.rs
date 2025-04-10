//! This page is for testing features and ideas

use leptos::prelude::*;

use crate::components::fretboard::{FretClickEvent, Fretboard};
use crate::models::fretboard_model::{FretState, FretStateColor, FretboardModel};
use crate::models::fretboard_trainer::FretboardTrainerTrait;

#[component]
pub fn FretboardPlayground() -> impl IntoView {
  let fretboard_model = RwSignal::new(FretboardModel::new(
    6,
    12,
    FretboardModel::standard_tuning(),
  ));

  let on_click_random_note = move |_| {
    fretboard_model.with(|model| {
      let random_fret = model.get_random_fret();
      model.set_fret_state(random_fret, FretState::Colored(FretStateColor::Red));
    });
  };

  let on_fret_clicked = Callback::new(move |evt: FretClickEvent| {
    let toggle_fret_state = match evt.fret_state {
      FretState::Hidden => FretState::Normal,
      FretState::Normal => FretState::Hidden,
      FretState::Colored(_) => FretState::Hidden,
    };

    fretboard_model
      .get()
      .set_fret_state(evt.coord, toggle_fret_state);
  });

  view! {
    <div class="flex flex-col space-y-4">
      <div class="flex flex-col items-center space-y-4">
        <h1 class="text-2xl font-bold">"Fretboard Playground"</h1>
        <p>"This page is for testing features and ideas"</p>
      </div>
      <Fretboard fretboard=fretboard_model on_fret_clicked />
      <div class="flex flex-row justify-center space-x-4">
        <button
          class="py-2 px-4 bg-blue-200 rounded-md border-4 border-slate-500"
          on:click=on_click_random_note
        >
          "Random note"
        </button>
        <button
          class="py-2 px-4 bg-blue-200 rounded-md border-4 border-slate-500"
          on:click=move |_| {
            fretboard_model.with(|model| model.set_all(FretState::Hidden));
          }
        >
          "Hide all"
        </button>
      </div>
    </div>
  }
}
