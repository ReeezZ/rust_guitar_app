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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FretState {
  Hidden,
  Normal,
  Colored(FretStateColor),
}
