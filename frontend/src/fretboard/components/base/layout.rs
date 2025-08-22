use std::sync::Arc;

use crate::fretboard::fretboard_model::FretCoord;

/// Snapshot of fretboard geometry for a render cycle.
#[derive(Clone, Debug)]
pub struct LayoutSnapshot {
  pub positions: Arc<Vec<f64>>,
  pub min_fret: usize,
  pub max_fret: usize,
  pub start_fret: usize,
  pub end_fret: usize,
  pub num_strings: u8,
  pub string_spacing: f64,
  pub svg_width: f64,
  pub svg_height: f64,
  pub fret_margin: f64,
  pub nut_width: f64,
  pub has_nut: bool,
  range_start: f64,
  scale_factor: f64,
}

impl LayoutSnapshot {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    positions: Vec<f64>,
    min_fret: usize,
    max_fret: usize,
    start_fret: usize,
    end_fret: usize,
    num_strings: u8,
    string_spacing: f64,
    svg_width: f64,
    svg_height: f64,
    fret_margin: f64,
    nut_width: f64,
  ) -> Self {
    let has_nut = min_fret == 0;
    let range_start = if has_nut { 0.0 } else { positions[min_fret] };
    let range_end = positions[max_fret];
    let range_width = range_end - range_start;
    let available_width = if has_nut {
      svg_width - nut_width
    } else {
      svg_width
    };
    let scale_factor = available_width / range_width;
    Self {
      positions: Arc::new(positions),
      min_fret,
      max_fret,
      start_fret,
      end_fret,
      num_strings,
      string_spacing,
      svg_width,
      svg_height,
      fret_margin,
      nut_width,
      has_nut,
      range_start,
      scale_factor,
    }
  }

  fn abs_to_viewbox_x(&self, absolute_x: f64) -> f64 {
    let offset = if self.has_nut { self.nut_width } else { 0.0 };
    offset + (absolute_x - self.range_start) * self.scale_factor
  }

  /// Transform an absolute (unscaled) x coordinate into the current viewbox space
  pub fn absolute_to_viewbox_x(&self, absolute_x: f64) -> f64 {
    self.abs_to_viewbox_x(absolute_x)
  }

  /// Effective nut width (0 if nut not visible)
  pub fn effective_nut_width(&self) -> f64 {
    if self.has_nut {
      self.nut_width
    } else {
      0.0
    }
  }

  // Removed unused fret_line_x helper (can be reinstated if needed later)

  pub fn fret_center_x(&self, fret: usize) -> Option<f64> {
    if fret == 0 {
      return if self.has_nut {
        Some(self.nut_width / 2.0)
      } else {
        None
      };
    }
    if fret < self.positions.len() {
      let prev = self.positions[fret - 1];
      let curr = self.positions[fret];
      Some(self.abs_to_viewbox_x((prev + curr) / 2.0))
    } else {
      None
    }
  }

  pub fn string_y(&self, string_idx: u8) -> f64 {
    (string_idx as f64 + 1.0) * self.string_spacing
  }

  pub fn note_position(&self, coord: FretCoord) -> Option<(f64, f64)> {
    let x = self.fret_center_x(coord.fret_idx as usize)?;
    let y = self.string_y(coord.string_idx);
    Some((x, y))
  }
}
