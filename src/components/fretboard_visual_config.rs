use leptos::prelude::*;

/// Configuration struct for visual fretboard properties shared across all fretboard components.
/// 
/// This consolidates the common visual configuration that's duplicated across
/// SvgFretboard, SvgFretboardWithNotes, and the higher-level components.
/// Contains only visual/display properties, no musical properties.
///
/// # Example
/// ```rust
/// let config = FretboardVisualConfig::default()
///   .with_num_strings(4) // Bass guitar
///   .with_aspect_ratio(4.0); // Wider aspect ratio
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct FretboardVisualConfig {
  /// Number of guitar strings (default: 6)
  pub num_strings: u8,
  /// Maximum number of frets to display (default: 22)
  pub max_frets: usize,
  /// Width-to-height aspect ratio (default: 3.0)
  pub svg_aspect_ratio: f64,
  /// Percentage of SVG height used as margin (default: 0.05)
  pub fret_margin_percentage: f64,
  /// Width of the nut in SVG units (default: 14.0)
  pub nut_width: f64,
  /// Number of extra frets to show for context (default: 1)
  pub extra_frets: usize,
  /// Fret positions where markers should be displayed
  pub marker_positions: Vec<u8>,
}

impl Default for FretboardVisualConfig {
  fn default() -> Self {
    Self {
      num_strings: 6,
      max_frets: 22,
      svg_aspect_ratio: 3.0,
      fret_margin_percentage: 0.05,
      nut_width: 14.0,
      extra_frets: 1,
      marker_positions: vec![3, 5, 7, 9, 12, 15, 17, 19, 21, 24],
    }
  }
}

impl FretboardVisualConfig {
  /// Create a new visual configuration with default values
  pub fn new() -> Self {
    Self::default()
  }

  /// Builder method to set the number of strings
  pub fn with_num_strings(mut self, num_strings: u8) -> Self {
    self.num_strings = num_strings;
    self
  }

  /// Builder method to set the maximum number of frets
  pub fn with_max_frets(mut self, max_frets: usize) -> Self {
    self.max_frets = max_frets;
    self
  }

  /// Builder method to set the aspect ratio
  pub fn with_aspect_ratio(mut self, ratio: f64) -> Self {
    self.svg_aspect_ratio = ratio;
    self
  }

  /// Builder method to set the fret margin percentage
  pub fn with_fret_margin(mut self, margin: f64) -> Self {
    self.fret_margin_percentage = margin;
    self
  }

  /// Builder method to set the nut width
  pub fn with_nut_width(mut self, width: f64) -> Self {
    self.nut_width = width;
    self
  }

  /// Builder method to set the extra frets
  pub fn with_extra_frets(mut self, extra: usize) -> Self {
    self.extra_frets = extra;
    self
  }

  /// Builder method to set marker positions
  pub fn with_marker_positions(mut self, positions: Vec<u8>) -> Self {
    self.marker_positions = positions;
    self
  }

  /// Preset for bass guitar configuration (4 strings)
  pub fn bass_guitar() -> Self {
    Self::default().with_num_strings(4)
  }

  /// Preset for 7-string guitar configuration
  pub fn seven_string() -> Self {
    Self::default().with_num_strings(7)
  }

  /// Preset for wider aspect ratio (good for larger displays)
  pub fn wide_aspect() -> Self {
    Self::default().with_aspect_ratio(4.0)
  }
}

/// Convert FretboardVisualConfig to individual signals for components that need them.
/// This allows gradual migration and compatibility with existing component APIs.
#[derive(Clone)]
pub struct FretboardVisualConfigSignals {
  pub num_strings: Signal<u8>,
  pub max_frets: Signal<usize>,
  pub svg_aspect_ratio: Signal<f64>,
  pub fret_margin_percentage: Signal<f64>,
  pub nut_width: Signal<f64>,
  pub extra_frets: Signal<usize>,
  pub marker_positions: Signal<Vec<u8>>,
}

impl From<FretboardVisualConfig> for FretboardVisualConfigSignals {
  fn from(config: FretboardVisualConfig) -> Self {
    Self {
      num_strings: Signal::derive(move || config.num_strings),
      max_frets: Signal::derive(move || config.max_frets),
      svg_aspect_ratio: Signal::derive(move || config.svg_aspect_ratio),
      fret_margin_percentage: Signal::derive(move || config.fret_margin_percentage),
      nut_width: Signal::derive(move || config.nut_width),
      extra_frets: Signal::derive(move || config.extra_frets),
      marker_positions: Signal::derive(move || config.marker_positions.clone()),
    }
  }
}
