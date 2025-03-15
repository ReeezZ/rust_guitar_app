use crate::music::{intervals::Interval, notes::Note};

use super::fretboard_model::{FretCoord, FretboardModel};
use leptos::prelude::Get;

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

impl FretboardTrainerTrait for FretboardModel {
  fn get_random_fret(&self) -> FretCoord {
    // TODO https://gitlab.com/ReeeZ/leptos_stuff/-/issues/11
    unimplemented!();
    // let string_idx = rand::rng().random_range(0..self.get_num_strings() as u8);
    // let fret_idx = rand::rng().random_range(0..self.get_num_frets().get() as u8);

    // FretCoord {
    //   string_idx,
    //   fret_idx,
    // }
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
    let string_note = self.get_tuning()[coord.string_idx as usize];
    string_note.add_steps(coord.fret_idx as usize)
  }
}

// tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fretboard_trainer_trait() {}
}
