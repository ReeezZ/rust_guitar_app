use std::collections::HashMap;

use leptos::prelude::RwSignal;
use shared::Note;

// Upper bounds used for preallocation; keeps per-cell signals stable (never created inside Effects).
// Adjust if you need more strings/frets; existing UI sliders should clamp within these maxima.
pub const MAX_STRINGS: usize = 8; // supports up to 8-string instruments
pub const MAX_FRETS: usize = 25; // frets 0..=24

// todo consider performance
pub type FretStateSignals = HashMap<FretCoord, RwSignal<FretState>>;

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub struct FretCoord {
  pub string_idx: u8,
  pub fret_idx: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct FretClickEvent {
  pub note: Note,
  pub coord: FretCoord,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FretStateColor {
  Red,
  Green,
  Blue,
}

#[derive(Clone, PartialEq, Debug)]
pub enum FretState {
  Hidden,
  Normal(FretStateColor, String),
}
