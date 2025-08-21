use std::{collections::HashMap, hash::Hash};

use leptos::prelude::*;

use crate::fretboard::components::{base::FretState, visual_config::FretboardVisualConfig};

pub(crate) type FretStateSignals = HashMap<FretCoord, RwSignal<FretState>>;

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub struct FretCoord {
  pub string_idx: u8,
  pub fret_idx: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct FretClickEvent {
  pub coord: FretCoord,
}

#[derive(Clone, Debug)]
pub struct FretboardBaseModel {
  /// First fret in the active/playable range
  pub start_fret: RwSignal<usize>,
  /// Last fret in the active/playable range
  pub end_fret: RwSignal<usize>,
  /// Number of guitar strings (default: 6)
  pub num_strings: RwSignal<u8>,
  /// Visual configuration for fretboard display properties
  pub config: RwSignal<FretboardVisualConfig>,
  /// Optional callback for fret click events
  pub on_fret_clicked: RwSignal<Option<Callback<FretClickEvent>>>,
  /// States for each fret
  pub fret_states: RwSignal<FretStateSignals>,
}

impl FretboardBaseModel {
  pub fn new(
    start_fret: RwSignal<usize>,
    end_fret: RwSignal<usize>,
    num_strings: RwSignal<u8>,
    config: RwSignal<FretboardVisualConfig>,
    on_fret_clicked: RwSignal<Option<Callback<FretClickEvent>>>,
    fret_states: RwSignal<FretStateSignals>,
  ) -> Self {
    assert!(
      start_fret.get() < end_fret.get(),
      "Start fret must be less than or equal to end fret"
    );
    assert!(
      end_fret.get() <= 24,
      "End fret must be within a reasonable range (0-24)"
    );
    assert!(num_strings.get() > 0, "Number of strings must be positive");

    Self {
      start_fret,
      end_fret,
      num_strings,
      config,
      on_fret_clicked,
      fret_states,
    }
  }

  // fn generate_frets(num_strings: u8, num_frets: u8) -> FretStateSignals {
  //   let mut frets: FretStateSignals = HashMap::with_capacity(num_strings as usize);

  //   for string_idx in 0..num_strings {
  //     let mut string_frets = Vec::with_capacity(num_frets as usize);
  //     for fret_idx in 0..num_frets {
  //       string_frets.push(RwSignal::new(FretState::Hidden));
  //       frets.insert(
  //         FretCoord {
  //           string_idx,
  //           fret_idx,
  //         },
  //         RwSignal::new(FretState::Hidden),
  //       );
  //     }
  //   }

  //   frets
  // }

  // Creates non interactive fretboard
  pub fn from_defaults() -> Self {
    Self {
      start_fret: RwSignal::new(1),
      end_fret: RwSignal::new(9),
      num_strings: RwSignal::new(6),
      config: RwSignal::new(FretboardVisualConfig::default()),
      on_fret_clicked: RwSignal::new(None),
      fret_states: RwSignal::new(HashMap::new()),
    }
  }

  pub fn set_start_fret(&self, fret: usize) {
    self.start_fret.set(fret);
  }

  pub fn set_end_fret(&self, fret: usize) {
    self.end_fret.set(fret);
  }

  pub fn set_num_strings(&self, num: u8) {
    self.num_strings.set(num);
  }

  pub fn set_config(&self, config: FretboardVisualConfig) {
    self.config.set(config);
  }

  pub fn set_fret_states(&self, states: FretStateSignals) {
    self.fret_states.set(states);
  }
}
