/// Calculate string spacing for the given number of strings and SVG height
pub fn calculate_string_spacing(num_strings: u8, svg_height: f64) -> f64 {
  svg_height / (num_strings as f64 + 1.0)
}

/// Visible fret range including context frets
#[derive(Debug, Clone, PartialEq)]
pub struct VisibleRange {
  /// Minimum fret to display (includes context)
  pub min_fret: usize,
  /// Maximum fret to display (includes context)
  pub max_fret: usize,
}

impl VisibleRange {
  /// Calculate the visible fret range including extra context frets
  pub fn new(start_fret: usize, end_fret: usize, extra_frets: usize, max_frets: usize) -> Self {
    let min_fret = start_fret.saturating_sub(extra_frets);
    let max_fret = (end_fret + extra_frets).min(max_frets);

    Self { min_fret, max_fret }
  }
}

/// Parameters for coordinate transformation and zoom scaling
#[derive(Debug, Clone, PartialEq)]
pub struct ZoomTransform {
  /// Starting position of the visible range in absolute coordinates
  pub range_start: f64,
  /// Scale factor to apply to coordinates
  pub scale_factor: f64,
  /// Whether the nut is visible (fret 0 is in range)
  pub has_nut: bool,
}

impl ZoomTransform {
  /// Create a new zoom transform for the given fret range and dimensions
  pub fn new(
    positions: &[f64],
    min_fret: usize,
    max_fret: usize,
    svg_width: f64,
    nut_width: f64,
  ) -> Self {
    let has_nut = min_fret == 0;

    // Physical range we want to display
    let range_start = if has_nut { 0.0 } else { positions[min_fret] };
    let range_end = positions[max_fret];
    let range_width = range_end - range_start;

    // Available width for fret content (accounting for nut if visible)
    let available_width = if has_nut {
      svg_width - nut_width
    } else {
      svg_width
    };

    // Scale factor to make the selected range fill the available width
    let scale_factor = available_width / range_width;

    Self {
      range_start,
      scale_factor,
      has_nut,
    }
  }

  /// Transform absolute fretboard coordinates to scaled viewbox coordinates
  pub fn to_viewbox_x(&self, absolute_x: f64, nut_width: f64) -> f64 {
    let offset = if self.has_nut { nut_width } else { 0.0 };
    offset + (absolute_x - self.range_start) * self.scale_factor
  }

  /// Get nut width to use in calculations (0 if nut not visible)
  pub fn effective_nut_width(&self, nut_width: f64) -> f64 {
    if self.has_nut {
      nut_width
    } else {
      0.0
    }
  }
}
