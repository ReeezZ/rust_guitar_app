use leptos::{logging::log, prelude::*};

use shared::music::{
  notes::Note,
  scales::{Scale, ScaleTrait},
};

use crate::fretboard::{
  base_model::{self, FretClickEvent, FretCoord, FretStateSignals, FretboardBaseModel},
  components::{
    base::{FretState, FretStateColor},
    visual_config::FretboardVisualConfig,
    with_notes::FretClickEventWithNote,
  },
};

#[derive(Clone, Debug)]
pub struct FretboardWithNotesModel {
  /// First fret in the active/playable range
  pub start_fret: RwSignal<usize>,
  /// Last fret in the active/playable range
  pub end_fret: RwSignal<usize>,
  /// Number of guitar strings (default: 6)
  pub num_strings: RwSignal<u8>,
  /// Visual configuration for fretboard display properties
  pub config: RwSignal<FretboardVisualConfig>,
  /// Optional callback for fret click events
  pub on_note_clicked: Signal<Option<Callback<FretClickEventWithNote>>>,
  /// States for each fret
  pub fret_states: RwSignal<FretStateSignals>,

  pub tuning: RwSignal<Vec<Note>>,
}

impl Default for FretboardWithNotesModel {
  fn default() -> Self {
    let base_model = FretboardBaseModel::from_defaults();
    Self::from_fretboard_base_model(base_model, RwSignal::new(Self::standard_tuning()))
  }
}

impl FretboardWithNotesModel {
  pub fn from_fretboard_base_model(
    base_model: FretboardBaseModel,
    tuning: RwSignal<Vec<Note>>,
  ) -> Self {
    let on_note_clicked = Signal::derive(move || {
      if let Some(callback) = base_model.on_fret_clicked.get() {
        let on_note_clicked = Callback::new(move |event: FretClickEventWithNote| {
          let coord = event.coord;
          let event = FretClickEvent { coord };
          callback.run(event);
        });
        Some(on_note_clicked)
      } else {
        None
      }
    });

    Self {
      start_fret: base_model.start_fret,
      end_fret: base_model.end_fret,
      num_strings: base_model.num_strings,
      config: base_model.config,
      on_note_clicked: on_note_clicked,
      fret_states: base_model.fret_states,
      tuning,
    }
  }

  fn get_note_by_coordinate(tuning: Vec<Note>, coord: FretCoord) -> Note {
    tuning
      .get(coord.string_idx as usize)
      .expect("Broken string bounds checking with tuning")
      .add_steps(coord.fret_idx as usize)
  }

  pub fn get_num_frets(&self) -> u8 {
    self.end_fret.get() as u8 - self.start_fret.get() as u8
  }

  // fn generate_frets(num_strings: u8, num_frets: u8) -> FretboardSignals {
  //   let mut frets: FretboardSignals = Vec::with_capacity(num_strings as usize);

  //   for _ in 0..=num_strings {
  //     let mut string_frets = Vec::with_capacity(num_frets as usize);
  //     for _ in 0..=num_frets {
  //       string_frets.push(RwSignal::new(FretState::Hidden));
  //     }
  //     frets.push(RwSignal::new(string_frets));
  //   }

  //   frets
  // }

  // pub fn update_num_frets(&mut self, num_frets: u8) {
  //   let current = self.num_frets.get();

  //   // Only update if actually changing
  //   if num_frets == current {
  //     return;
  //   }

  //   log!("Updating number of frets from {} to {}", current, num_frets);
  //   // Set the new number of frets
  //   self.num_frets.set(num_frets);

  //   if num_frets > current {
  //     for string_idx in 0..self.num_strings as usize {
  //       let string_frets = &mut self.frets[string_idx];

  //       // Add additional fret signals
  //       for _ in (current + 1)..=num_frets {
  //         string_frets.update(|string_frets| string_frets.push(RwSignal::new(FretState::Hidden)));
  //       }
  //     }
  //   }
  //   // If decreasing, we can leave the extra frets in the array
  //   // as they won't be displayed or accessed
  // }

  // pub fn six_string_standard_tuning(num_frets: u8) -> Self {
  //   Self::new(0, num_frets, Self::standard_tuning())
  // }

  // fn get_fret_state(&self, coord: FretCoord) -> RwSignal<FretState> {
  //   self.frets[coord.string_idx as usize].get_untracked()[coord.fret_idx as usize]
  // }

  // pub fn set_fret_state(&self, coord: FretCoord, state: FretState) {
  //   self.get_fret_state(coord).set(state);
  // }

  // pub fn set_all(&self, state: FretState) {
  //   for string in &self.frets {
  //     for fret in string.get() {
  //       fret.set(state.clone());
  //     }
  //   }
  // }

  // pub fn get_num_strings(&self) -> u8 {
  //   self.num_strings
  // }

  // pub fn get_num_frets(&self) -> ReadSignal<u8> {
  //   self.num_frets.read_only()
  // }

  // pub fn get_tuning(&self) -> &[Note] {
  //   &self.tuning
  // }

  pub fn standard_tuning() -> Vec<Note> {
    // Standard guitar tuning from thinnest to thickest string (top to bottom on fretboard display)
    // String 0 (top): High E (1st string)
    // String 1: B (2nd string)
    // String 2: G (3rd string)
    // String 3: D (4th string)
    // String 4: A (5th string)
    // String 5 (bottom): Low E (6th string)
    vec![Note::E, Note::B, Note::G, Note::D, Note::A, Note::E]
  }

  // pub fn get_frets_of_string(&self, string_no: u8) -> FretStringSignals {
  //   self.frets[string_no as usize]
  // }

  // fn determine_fret_state(note: Note, scale: &Scale) -> FretState {
  //   if scale.contains_note(note) {
  //     match scale.root_note() {
  //       Some(root_note) if root_note == note => {
  //         FretState::Normal(FretStateColor::Red, note.to_string())
  //       }
  //       _ => FretState::Normal(FretStateColor::Blue, note.to_string()),
  //     }
  //   } else {
  //     FretState::Hidden
  //   }
  // }

  // pub fn update_from_scale(&self, scale: &Scale) {
  //   for (string_idx, &tuning) in self.tuning.iter().enumerate() {
  //     let string_idx = string_idx as u8;

  //     // Open string (fret 0)
  //     let open_state = Self::determine_fret_state(tuning, scale);

  //     if string_idx < self.num_strings {
  //       let coord = FretCoord {
  //         string_idx,
  //         fret_idx: 0,
  //       };
  //       self.set_fret_state(coord, open_state);
  //     }

  //     // Fretted notes
  //     for fret_idx in 1..=self.num_frets.get() {
  //       let note = tuning.add_steps(fret_idx as usize);

  //       let state = Self::determine_fret_state(note, scale);

  //       if string_idx < self.num_strings && fret_idx <= self.num_frets.get() {
  //         let coord = FretCoord {
  //           string_idx,
  //           fret_idx,
  //         };
  //         self.set_fret_state(coord, state);
  //       }
  //     }
  //   }
  // }
}
