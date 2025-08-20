use std::collections::HashMap;

use crate::{
  components::fretboard::base::{fretboard::FretClickEvent, layout::LayoutSnapshot},
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
pub fn FretboardFrets(layout: LayoutSnapshot) -> impl IntoView {
  (layout.min_fret..=layout.max_fret)
    .map(|fret_no| {
      let absolute_x = layout.positions[fret_no];
      let x_pos = layout.absolute_to_viewbox_x(absolute_x);
      let is_playable = fret_no >= layout.start_fret && fret_no <= layout.end_fret;
      let color = if is_playable { "#444" } else { "#bbb" };
      let width = if is_playable { "5" } else { "3" };
      view! { <line
        x1=x_pos y1=layout.fret_margin x2=x_pos y2=layout.svg_height - layout.fret_margin
        stroke=color stroke-width=width opacity=if is_playable { "1.0" } else { "0.6" }
      /> }
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
pub fn FretboardMarkers(layout: LayoutSnapshot, marker_positions: Vec<u8>) -> impl IntoView {
  (layout.min_fret..=layout.max_fret)
    .filter(|&fret| marker_positions.contains(&(fret as u8)))
    .map(|fret| {
      let x_prev = layout.positions[(fret - 1).max(0)];
      let x_curr = layout.positions[fret];
      let x_center = (x_prev + x_curr) / 2.0;
      let x = layout.absolute_to_viewbox_x(x_center);
      let y = layout.svg_height / 2.0;
      let r = if fret == 12 || fret == 24 { 8.0 } else { 6.0 };
      let y_offset = 28.0;
      let (cy1, cy2, op2) = if fret == 12 || fret == 24 { (y - y_offset, y + y_offset, 0.25) } else { (y, y + y_offset, 0.0) };
      view! { <g>
        <circle cx=x cy=cy1 r=r fill="#444" opacity="0.25" />
        <circle cx=x cy=cy2 r=r fill="#444" opacity=op2 />
      </g> }
    })
    .collect_view()
}

/// Renders semi-transparent overlays for non-playable regions
#[component]
pub fn FretboardOverlays(layout: LayoutSnapshot) -> impl IntoView {
  let overlay_left = if layout.start_fret > layout.min_fret {
    let x_prev = if layout.start_fret == 0 { 0.0 } else { layout.positions[(layout.start_fret - 1).max(0)] };
    let x_curr = layout.positions[layout.start_fret];
    let playable_area_start = (x_prev + x_curr) / 2.0 - (x_curr - x_prev) / 4.0;
    let start_x = layout.absolute_to_viewbox_x(playable_area_start);
    let width = start_x - layout.effective_nut_width();
    Some(view! { <rect
      x=layout.effective_nut_width() y=layout.fret_margin width=width
      height=layout.svg_height - 2.0 * layout.fret_margin fill="#fff" opacity="0.35"
      style="pointer-events:none;" /> })
  } else { None };

  let overlay_right = if layout.end_fret < layout.max_fret {
    let end_x = layout.absolute_to_viewbox_x(layout.positions[layout.end_fret]);
    let width = layout.svg_width - end_x;
    Some(view! { <rect
      x=end_x y=layout.fret_margin width=width height=layout.svg_height - 2.0 * layout.fret_margin
      fill="#fff" opacity="0.35" style="pointer-events:none;" /> })
  } else { None };

  view! { {overlay_left} {overlay_right} }
}

/// Renders invisible clickable areas over each fret position
#[component]
pub fn FretboardClickableAreas(layout: LayoutSnapshot, on_fret_clicked: Callback<FretClickEvent>) -> impl IntoView {
  view! { <>
    {(0..layout.num_strings).map(move |string_idx| {
      let string_y = (string_idx as f64 + 1.0) * layout.string_spacing;
      view! { <>
        {move || if layout.has_nut { // nut clickable
          let on_click = move |_| on_fret_clicked.run(FretClickEvent { coord: FretCoord { string_idx, fret_idx: 0 }});
          Some(view! { <rect x="0" y=string_y - layout.string_spacing * 0.4 width=layout.nut_width height=layout.string_spacing * 0.8
            fill="transparent" stroke="red" stroke-width="1" stroke-opacity="0.3" on:click=on_click style="cursor: pointer;" /> })
        } else { None }}
        {(layout.min_fret.max(1)..=layout.max_fret).map(|fret_idx| {
          let x_prev = if fret_idx == 0 { 0.0 } else { layout.positions[(fret_idx - 1).max(0)] };
          let x_curr = layout.positions[fret_idx];
          let x_start = layout.absolute_to_viewbox_x((x_prev + x_curr) / 2.0 - (x_curr - x_prev) / 4.0);
            let x_width = layout.absolute_to_viewbox_x((x_prev + x_curr) / 2.0 + (x_curr - x_prev) / 4.0) - x_start;
          let on_click = move |_| on_fret_clicked.run(FretClickEvent { coord: FretCoord { string_idx, fret_idx: fret_idx as u8 }});
          view! { <rect x=x_start y=string_y - layout.string_spacing * 0.4 width=x_width height=layout.string_spacing * 0.8
            fill="transparent" stroke="red" stroke-width="1" stroke-opacity="0.3" on:click=on_click style="cursor: pointer;" /> }
        }).collect_view()}
      </> }
    }).collect_view()}
  </> }
}

#[component]
pub fn FretboardNotes(
  layout: LayoutSnapshot,
  frets: Signal<HashMap<FretCoord, Signal<FretState>>>,
) -> impl IntoView {
  let fret_states = frets.get();
  let min_fret = layout.min_fret;
  let max_fret = layout.max_fret;
  let num_strings = layout.num_strings;

  view! {
    <g class="notes-layer">
      {(0..num_strings)
        .map(move |string_idx| {
          view! { <g class="string-group" data-string=string_idx>
            {(min_fret..=max_fret)
              .filter_map({
                let value = fret_states.clone();
                let layout = layout.clone();
                move |fret| {
                  let coord = FretCoord { string_idx, fret_idx: fret as u8 };
                  let state_signal = value.get(&coord)?;
                  match state_signal.get() {
                    FretState::Hidden => None,
                    other_state => {
                      let (x, y) = layout.note_position(coord)?;
                      let (fill, radius) = match other_state {
                        FretState::Hidden => unreachable!(),
                        FretState::Normal => ("red".to_string(), 6.0),
                        FretState::Colored(color) => (color.as_str().to_string(), 6.0),
                      };
                      Some(view! { <circle cx=x cy=y r=radius fill=fill opacity="0.85" /> })
                    }
                  }
                }
              })
              .collect_view()}
          </g> }
        })
        .collect_view()}
    </g>
  }
}
