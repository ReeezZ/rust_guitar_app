/// Important: The `scale_length` is the length of the vibrating string (e.g., 648mm for a Stratocaster).
/// This length must be converted into the coordinates of your `viewBox`.
/// For example, if your `viewBox` is 800 units wide and you want to display 24 frets,
/// then the `scale_length` must be scaled accordingly so that it fits the width of your SVG.
pub fn calculate_fret_positions(scale_length: f64, num_frets: u8) -> Vec<f64> {
  let mut positions = Vec::with_capacity(num_frets as usize + 1); // +1 for saddle
  let twelfth_root_of_2 = 2.0_f64.powf(1.0 / 12.0);

  // Position of saddle (fret 0) is 0
  positions.push(0.0);

  for n in 1..=num_frets {
    let position = scale_length * (1.0 - (1.0 / twelfth_root_of_2).powi(n as i32));
    positions.push(position);
  }
  positions
}

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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FretStateColor {
  Red,
  Green,
  Blue,
}

// TODO add a proper to css color string trait
impl FretStateColor {
  pub fn as_str(&self) -> &str {
    match self {
      FretStateColor::Red => "red",
      FretStateColor::Green => "green",
      FretStateColor::Blue => "blue",
    }
  }
}

#[derive(Clone, PartialEq, Debug)]
pub enum FretState {
  Hidden,
  Normal(FretStateColor, String),
}
