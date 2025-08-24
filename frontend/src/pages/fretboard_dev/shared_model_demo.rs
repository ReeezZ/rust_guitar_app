use leptos::prelude::*;
use shared::Note;

use crate::{
  components::fretboard::{
    base::Fretboard, visual_config::FretboardVisualConfig, FretState, FretStateColor,
    FretboardViewModel,
  },
  models::fretboard_model::{FretClickEvent, FretCoord, FretboardModel},
  pages::fretboard_dev::{frets_editor::FretsEditor, helper::get_fret_positions},
};

#[component]
pub fn SharedModelDemo() -> impl IntoView {
  let frets = RwSignal::new(get_fret_positions());

  let label = RwSignal::new(String::from("foobar"));
  let color = RwSignal::new(FretStateColor::Green);
  let hidden = RwSignal::new(false);

  let update_fret = Callback::new(move |coord: FretCoord| {
    frets.with(|map| {
      let state = if hidden.get() {
        FretState::Hidden
      } else {
        FretState::Normal(color.get(), label.get())
      };
      if let Some(sig) = map.get(&coord) {
        sig.set(state);
      }
    });
  });

  let handle_note_clicked = Callback::new(move |coord: FretClickEvent| {
    leptos::logging::log!("{:?} {:?}", coord.note, coord.coord);
    update_fret.run(coord.coord);
  });

  let model = RwSignal::new(FretboardModel::new(
    Signal::derive(move || 0),
    Signal::derive(move || 12),
    Signal::derive(move || vec![Note::E, Note::A, Note::D, Note::G, Note::B, Note::E]),
    Signal::derive(move || FretboardVisualConfig::default()),
    Signal::derive(move || Some(handle_note_clicked.clone())),
  ));

  // Update model when demo fret states change (merge into model's internal signals)
  Effect::new(move || {
    model.with(|m| {
      m.set_fret_states(frets.get());
    });
  });

  view! {
    <h1 class="mb-2 text-xl font-bold">"Fretboard Dev: FretboardWithNotes"</h1>
    <p class="mb-4 text-sm text-gray-600">
      Test page showing a variety of FretState values (Normal, Colored, Hidden).
    </p>
    <FretsEditor frets label color hidden />
    <div>
      <Fretboard
        fret_states=Signal::derive(move || frets.get())
        start_fret=0
        end_fret=12
        tuning=RwSignal::new(vec![Note::E, Note::A, Note::D, Note::G, Note::B, Note::E])
        config=FretboardVisualConfig::default()
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
      <FretboardViewModel model=FretboardModel::default() />
    </div>
  }
}
