use std::collections::HashMap;

use crate::{
  components::fretboard::base::fretboard::FretClickEvent,
  models::{FretCoord, FretState},
};

use leptos::prelude::*;

/// Renders the nut (zero fret) when visible
#[component]
pub fn FretboardNut(
  /// Width of the nut in SVG units
  nut_width: f64,
  /// Top margin for the nut
  fret_margin: f64,
  /// Total SVG height (needed for nut rect height calculation)
  svg_height: f64,
) -> impl IntoView {
  view! {
    <rect
      x="0"
      y=fret_margin
      width=nut_width
      height=svg_height - 2.0 * fret_margin
      fill="#f8f8f8"
      stroke="#222"
      stroke-width="5"
      rx="3"
    />
  }
}

/// Renders all fret lines with different styles for playable vs non-playable
#[component]
pub fn FretboardFrets(
  /// Minimum visible fret number
  min_fret: usize,
  /// Maximum visible fret number  
  max_fret: usize,
  /// Start of playable range
  start_fret: usize,
  /// End of playable range
  end_fret: usize,
  /// Absolute fret positions array
  positions: Vec<f64>,
  /// Function to transform absolute x to viewbox x
  to_viewbox_x: impl Fn(f64) -> f64 + Copy + 'static,
  /// Top margin for frets
  fret_margin: f64,
  /// Total SVG height
  svg_height: f64,
) -> impl IntoView {
  (min_fret..=max_fret)
    .map(|fret_no| {
      let absolute_x = positions[fret_no];
      let x_pos = to_viewbox_x(absolute_x);
      let is_playable = fret_no >= start_fret && fret_no <= end_fret;
      let color = if is_playable { "#444" } else { "#bbb" };
      let width = if is_playable { "5" } else { "3" };

      view! {
        <line
          x1=x_pos
          y1=fret_margin
          x2=x_pos
          y2=svg_height - fret_margin
          stroke=color
          stroke-width=width
          opacity=if is_playable { "1.0" } else { "0.6" }
        />
      }
    })
    .collect_view()
}

/// Renders horizontal string lines
#[component]
pub fn FretboardStrings(
  /// Number of strings on the instrument
  num_strings: u8,
  /// Spacing between strings
  string_spacing: f64,
  /// Total viewbox width
  viewbox_width: f64,
) -> impl IntoView {
  (0..num_strings)
    .map(|i| {
      let y_pos = (i as f64 + 1.0) * string_spacing;
      let string_thickness = 1.0 + (i as f64);

      view! {
        <line
          x1="0"
          y1=y_pos
          x2=viewbox_width
          y2=y_pos
          stroke="#888"
          stroke-width=string_thickness
        />
      }
    })
    .collect_view()
}

/// Renders fret position markers (dots)
#[component]
pub fn FretboardMarkers(
  /// Minimum visible fret number
  min_fret: usize,
  /// Maximum visible fret number
  max_fret: usize,
  /// Fret positions where markers should appear
  marker_positions: Vec<u8>,
  /// Absolute fret positions array
  positions: Vec<f64>,
  /// Function to transform absolute x to viewbox x
  to_viewbox_x: impl Fn(f64) -> f64 + Copy + 'static,
  /// Total SVG height
  svg_height: f64,
) -> impl IntoView {
  (min_fret..=max_fret)
    .filter(|&fret| marker_positions.contains(&(fret as u8)))
    .map(|fret| {
      let x_prev = positions[(fret - 1).max(0)];
      let x_curr = positions[fret];
      let x_center = (x_prev + x_curr) / 2.0;
      let x = to_viewbox_x(x_center);
      let y = svg_height / 2.0;
      let r = if fret == 12 || fret == 24 { 8.0 } else { 6.0 };
      let y_offset = 28.0;
      let (cy1, cy2, op2) = if fret == 12 || fret == 24 {
        (y - y_offset, y + y_offset, 0.25)
      } else {
        (y, y + y_offset, 0.0)
      };

      view! {
        <g>
          <circle cx=x cy=cy1 r=r fill="#444" opacity="0.25" />
          <circle cx=x cy=cy2 r=r fill="#444" opacity=op2 />
        </g>
      }
    })
    .collect_view()
}

/// Renders semi-transparent overlays for non-playable regions
#[component]
pub fn FretboardOverlays(
  /// Minimum visible fret number
  min_fret: usize,
  /// Maximum visible fret number
  max_fret: usize,
  /// Start of playable range
  start_fret: usize,
  /// End of playable range
  end_fret: usize,
  /// Absolute fret positions array
  positions: Vec<f64>,
  /// Function to transform absolute x to viewbox x
  to_viewbox_x: impl Fn(f64) -> f64 + Copy + 'static,
  /// Width of nut (0 if not visible)
  nut_width: f64,
  /// Top margin for overlays
  fret_margin: f64,
  /// Total SVG height
  svg_height: f64,
  /// Total SVG width
  svg_width: f64,
) -> impl IntoView {
  let overlay_left = if start_fret > min_fret {
    // Calculate the playable area for start_fret (the space where you press the string)
    let x_prev = if start_fret == 0 {
      0.0
    } else {
      positions[(start_fret - 1).max(0)]
    };
    let x_curr = positions[start_fret];
    // End overlay at the beginning of the playable area, not at the fret line
    let playable_area_start = (x_prev + x_curr) / 2.0 - (x_curr - x_prev) / 4.0;
    let start_x = to_viewbox_x(playable_area_start);
    let width = start_x - nut_width;
    Some(view! {
      <rect
        x=nut_width
        y=fret_margin
        width=width
        height=svg_height - 2.0 * fret_margin
        fill="#fff"
        opacity="0.35"
        style="pointer-events:none;"
      />
    })
  } else {
    None
  };

  let overlay_right = if end_fret < max_fret {
    let end_x = to_viewbox_x(positions[end_fret]);
    let width = svg_width - end_x;
    Some(view! {
      <rect
        x=end_x
        y=fret_margin
        width=width
        height=svg_height - 2.0 * fret_margin
        fill="#fff"
        opacity="0.35"
        style="pointer-events:none;"
      />
    })
  } else {
    None
  };

  view! {
    {overlay_left}
    {overlay_right}
  }
}

/// Renders invisible clickable areas over each fret position
#[component]
pub fn FretboardClickableAreas(
  /// Minimum visible fret number
  min_fret: usize,
  /// Maximum visible fret number
  max_fret: usize,
  /// Number of strings
  num_strings: u8,
  /// String spacing in SVG units
  string_spacing: f64,
  /// Absolute fret positions array
  positions: Vec<f64>,
  /// Function to transform absolute x to viewbox x
  to_viewbox_x: impl Fn(f64) -> f64 + Copy + 'static,
  /// Whether the nut is visible
  has_nut: bool,
  /// Width of the nut (only used if has_nut is true)
  nut_width: f64,
  /// Callback for fret clicks
  on_fret_clicked: Callback<FretClickEvent>,
) -> impl IntoView {
  view! {
    <>
      // Clickable areas for each fret position
      {(0..num_strings)
        .map(move |string_idx| {
          let string_y = (string_idx as f64 + 1.0) * string_spacing;
          view! {
            <>
              // Nut clickable area (fret 0) if visible
              {move || {
                if has_nut {
                  let on_click = move |_| {
                    on_fret_clicked
                      .run(FretClickEvent {
                        coord: FretCoord {
                          string_idx,
                          fret_idx: 0,
                        },
                      });
                  };
                  Some(

                    view! {
                      <rect
                        x="0"
                        y=string_y - string_spacing * 0.4
                        width=nut_width
                        height=string_spacing * 0.8
                        fill="transparent"
                        stroke="red"
                        stroke-width="1"
                        stroke-opacity="0.3"
                        on:click=on_click
                        style="cursor: pointer;"
                      ></rect>
                    },
                  )
                } else {
                  None
                }
              }} // Clickable areas for fretted positions
              {(min_fret.max(1)..=max_fret)
                .map(|fret_idx| {
                  let x_prev = if fret_idx == 0 { 0.0 } else { positions[(fret_idx - 1).max(0)] };
                  let x_curr = positions[fret_idx];
                  let x_start = to_viewbox_x((x_prev + x_curr) / 2.0 - (x_curr - x_prev) / 4.0);
                  let x_width = to_viewbox_x((x_prev + x_curr) / 2.0 + (x_curr - x_prev) / 4.0)
                    - x_start;
                  let on_click = move |_| {
                    on_fret_clicked
                      .run(FretClickEvent {
                        coord: FretCoord {
                          string_idx,
                          fret_idx: fret_idx as u8,
                        },
                      });
                  };

                  view! {
                    <rect
                      x=x_start
                      y=string_y - string_spacing * 0.4
                      width=x_width
                      height=string_spacing * 0.8
                      fill="transparent"
                      stroke="red"
                      stroke-width="1"
                      stroke-opacity="0.3"
                      on:click=on_click
                      style="cursor: pointer;"
                    />
                  }
                })
                .collect_view()}
            </>
          }
        })
        .collect_view()}
    </>
  }
}

#[component]
pub fn FretboardNotes(
  /// Minimum visible fret number
  min_fret: usize,
  /// Maximum visible fret number
  max_fret: usize,
  /// Absolute fret positions array
  positions: Vec<f64>,
  /// Function to transform absolute x to viewbox x
  to_viewbox_x: impl Fn(f64) -> f64 + Copy + 'static,
  /// Fret states for each string and fret
  frets: Signal<HashMap<FretCoord, Signal<FretState>>>,
  /// Number of strings (dynamic, replaces previous hard-coded 6)
  num_strings: u8,
  /// Spacing between strings (precomputed for consistency with other layers)
  string_spacing: f64,
  /// Width of the nut (needed to center open-string notes)
  nut_width: f64,
  /// Whether nut is visible (min fret == 0 in current zoom window)
  has_nut: bool,
) -> impl IntoView {
  let fret_states = frets.get();

  view! {
    <>
      {(0..num_strings)
        .map(move |string_idx| {
          // Group per string for future styling / interactions
          view! {
            <g class="string-group" data-string=string_idx>
              {(min_fret..=max_fret)
                .filter_map({
                  let positions = positions.clone();
                  let value = fret_states.clone();
                  move |fret| {
                    let coord = FretCoord { string_idx, fret_idx: fret as u8 };
                    let state_signal = value.get(&coord)?;
                    match state_signal.get() {
                      FretState::Hidden => None,
                      other_state => {
                        // Compute x centered in playable area (midpoint between previous and current fret lines)
                        let x = if fret == 0 {
                          // Place open-string note centered in nut if nut visible; else skip (outside view)
                          if has_nut { nut_width / 2.0 } else { return None; }
                        } else {
                          let x_prev = positions[fret - 1];
                          let x_curr = positions[fret];
                          let mid = (x_prev + x_curr) / 2.0;
                          to_viewbox_x(mid)
                        };
                        // Y based on provided string spacing
                        let y = (string_idx as f64 + 1.0) * string_spacing;
                        let (fill_owned, radius) = match other_state {
                          FretState::Hidden => unreachable!(),
                          FretState::Normal => ("red".to_string(), 6.0),
                          FretState::Colored(color) => (color.as_str().to_string(), 6.0),
                        };
                        Some(view! { <circle cx=x cy=y r=radius fill=fill_owned opacity="0.85" /> })
                      }
                    }
                  }
                })
                .collect_view()}
            </g>
          }
        })
        .collect_view()}
    </>
  }
}
