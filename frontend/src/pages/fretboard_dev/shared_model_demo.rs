use leptos::prelude::*;

use crate::{
  fretboard::{
    base_model::{FretClickEvent, FretboardBaseModel},
    components::{
      base::{FretState, FretStateColor, FretboardViewModel},
      visual_config::FretboardVisualConfig,
      with_notes::{FretClickEventWithNote, FretboardWithNotes},
    },
    FretCoord,
  },
  pages::fretboard_dev::{frets_editor::FretsEditor, helper::get_fret_positions},
};

#[component]
pub fn SharedModelDemo() -> impl IntoView {
  let model = RwSignal::new(FretboardBaseModel::from_defaults());

  let frets = RwSignal::new(get_fret_positions());

  let label = RwSignal::new(String::from("foobar"));
  let color = RwSignal::new(FretStateColor::Green);
  let hidden = RwSignal::new(false);

  let update_fret = Callback::new(move |coord: FretCoord| {
    frets.update(|map| {
      let state = if hidden.get() {
        FretState::Hidden
      } else {
        FretState::Normal(color.get(), label.get())
      };
      map.insert(coord, RwSignal::new(state).into());
    });
  });

  let handle_note_clicked = Callback::new(move |coord: FretClickEventWithNote| {
    leptos::logging::log!("{:?} {:?}", coord.note, coord.coord);
    update_fret.run(coord.coord);
  });

  let handle_fret_clicked = Callback::new(move |coord: FretClickEvent| {
    leptos::logging::log!("{:?}", coord);
    update_fret.run(coord.coord);
  });

  model.update(|m| {
    m.on_fret_clicked.set(Some(handle_fret_clicked));
  });

  Effect::new(move || {
    // Ensure the model is updated with the current fret states
    model.update(|m| {
      m.fret_states.set(frets.get());
    });
  });

  view! {
    <h1 class="mb-2 text-xl font-bold">"Fretboard Dev: FretboardWithNotes"</h1>
    <p class="mb-4 text-sm text-gray-600">
      Test page showing a variety of FretState values (Normal, Colored, Hidden).
    </p>
    <FretsEditor frets label color hidden />
    <div>
      <FretboardWithNotes
        fret_states=frets
        start_fret=0
        end_fret=12
        num_strings=6
        config=Signal::derive(FretboardVisualConfig::default)
        on_note_clicked=handle_note_clicked
      />

    </div>

    <div>
      <h1 class="mb-2 text-xl font-bold">"Fretboard (base) from model"</h1>
      <FretboardViewModel model=model />
    </div>

    <div>
      <h1 class="mb-2 text-xl font-bold">
        "Fretboard (base) with no callback to check Clickable areas are not rendered"
      </h1>
      <FretboardViewModel model=RwSignal::new(FretboardBaseModel::from_defaults()) />
    </div>
  }
}
