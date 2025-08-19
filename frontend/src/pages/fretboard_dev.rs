use std::collections::HashMap;

use leptos::{ev, logging::log, prelude::*};

use crate::{
  components::fretboard::with_notes::{FretClickEventWithNote, FretboardWithNotes},
  models::{FretCoord, FretState},
};

/// Page for the SVG fretboard with a runtime-adjustable fret count slider.
/// See: https://leptos.dev/docs/reference/signals/
#[component]
pub fn FretboardDevPage() -> impl IntoView {
  let foo = RwSignal::new(FretState::Normal);

  let mut fret_positions: HashMap<FretCoord, Signal<FretState>> = HashMap::new();
  fret_positions.insert(
    FretCoord {
      string_idx: 0,
      fret_idx: 5,
    },
    foo.into(),
  );
  let frets = RwSignal::new(fret_positions);

  view! {
    <h1>"FretboardWithNotes"</h1>
    <div>
      <FretboardWithNotes fret_states=frets.into() start_fret=0.into() end_fret=12.into() />

    </div>
  }
}
