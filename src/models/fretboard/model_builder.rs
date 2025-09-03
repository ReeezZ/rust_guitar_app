use crate::music::Note;
use leptos::prelude::{RwSignal, Signal};

use crate::{
  components::fretboard::{
    base::get_preallocated_fret_states, FretStateSignals, FretboardVisualConfig,
  },
  models::fretboard::model::FretboardModel,
};

pub fn default_tuning() -> Signal<Vec<Note>> {
  Signal::derive(move || vec![Note::E, Note::B, Note::G, Note::D, Note::A, Note::E])
}

pub struct FretboardModelBuilder {
  start_fret: Option<Signal<usize>>,
  end_fret: Option<Signal<usize>>,
  tuning: Option<Signal<Vec<Note>>>,
  config: Option<Signal<FretboardVisualConfig>>,
  fret_states: Option<Signal<FretStateSignals>>,
}

impl FretboardModelBuilder {
  pub fn new() -> Self {
    Self {
      start_fret: None,
      end_fret: None,
      tuning: None,
      config: None,
      fret_states: None,
    }
  }

  pub fn start_fret(mut self, start_fret: Signal<usize>) -> Self {
    self.start_fret = Some(start_fret);
    self
  }
  pub fn start_fret_val(mut self, start_fret: usize) -> Self {
    self.start_fret = Some(Signal::derive(move || start_fret));
    self
  }

  pub fn end_fret(mut self, end_fret: Signal<usize>) -> Self {
    self.end_fret = Some(end_fret);
    self
  }
  pub fn end_fret_val(mut self, end_fret: usize) -> Self {
    self.end_fret = Some(Signal::derive(move || end_fret));
    self
  }

  pub fn tuning(mut self, tuning: Signal<Vec<Note>>) -> Self {
    self.tuning = Some(tuning);
    self
  }

  pub fn config(mut self, config: Signal<FretboardVisualConfig>) -> Self {
    self.config = Some(config);
    self
  }

  pub fn fret_states(mut self, fret_states: Signal<FretStateSignals>) -> Self {
    self.fret_states = Some(fret_states);
    self
  }

  pub fn from_options(
    start_fret: Option<Signal<usize>>,
    end_fret: Option<Signal<usize>>,
    tuning: Option<Signal<Vec<Note>>>,
    config: Option<Signal<FretboardVisualConfig>>,
    fret_states: Option<Signal<FretStateSignals>>,
  ) -> FretboardModel {
    let fret_states =
      fret_states.unwrap_or_else(|| RwSignal::new(get_preallocated_fret_states()).into());
    FretboardModel::new(
      start_fret.unwrap_or_else(|| Signal::derive(move || 0)),
      end_fret.unwrap_or_else(|| Signal::derive(move || 12)),
      tuning.unwrap_or_else(default_tuning),
      config.unwrap_or_else(|| Signal::derive(FretboardVisualConfig::default)),
      fret_states,
    )
  }

  pub fn build(self) -> FretboardModel {
    Self::from_options(
      self.start_fret,
      self.end_fret,
      self.tuning,
      self.config,
      self.fret_states,
    )
  }
}

impl Default for FretboardModelBuilder {
  fn default() -> Self {
    Self::new()
  }
}
