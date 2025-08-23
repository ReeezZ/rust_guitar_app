use leptos::prelude::RwSignal;

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
  pub svg_aspect_ratio: RwSignal<f64>,
  /// Percentage of SVG height used as margin (default: 0.05)
  pub fret_margin_percentage: RwSignal<f64>,
  /// Width of the nut in SVG units (default: 14.0)
  pub nut_width: RwSignal<f64>,
  /// Number of extra frets to show for context (default: 1)
  pub extra_frets: RwSignal<usize>,
  /// Fret positions where markers should be displayed
  pub marker_positions: RwSignal<Vec<usize>>,
}

impl Default for FretboardVisualConfig {
  fn default() -> Self {
    Self {
      svg_aspect_ratio: RwSignal::new(3.0),
      fret_margin_percentage: RwSignal::new(0.05),
      nut_width: RwSignal::new(14.0),
      extra_frets: RwSignal::new(1),
      marker_positions: RwSignal::new(vec![3, 5, 7, 9, 12, 15, 17, 19, 21, 24]),
    }
  }
}
