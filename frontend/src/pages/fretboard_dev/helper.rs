use leptos::prelude::*;

use crate::fretboard::{
  base_model::FretStateSignals,
  components::base::{FretState, FretStateColor},
  FretCoord,
};

pub(super) fn get_fret_positions() -> FretStateSignals {
  // Build a sample set of fret states to visualize different cases
  let mut fret_positions: FretStateSignals = FretStateSignals::new();

  // Normal notes across several strings/frets
  for (s, f) in [(0, 5), (1, 3), (2, 7), (3, 2)] {
    let sig = RwSignal::new(FretState::Normal(
      FretStateColor::Green,
      format!("{}-{}", f, s),
    ));
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
    RwSignal::new(FretState::Normal(FretStateColor::Blue, "foo".into())).into(),
  );
  fret_positions.insert(
    FretCoord {
      string_idx: 5,
      fret_idx: 0,
    },
    RwSignal::new(FretState::Normal(FretStateColor::Red, "foo".into())).into(),
  );
  fret_positions.insert(
    FretCoord {
      string_idx: 5,
      fret_idx: 4,
    },
    RwSignal::new(FretState::Normal(
      FretStateColor::Red,
      "loooooooong text".into(),
    ))
    .into(),
  );

  // A hidden example (should not render) - included to ensure Hidden is ignored
  fret_positions.insert(
    FretCoord {
      string_idx: 2,
      fret_idx: 9,
    },
    RwSignal::new(FretState::Hidden).into(),
  );

  fret_positions
}

// Local helper to extract value from input/select elements
fn event_target_value(ev: &leptos::ev::Event) -> String {
  use leptos::wasm_bindgen::JsCast;
  if let Some(target) = ev.target() {
    if let Ok(input) = target.clone().dyn_into::<web_sys::HtmlInputElement>() {
      return input.value();
    }
    if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
      return select.value();
    }
  }
  String::new()
}
