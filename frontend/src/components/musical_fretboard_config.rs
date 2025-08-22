use shared::music::notes::Note;

use crate::fretboard::components::visual_config::FretboardVisualConfig;


#[derive(Clone, Debug)]
pub struct FretboardBaseModelBuilder {
  pub start_fret: usize,
  pub end_fret: usize,
  pub num_strings: u8,
  /// Visual configuration (aspect ratio, strings, margins, etc.)
  pub visual: FretboardVisualConfig,
}

pub trait FretboardBaseModelBuilderTrait {
  fn with_start_fret(self, fret: usize) -> Self;
  fn with_end_fret(self, fret: usize) -> Self;
  fn with_num_strings(self, num: u8) -> Self;

  // Overwrite the entire visual config
  fn with_visual(self, visual: FretboardVisualConfig) -> Self;
  // Set the aspect ratio of the visual config
  fn with_aspect_ratio(self, ratio: f64) -> Self;

  // TODO comments for the visual config fields
  fn with_fret_margin(self, margin: f64) -> Self;
  fn with_nut_width(self, width: f64) -> Self;
  fn with_extra_frets(self, extra: usize) -> Self;
  fn with_marker_positions(self, positions: Vec<u8>) -> Self;
}

impl Default for FretboardBaseModelBuilder {
  fn default() -> Self {
    Self {
      start_fret: 0,
      end_fret: 24,
      num_strings: 6,
      visual: FretboardVisualConfig::default(),
    }
  }
}

impl FretboardBaseModelBuilderTrait for FretboardBaseModelBuilder {
  fn with_start_fret(mut self, fret: usize) -> Self {
    self.start_fret = fret;
    self
  }
  fn with_end_fret(mut self, fret: usize) -> Self {
    self.end_fret = fret;
    self
  }
  fn with_num_strings(mut self, num: u8) -> Self {
    self.num_strings = num;
    self
  }

  fn with_visual(mut self, visual: FretboardVisualConfig) -> Self {
    self.visual = visual;
    self
  }

  /// Builder method to set the aspect ratio (updates visual config)
  fn with_aspect_ratio(mut self, ratio: f64) -> Self {
    self.visual.svg_aspect_ratio = ratio;
    self
  }
  /// Builder method to set the fret margin percentage (updates visual config)
  fn with_fret_margin(mut self, margin: f64) -> Self {
    self.visual.fret_margin_percentage = margin;
    self
  }
  /// Builder method to set the nut width (updates visual config)
  fn with_nut_width(mut self, width: f64) -> Self {
    self.visual.nut_width = width;
    self
  }
  /// Builder method to set the extra frets (updates visual config)
  fn with_extra_frets(mut self, extra: usize) -> Self {
    self.visual.extra_frets = extra;
    self
  }
  /// Builder method to set marker positions (updates visual config)
  fn with_marker_positions(mut self, positions: Vec<u8>) -> Self {
    self.visual.marker_positions = positions;
    self
  }
}

impl FretboardBaseModelBuilderTrait for FretboardWithNotesModelBuilder {
  fn with_start_fret(mut self, fret: usize) -> Self {
    self.base = self.base.with_start_fret(fret);
    self
  }
  fn with_end_fret(mut self, fret: usize) -> Self {
    self.base = self.base.with_end_fret(fret);
    self
  }
  fn with_num_strings(mut self, num: u8) -> Self {
    self.base = self.base.with_num_strings(num);
    self
  }
  fn with_visual(mut self, visual: FretboardVisualConfig) -> Self {
    self.base = self.base.with_visual(visual);
    self
  }
  fn with_aspect_ratio(mut self, ratio: f64) -> Self {
    self.base = self.base.with_aspect_ratio(ratio);
    self
  }
  fn with_fret_margin(mut self, margin: f64) -> Self {
    self.base = self.base.with_fret_margin(margin);
    self
  }
  fn with_nut_width(mut self, width: f64) -> Self {
    self.base = self.base.with_nut_width(width);
    self
  }
  fn with_extra_frets(mut self, extra: usize) -> Self {
    self.base = self.base.with_extra_frets(extra);
    self
  }
  fn with_marker_positions(mut self, positions: Vec<u8>) -> Self {
    self.base = self.base.with_marker_positions(positions);
    self
  }
}

pub struct FretboardWithNotesModelBuilder {
  pub base: FretboardBaseModelBuilder,

  /// Guitar tuning (defaults to standard: E-A-D-G-H-E from lowest to highest string)
  pub tuning: Vec<Note>,
}

impl Default for FretboardWithNotesModelBuilder {
  fn default() -> Self {
    Self {
      base: FretboardBaseModelBuilder::default(),
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

pub trait FretboardWithNotesModelBuilderTrait: FretboardBaseModelBuilderTrait {
  /// Builder method to set the tuning
  fn with_tuning(self, tuning: Vec<Note>) -> Self;
  fn seven_string() -> Self;
  fn bass_guitar() -> Self;
  fn drop_d_tuning() -> Self;
}

impl FretboardWithNotesModelBuilderTrait for FretboardWithNotesModelBuilder {
  fn with_tuning(mut self, tuning: Vec<Note>) -> Self {
    self.tuning = tuning;
    // Keep number of strings in sync with tuning automatically
    self.base.num_strings = self.tuning.len() as u8;
    self
  }

  /// Preset for 7-string guitar configuration

  fn seven_string() -> Self {
    Self::default()
      // .with_visual(FretboardVisualConfig::seven_string())
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
  fn bass_guitar() -> Self {
    Self::default()
      // .with_visual(FretboardVisualConfig::bass_guitar())
      .with_tuning(vec![
        Note::E, // 4th string (low E)
        Note::A, // 3rd string
        Note::D, // 2nd string
        Note::G, // 1st string
      ])
  }

  /// Preset for drop D tuning
  fn drop_d_tuning() -> Self {
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