use leptos::prelude::*;

use crate::components::fretboard::base::get_preallocated_fret_states;
use crate::components::fretboard::{FretCoord, FretState, FretStateColor, FretStateSignals};

pub(super) fn get_fret_positions() -> FretStateSignals {
  // Build a sample set of fret states to visualize different cases
  let mut fret_positions: FretStateSignals = get_preallocated_fret_states();

  // Normal notes across several strings/frets
  for (s, f) in [(0, 5), (1, 3), (2, 7), (3, 2)] {
    fret_positions.insert(
      FretCoord {
        string_idx: s,
        fret_idx: f,
      },
      RwSignal::new(FretState::Normal(FretStateColor::Green, format!("{f}-{s}"))),
    );
  }

  // Colored examples
  fret_positions.insert(
    FretCoord {
      string_idx: 4,
      fret_idx: 8,
    },
    RwSignal::new(FretState::Normal(FretStateColor::Blue, "foo".into())),
  );
  fret_positions.insert(
    FretCoord {
      string_idx: 5,
      fret_idx: 0,
    },
    RwSignal::new(FretState::Normal(FretStateColor::Red, "foo".into())),
  );
  fret_positions.insert(
    FretCoord {
      string_idx: 5,
      fret_idx: 4,
    },
    RwSignal::new(FretState::Normal(
      FretStateColor::Red,
      "loooooooong text".into(),
    )),
  );

  // A hidden example (should not render) - included to ensure Hidden is ignored
  fret_positions.insert(
    FretCoord {
      string_idx: 2,
      fret_idx: 9,
    },
    RwSignal::new(FretState::Hidden),
  );

  fret_positions
}
