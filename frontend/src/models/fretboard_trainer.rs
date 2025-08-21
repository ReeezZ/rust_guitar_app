use shared::music::{intervals::Interval, notes::Note};

use leptos::prelude::{Get, GetUntracked};
use rand::{seq::IteratorRandom, Rng};
use strum::IntoEnumIterator;

use crate::fretboard::{base_model::FretCoord, with_notes_model::FretboardWithNotesModel};

pub trait FretboardTrainerTrait {
  fn note_from_fret(&self, coord: FretCoord) -> Note;
  fn is_interval_of(
    &self,
    coord_left: FretCoord,
    coord_right: FretCoord,
    interval: Interval,
  ) -> bool;
  fn get_random_fret(&self) -> FretCoord;
}

impl FretboardTrainerTrait for FretboardWithNotesModel {
  fn get_random_fret(&self) -> FretCoord {
    let string_idx = rand::rng().random_range(0..self.num_strings.get());
    let fret_idx = rand::rng().random_range(0..self.get_num_frets());

    FretCoord {
      string_idx,
      fret_idx,
    }
  }

  fn is_interval_of(
    &self,
    coord_left: FretCoord,
    coord_right: FretCoord,
    interval: Interval,
  ) -> bool {
    let note_left = self.note_from_fret(coord_left);
    let note_right = self.note_from_fret(coord_right);
    let left_note_plus_interval = interval.of(note_left);

    note_right == left_note_plus_interval
  }

  fn note_from_fret(&self, coord: FretCoord) -> Note {
    let string_note = self.tuning[coord.string_idx as usize];
    string_note.add_steps(coord.fret_idx as usize)
  }
}
pub fn get_random_interval() -> Interval {
  Interval::iter()
    .choose(&mut rand::rng())
    .expect("Failed to get random interval")
}

// tests
#[cfg(test)]
mod tests {

  #[test]
  fn test_fretboard_trainer_trait() {}
}
