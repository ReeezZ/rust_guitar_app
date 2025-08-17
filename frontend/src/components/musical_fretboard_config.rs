use crate::components::fretboard::visual_config::FretboardVisualConfig;
use leptos::prelude::*;
use shared::music::notes::Note;

/// Configuration struct for musical fretboard visualization components.
///
/// This consolidates visual and musical properties for fretboard components
/// that display notes, scales, chords, or other musical information.
/// It's specifically designed for components like SvgFretboardScaleDisplay.
///
/// # Example
/// ```rust
/// use frontend::components::musical_fretboard_config::MusicalFretboardConfig;
/// use frontend::components::fretboard::visual_config::FretboardVisualConfig;
///
/// let config = MusicalFretboardConfig::default()
///   .with_visual(FretboardVisualConfig::seven_string());
/// ```
#[derive(Clone, Debug)]
pub struct MusicalFretboardConfig {
  /// Visual configuration (aspect ratio, strings, margins, etc.)
  pub visual: FretboardVisualConfig,
  /// Guitar tuning (defaults to standard: E-A-D-G-H-E from lowest to highest string)
  pub tuning: Vec<Note>,
}

impl Default for MusicalFretboardConfig {
  fn default() -> Self {
    Self {
      visual: FretboardVisualConfig::default(),
      tuning: vec![
        Note::E, // 6th string (lowest)
        Note::A, // 5th string
        Note::D, // 4th string
        Note::G, // 3rd string
        Note::B, // 2nd string (B in standard notation)
        Note::E, // 1st string (highest)
      ],
    }
  }
}

impl MusicalFretboardConfig {
  /// Create a new fretboard configuration with default values
  pub fn new() -> Self {
    Self::default()
  }

  /// Builder method to set the visual configuration
  pub fn with_visual(mut self, visual: FretboardVisualConfig) -> Self {
    self.visual = visual;
    self
  }

  /// Builder method to set the number of strings (updates visual config)
  pub fn with_num_strings(mut self, num_strings: u8) -> Self {
    self.visual.num_strings = num_strings;
    self
  }

  /// Builder method to set the maximum number of frets (updates visual config)
  pub fn with_max_frets(mut self, max_frets: usize) -> Self {
    self.visual.max_frets = max_frets;
    self
  }

  /// Builder method to set the aspect ratio (updates visual config)
  pub fn with_aspect_ratio(mut self, ratio: f64) -> Self {
    self.visual.svg_aspect_ratio = ratio;
    self
  }

  /// Builder method to set the fret margin percentage (updates visual config)
  pub fn with_fret_margin(mut self, margin: f64) -> Self {
    self.visual.fret_margin_percentage = margin;
    self
  }

  /// Builder method to set the nut width (updates visual config)
  pub fn with_nut_width(mut self, width: f64) -> Self {
    self.visual.nut_width = width;
    self
  }

  /// Builder method to set the extra frets (updates visual config)
  pub fn with_extra_frets(mut self, extra: usize) -> Self {
    self.visual.extra_frets = extra;
    self
  }

  /// Builder method to set marker positions (updates visual config)
  pub fn with_marker_positions(mut self, positions: Vec<u8>) -> Self {
    self.visual.marker_positions = positions;
    self
  }

  /// Builder method to set the tuning
  pub fn with_tuning(mut self, tuning: Vec<Note>) -> Self {
    self.tuning = tuning;
    self
  }

  /// Preset for 7-string guitar configuration
  pub fn seven_string() -> Self {
    Self::default()
      .with_visual(FretboardVisualConfig::seven_string())
      .with_tuning(vec![
        Note::B, // 7th string (low B)
        Note::E, // 6th string
        Note::A, // 5th string
        Note::D, // 4th string
        Note::G, // 3rd string
        Note::B, // 2nd string
        Note::E, // 1st string
      ])
  }

  /// Preset for bass guitar configuration (4 strings)
  pub fn bass_guitar() -> Self {
    Self::default()
      .with_visual(FretboardVisualConfig::bass_guitar())
      .with_tuning(vec![
        Note::E, // 4th string (low E)
        Note::A, // 3rd string
        Note::D, // 2nd string
        Note::G, // 1st string
      ])
  }

  /// Preset for drop D tuning
  pub fn drop_d_tuning() -> Self {
    Self::default().with_tuning(vec![
      Note::D, // 6th string (dropped to D)
      Note::A, // 5th string
      Note::D, // 4th string
      Note::G, // 3rd string
      Note::B, // 2nd string
      Note::E, // 1st string
    ])
  }
}

/// Convert MusicalFretboardConfig to individual signals for components that need them.
/// This allows gradual migration of existing components.
#[derive(Clone)]
pub struct MusicalFretboardConfigSignals {
  pub num_strings: Signal<u8>,
  pub max_frets: Signal<usize>,
  pub svg_aspect_ratio: Signal<f64>,
  pub fret_margin_percentage: Signal<f64>,
  pub nut_width: Signal<f64>,
  pub extra_frets: Signal<usize>,
  pub marker_positions: Signal<Vec<u8>>,
  pub tuning: Signal<Vec<Note>>,
}

impl From<MusicalFretboardConfig> for MusicalFretboardConfigSignals {
  fn from(config: MusicalFretboardConfig) -> Self {
    Self {
      num_strings: Signal::derive(move || config.visual.num_strings),
      max_frets: Signal::derive(move || config.visual.max_frets),
      svg_aspect_ratio: Signal::derive(move || config.visual.svg_aspect_ratio),
      fret_margin_percentage: Signal::derive(move || config.visual.fret_margin_percentage),
      nut_width: Signal::derive(move || config.visual.nut_width),
      extra_frets: Signal::derive(move || config.visual.extra_frets),
      marker_positions: Signal::derive(move || config.visual.marker_positions.clone()),
      tuning: Signal::derive(move || config.tuning.clone()),
    }
  }
}
