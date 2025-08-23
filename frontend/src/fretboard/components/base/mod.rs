mod fretboard;
mod helper;
mod layout;
mod parts;

/// for demo page, should probably be private
pub use fretboard::Fretboard;
/// Event emitted when a fret position is clicked on the SVG fretboard
pub use fretboard::FretboardViewModel;
pub use helper::{FretState, FretStateColor};
