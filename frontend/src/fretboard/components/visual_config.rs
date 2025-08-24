use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct FretboardVisualConfig {
  /// Width-to-height aspect ratio (default: 3.0)
  pub svg_aspect_ratio: Signal<f64>,
  /// Percentage of SVG height used as margin (default: 0.05)
  pub fret_margin_percentage: Signal<f64>,
  /// Width of the nut in SVG units (default: 14.0)
  pub nut_width: Signal<f64>,
  /// Number of extra frets to show for context (default: 1)
  pub extra_frets: Signal<usize>,
  /// Fret positions where markers should be displayed
  pub marker_positions: Signal<Vec<usize>>,
}

impl Default for FretboardVisualConfig {
  fn default() -> Self {
    FretboardVisualConfigBuilder::new().build()
  }
}

impl FretboardVisualConfig {}

pub struct FretboardVisualConfigBuilder {
  svg_aspect_ratio: Option<Signal<f64>>,
  fret_margin_percentage: Option<Signal<f64>>,
  nut_width: Option<Signal<f64>>,
  extra_frets: Option<Signal<usize>>,
  marker_positions: Option<Signal<Vec<usize>>>,
}

impl FretboardVisualConfigBuilder {
  pub fn new() -> Self {
    Self {
      svg_aspect_ratio: None,
      fret_margin_percentage: None,
      nut_width: None,
      extra_frets: None,
      marker_positions: None,
    }
  }

  pub fn svg_aspect_ratio(mut self, ratio: Signal<f64>) -> Self {
    self.svg_aspect_ratio = Some(ratio);
    self
  }

  pub fn fret_margin_percentage(mut self, percentage: Signal<f64>) -> Self {
    self.fret_margin_percentage = Some(percentage);
    self
  }

  pub fn nut_width(mut self, width: Signal<f64>) -> Self {
    self.nut_width = Some(width);
    self
  }

  pub fn extra_frets(mut self, extra: Signal<usize>) -> Self {
    self.extra_frets = Some(extra);
    self
  }

  pub fn marker_positions(mut self, positions: Signal<Vec<usize>>) -> Self {
    self.marker_positions = Some(positions);
    self
  }

  pub fn build(self) -> FretboardVisualConfig {
    let svg_aspect_ratio = self
      .svg_aspect_ratio
      .unwrap_or_else(|| Signal::derive(move || 3.0));

    let fret_margin_percentage = self
      .fret_margin_percentage
      .unwrap_or_else(|| Signal::derive(move || 0.05));

    let nut_width = self
      .nut_width
      .unwrap_or_else(|| Signal::derive(move || 14.0));

    let extra_frets = self
      .extra_frets
      .unwrap_or_else(|| Signal::derive(move || 1));

    let marker_positions = self
      .marker_positions
      .unwrap_or_else(|| Signal::derive(move || vec![3, 5, 7, 9, 12, 15, 17, 19, 21, 24]));

    FretboardVisualConfig {
      svg_aspect_ratio,
      fret_margin_percentage,
      nut_width,
      extra_frets,
      marker_positions,
    }
  }
}
