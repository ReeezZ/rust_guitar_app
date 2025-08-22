/// Configuration struct for visual fretboard properties shared across all fretboard components.
///
/// This consolidates the common visual configuration that controls the appearance
/// and layout of fretboard components. It handles display properties like dimensions,
/// spacing, and visual markers, but not musical properties like tuning or scales.
///
/// # Basic Usage
/// ```rust
/// use frontend::components::fretboard::visual_config::FretboardVisualConfig;
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
/// use frontend::components::fretboard::visual_config::FretboardVisualConfig;
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
/// use frontend::components::fretboard::visual_config::FretboardVisualConfig;
///
/// let advanced_config = FretboardVisualConfig::default()
///   .with_marker_positions(vec![3, 5, 7, 9, 12, 15, 17]) // Custom fret markers
///   .with_nut_width(16.0)    // Wider nut
///   .with_extra_frets(2);    // Show 2 extra frets for context
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct FretboardVisualConfig {
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
      svg_aspect_ratio: 3.0,
      fret_margin_percentage: 0.05,
      nut_width: 14.0,
      extra_frets: 1,
      marker_positions: vec![3, 5, 7, 9, 12, 15, 17, 19, 21, 24],
    }
  }
}

impl FretboardVisualConfig {
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
  /// use frontend::components::fretboard::visual_config::FretboardVisualConfig;
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
  /// use frontend::components::fretboard::visual_config::FretboardVisualConfig;
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
  /// use frontend::components::fretboard::visual_config::FretboardVisualConfig;
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
}
