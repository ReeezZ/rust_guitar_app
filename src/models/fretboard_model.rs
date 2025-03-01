use leptos::prelude::*;

use crate::music::notes::Note;
use crate::music::scales::{Scale, ScaleType};

#[derive(Clone, PartialEq, Debug)]
pub struct NoteDisplay {
  pub visible: bool,
  pub display_text: String,
  pub is_root: bool,
  // Add more properties as needed (is_blue_note, etc.)
}

#[derive(Clone, PartialEq, Debug)]
pub struct FretboardModel {
  root_note: ReadSignal<Note>,
  scale_type: ReadSignal<ScaleType>,
  scale: Memo<Scale>,
}

impl FretboardModel {
  pub fn new() -> Self {
    let (root_note, set_root_note) = signal(Note::C);
    let (scale_type, set_scale_type) = signal(ScaleType::Major);

    // Create a memo that computes the scale when root or type changes
    let scale = Memo::new(move |_| Scale::new(root_note.get(), scale_type.get()));

    provide_context(set_root_note);
    provide_context(set_scale_type);

    Self {
      root_note,
      scale_type,
      scale,
    }
  }

  pub fn root_note(&self) -> ReadSignal<Note> {
    self.root_note
  }

  pub fn scale_type(&self) -> ReadSignal<ScaleType> {
    self.scale_type
  }

  // Key method: evaluate a note given current scale settings
  pub fn evaluate_note(&self, note: Note) -> Memo<NoteDisplay> {
    let scale = self.scale.clone();
    let root_note = self.root_note.clone();
    Memo::new(move |_| {
      let scale = scale.get();
      let is_in_scale = scale.contains_note(note);
      let is_root = note == root_note.get();

      NoteDisplay {
        visible: is_in_scale,
        display_text: note.to_string(),
        is_root,
      }
    })
  }
}
