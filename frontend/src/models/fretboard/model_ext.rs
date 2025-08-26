use leptos::prelude::*;
use shared::Scale;

use crate::{
  components::fretboard::{FretCoord, FretState, FretStateColor},
  models::fretboard::model::FretboardModel,
};

pub trait FretboardModelExt {
  fn update_from_scale(&self, scale: Scale);
  fn get_random_fret(&self) -> FretCoord;
}

impl FretboardModelExt for FretboardModel {
  fn update_from_scale(&self, scale: Scale) {
    self
      .tuning
      .get()
      .iter()
      .enumerate()
      .for_each(|(string_idx, string_note)| {
        for fret_idx in self.get_min_fret().get()..=self.get_max_visible_fret().get() {
          let coord = FretCoord {
            string_idx: string_idx as u8,
            fret_idx: fret_idx as u8,
          };
          let state = if fret_idx >= self.start_fret.get() && fret_idx <= self.end_fret.get() {
            let note_at_fret = string_note.add_steps(fret_idx);
            let state = if scale.root_note() == Some(note_at_fret) {
              FretState::Normal(FretStateColor::Green, note_at_fret.to_string())
            } else if scale.contains_note(note_at_fret) {
              FretState::Normal(FretStateColor::Blue, note_at_fret.to_string())
            } else {
              FretState::Hidden
            };
            state
          } else {
            FretState::Hidden
          };
          self.fret_states.with(|fret_states| {
            if let Some(sig) = fret_states.get(&coord) {
              sig.set(state);
            }
          });
        }
      });
  }

  /// Get a random fret within the active range
  fn get_random_fret(&self) -> FretCoord {
    use rand::Rng;
    let mut rng = rand::rng();

    let start = self.start_fret.get_untracked();
    let end = self.end_fret.get_untracked();
    let num_strings = self.tuning.get_untracked().len();

    FretCoord {
      string_idx: rng.random_range(0..num_strings) as u8,
      fret_idx: rng.random_range(start..=end) as u8,
    }
  }
}
