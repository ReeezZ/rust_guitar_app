use leptos::prelude::*;

/// Configuration struct for visual fretboard properties shared across all fretboard components.
///
/// This consolidates the common visual configuration that controls the appearance
/// and layout of fretboard components. It handles display properties like dimensions,
/// spacing, and visual markers, but not musical properties like tuning or scales.
///
/// # Basic Usage
/// ```rust
/// use rust_guitar_app::components::fretboard_visual_config::FretboardVisualConfig;
///
/// // Use defaults (6-string guitar, 22 frets, 3:1 aspect ratio)
/// let config = FretboardVisualConfig::default();
///
/// // Or customize with builder pattern
/// let custom_config = FretboardVisualConfig::default()
///   .with_num_strings(7)  // 7-string guitar
///   .with_max_frets(24)   // 24 frets
///   .with_aspect_ratio(4.0); // Wider display
/// ```
///
/// # Preset Configurations
/// ```rust
/// use rust_guitar_app::components::fretboard_visual_config::FretboardVisualConfig;
///
/// // Bass guitar (4 strings)
/// let bass_config = FretboardVisualConfig::bass_guitar();
///
/// // 7-string guitar
/// let seven_string_config = FretboardVisualConfig::seven_string();
///
/// // Wide aspect ratio for larger displays
/// let wide_config = FretboardVisualConfig::wide_aspect();
///
/// // Chain presets with customizations
/// let custom_bass = FretboardVisualConfig::bass_guitar()
///   .with_max_frets(20)
///   .with_fret_margin(0.08);
/// ```
///
/// # Advanced Customization
/// ```rust
/// use rust_guitar_app::components::fretboard_visual_config::FretboardVisualConfig;
///
/// let advanced_config = FretboardVisualConfig::default()
///   .with_marker_positions(vec![3, 5, 7, 9, 12, 15, 17]) // Custom fret markers
///   .with_nut_width(16.0)    // Wider nut
///   .with_extra_frets(2);    // Show 2 extra frets for context
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
  ///
  /// # Arguments
  /// * `num_strings` - Number of guitar strings (typically 4-8)
  ///
  /// # Examples
  /// ```rust
  /// use rust_guitar_app::components::fretboard_visual_config::FretboardVisualConfig;
  ///
  /// let bass = FretboardVisualConfig::default().with_num_strings(4);
  /// let seven_string = FretboardVisualConfig::default().with_num_strings(7);
  /// ```
  pub fn with_num_strings(mut self, num_strings: u8) -> Self {
    self.num_strings = num_strings;
    self
  }

  /// Builder method to set the maximum number of frets
  ///
  /// Controls how many frets are available for display, regardless of the
  /// current view range (start_fret to end_fret).
  ///
  /// # Arguments
  /// * `max_frets` - Maximum frets (typically 12-24)
  ///
  /// # Examples
  /// ```rust
  /// use rust_guitar_app::components::fretboard_visual_config::FretboardVisualConfig;
  ///
  /// let acoustic = FretboardVisualConfig::default().with_max_frets(14);
  /// let electric = FretboardVisualConfig::default().with_max_frets(24);
  /// ```
  pub fn with_max_frets(mut self, max_frets: usize) -> Self {
    self.max_frets = max_frets;
    self
  }

  /// Builder method to set the aspect ratio
  ///
  /// Controls the width-to-height ratio of the SVG display.
  /// Higher values create wider, shorter fretboards.
  ///
  /// # Arguments
  /// * `ratio` - Width/height ratio (typically 2.0-5.0)
  ///
  /// # Examples
  /// ```rust
  /// use rust_guitar_app::components::fretboard_visual_config::FretboardVisualConfig;
  ///
  /// let compact = FretboardVisualConfig::default().with_aspect_ratio(2.5);
  /// let wide = FretboardVisualConfig::default().with_aspect_ratio(4.5);
  /// ```
  pub fn with_aspect_ratio(mut self, ratio: f64) -> Self {
    self.svg_aspect_ratio = ratio;
    self
  }

  /// Builder method to set the fret margin percentage
  ///
  /// Controls the spacing around the fretboard as a percentage of total height.
  ///
  /// # Arguments
  /// * `margin` - Margin as decimal percentage (e.g., 0.05 = 5%)
  ///
  /// # Examples
  /// ```rust
  /// use rust_guitar_app::components::fretboard_visual_config::FretboardVisualConfig;
  ///
  /// let tight = FretboardVisualConfig::default().with_fret_margin(0.02);
  /// let spacious = FretboardVisualConfig::default().with_fret_margin(0.1);
  /// ```
  pub fn with_fret_margin(mut self, margin: f64) -> Self {
    self.fret_margin_percentage = margin;
    self
  }

  /// Builder method to set the nut width
  ///
  /// Controls the visual width of the nut (zero fret) in SVG units.
  ///
  /// # Arguments
  /// * `width` - Nut width in SVG units (typically 10.0-20.0)
  pub fn with_nut_width(mut self, width: f64) -> Self {
    self.nut_width = width;
    self
  }

  /// Builder method to set the extra frets
  ///
  /// Controls how many frets beyond the visible range to show for context.
  ///
  /// # Arguments
  /// * `extra` - Number of extra frets (typically 0-3)
  pub fn with_extra_frets(mut self, extra: usize) -> Self {
    self.extra_frets = extra;
    self
  }

  /// Builder method to set marker positions
  ///
  /// Sets which fret positions should display visual markers (dots).
  ///
  /// # Arguments
  /// * `positions` - Vector of fret numbers to mark
  ///
  /// # Examples
  /// ```rust
  /// use rust_guitar_app::components::fretboard_visual_config::FretboardVisualConfig;
  ///
  /// // Standard guitar markers
  /// let standard = FretboardVisualConfig::default()
  ///   .with_marker_positions(vec![3, 5, 7, 9, 12, 15, 17, 19, 21]);
  ///
  /// // Minimal markers
  /// let minimal = FretboardVisualConfig::default()
  ///   .with_marker_positions(vec![3, 12]);
  /// ```
  pub fn with_marker_positions(mut self, positions: Vec<u8>) -> Self {
    self.marker_positions = positions;
    self
  }

  /// Preset configuration for bass guitar (4 strings)
  ///
  /// Creates a configuration optimized for bass guitar display with fewer strings
  /// and adjusted proportions.
  ///
  /// Features:
  /// - 4 strings instead of standard 6
  /// - Same fret range as standard guitar
  /// - Default aspect ratio works well for bass proportions
  ///
  /// # Returns
  /// A `FretboardVisualConfig` configured for 4-string bass guitar
  ///
  /// # Examples
  /// ```rust
  /// use rust_guitar_app::components::fretboard_visual_config::FretboardVisualConfig;
  ///
  /// let bass_config = FretboardVisualConfig::bass_guitar();
  /// // Further customize if needed
  /// let custom_bass = bass_config.with_max_frets(20);
  /// ```
  pub fn bass_guitar() -> Self {
    Self::default().with_num_strings(4)
  }

  /// Preset configuration for 7-string guitar
  ///
  /// Creates a configuration for 7-string guitars commonly used in metal
  /// and extended range playing.
  ///
  /// Features:
  /// - 7 strings instead of standard 6
  /// - Standard fret range (same as 6-string)
  /// - Same aspect ratio (7th string doesn't significantly change proportions)
  ///
  /// # Returns
  /// A `FretboardVisualConfig` configured for 7-string guitar
  ///
  /// # Examples
  /// ```rust
  /// use rust_guitar_app::components::fretboard_visual_config::FretboardVisualConfig;
  ///
  /// let seven_string = FretboardVisualConfig::seven_string();
  /// // Add extended fret range if desired
  /// let extended_seven = seven_string.with_max_frets(24);
  /// ```
  pub fn seven_string() -> Self {
    Self::default().with_num_strings(7)
  }

  /// Preset configuration for wide aspect ratio display
  ///
  /// Creates a configuration with a wider aspect ratio, useful for:
  /// - Horizontal layouts where height is constrained
  /// - Desktop displays with wide screens
  /// - Embedded views in wider containers
  ///
  /// Features:
  /// - 4.0 aspect ratio (vs default 3.0)
  /// - Creates a more stretched, horizontal appearance
  /// - Same string and fret configuration as default
  ///
  /// # Returns
  /// A `FretboardVisualConfig` with wide aspect ratio
  /// ```
  pub fn wide_aspect() -> Self {
    Self::default().with_aspect_ratio(4.0)
  }
}

/// Signal utilities for FretboardVisualConfig
///
/// Provides individual reactive signals for each configuration property.
/// This is used internally by components that need reactive access to
/// individual config properties.
#[derive(Clone)]
pub struct FretboardVisualConfigSignals {
  /// Reactive signal for number of strings
  pub num_strings: Signal<u8>,
  /// Reactive signal for maximum frets
  pub max_frets: Signal<usize>,
  /// Reactive signal for SVG aspect ratio
  pub svg_aspect_ratio: Signal<f64>,
  /// Reactive signal for fret margin percentage
  pub fret_margin_percentage: Signal<f64>,
  /// Reactive signal for nut width
  pub nut_width: Signal<f64>,
  /// Reactive signal for extra frets
  pub extra_frets: Signal<usize>,
  /// Reactive signal for marker positions
  pub marker_positions: Signal<Vec<u8>>,
}

impl From<FretboardVisualConfig> for FretboardVisualConfigSignals {
  /// Convert a FretboardVisualConfig into individual reactive signals
  ///
  /// This conversion creates derived signals for each configuration property,
  /// allowing components to access individual reactive config properties.
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
