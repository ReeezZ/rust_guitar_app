use std::collections::HashMap;

use leptos::{ev, logging::log, prelude::*};

use crate::{
  components::fretboard::{base::Fretboard, with_notes::FretboardWithNotes},
  models::{fretboard_model::FretStateColor, FretCoord, FretState},
};

/// Page for the SVG fretboard with a runtime-adjustable fret count slider.
/// See: https://leptos.dev/docs/reference/signals/
#[component]
pub fn FretboardDevPage() -> impl IntoView {
  // Build a sample set of fret states to visualize different cases
  let mut fret_positions: HashMap<FretCoord, Signal<FretState>> = HashMap::new();

  // Normal notes across several strings/frets
  for (s, f) in [(0, 5), (1, 3), (2, 7), (3, 2)] {
    let sig = RwSignal::new(FretState::Normal);
    fret_positions.insert(
      FretCoord {
        string_idx: s,
        fret_idx: f,
      },
      sig.into(),
    );
  }

  // Colored examples
  fret_positions.insert(
    FretCoord {
      string_idx: 4,
      fret_idx: 8,
    },
    RwSignal::new(FretState::Colored(FretStateColor::Blue)).into(),
  );
  fret_positions.insert(
    FretCoord {
      string_idx: 5,
      fret_idx: 0,
    },
    RwSignal::new(FretState::Colored(FretStateColor::Green)).into(),
  );

  // A hidden example (should not render) - included to ensure Hidden is ignored
  fret_positions.insert(
    FretCoord {
      string_idx: 2,
      fret_idx: 9,
    },
    RwSignal::new(FretState::Hidden).into(),
  );

  let frets = RwSignal::new(fret_positions);

  view! {
    <h1 class="mb-2 text-xl font-bold">"Fretboard Dev: FretboardWithNotes"</h1>
    <p class="mb-4 text-sm text-gray-600">
      Test page showing a variety of FretState values (Normal, Colored, Hidden).
    </p>
    <div>
      <FretboardWithNotes fret_states=frets.into() start_fret=0.into() end_fret=12.into() />

    </div>

    <div>
      <h1 class="mb-2 text-xl font-bold">
        "Fretboard with not callback to check Clickable areas are not rendered"
      </h1>
      <Fretboard fret_states=frets.into() start_fret=0.into() end_fret=12.into() />
    </div>
  }
}
