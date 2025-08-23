use leptos::prelude::*;

use crate::fretboard::fretboard_model::FretCoord;

/// Snapshot of fretboard geometry for a render cycle.
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct LayoutSnapshot {
  pub positions: Signal<Vec<f64>>,
  pub num_strings: Signal<u8>,
  pub string_spacing: Signal<f64>,
  pub svg_width: Signal<f64>,
  pub svg_height: Signal<f64>,
  pub fret_margin: Signal<f64>,
  pub nut_width: Signal<f64>,
  pub has_nut: Signal<bool>,
  range_start: Signal<f64>,
  scale_factor: Signal<f64>,
}

impl LayoutSnapshot {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    positions: Signal<Vec<f64>>,
    min_visible_fret: Signal<usize>,
    max_visible_fret: Signal<usize>,
    num_strings: Signal<u8>,
    string_spacing: Signal<f64>,
    svg_width: Signal<f64>,
    svg_height: Signal<f64>,
    fret_margin: Signal<f64>,
    nut_width: Signal<f64>,
    has_nut: Signal<bool>,
    num_frets: Signal<usize>,
  ) -> Self {
    let range_start = Signal::derive(move || {
      if has_nut.get() {
        0.0
      } else {
        positions.get()[0]
      }
    });
    let scale_factor = Signal::derive(move || {
      let range_end = positions.get()[num_frets.get()];
      let range_width = range_end - range_start.get();
      let available_width = if has_nut.get() {
        svg_width.get() - nut_width.get()
      } else {
        svg_width.get()
      };
      available_width / range_width
    });

    Self {
      positions: positions,
      num_strings,
      string_spacing,
      svg_width,
      svg_height,
      fret_margin,
      nut_width,
      has_nut: has_nut.into(),
      range_start: range_start.into(),
      scale_factor: scale_factor.into(),
    }
  }

  fn abs_to_viewbox_x(&self, absolute_x: f64) -> f64 {
    let offset = if self.has_nut.get() {
      self.nut_width.get()
    } else {
      0.0
    };
    offset + (absolute_x - self.range_start.get()) * self.scale_factor.get()
  }

  /// Transform an absolute (unscaled) x coordinate into the current viewbox space
  pub fn absolute_to_viewbox_x(&self, absolute_x: f64) -> f64 {
    self.abs_to_viewbox_x(absolute_x)
  }

  /// Effective nut width (0 if nut not visible)
  pub fn effective_nut_width(&self) -> f64 {
    if self.has_nut.get() {
      self.nut_width.get()
    } else {
      0.0
    }
  }

  // Removed unused fret_line_x helper (can be reinstated if needed later)

  pub fn fret_center_x(&self, fret: usize) -> Option<f64> {
    if fret == 0 {
      return if self.has_nut.get() {
        Some(self.nut_width.get() / 2.0)
      } else {
        None
      };
    }
    if fret < self.positions.get().len() {
      let prev = self.positions.get()[fret - 1];
      let curr = self.positions.get()[fret];
      Some(self.abs_to_viewbox_x((prev + curr) / 2.0))
    } else {
      None
    }
  }

  pub fn string_y(&self, string_idx: u8) -> f64 {
    (string_idx as f64 + 1.0) * self.string_spacing.get()
  }

  pub fn note_position(&self, coord: FretCoord) -> Option<(f64, f64)> {
    let x = self.fret_center_x(coord.fret_idx as usize)?;
    let y = self.string_y(coord.string_idx);
    Some((x, y))
  }
}
