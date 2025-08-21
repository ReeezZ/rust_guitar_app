// (No HashMap needed in per-cell components currently.)

use std::{collections::HashMap, fmt::format};

use crate::fretboard::{
  components::base::{fretboard::FretClickEvent, helper::FretState, layout::LayoutSnapshot},
  model::FretCoord,
};

use leptos::prelude::*;

/// Renders the nut (zero fret) when visible
#[component]
pub(crate) fn FretboardNut(
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
pub(crate) fn FretboardFrets(layout: LayoutSnapshot) -> impl IntoView {
  (layout.min_fret..=layout.max_fret)
    .map(|fret_no| {
      let absolute_x = layout.positions[fret_no];
      let x_pos = layout.absolute_to_viewbox_x(absolute_x);
      let is_playable = fret_no >= layout.start_fret && fret_no <= layout.end_fret;
      let color = if is_playable { "#444" } else { "#bbb" };
      let width = if is_playable { "5" } else { "3" };
      view! {
        <line
          x1=x_pos
          y1=layout.fret_margin
          x2=x_pos
          y2=layout.svg_height - layout.fret_margin
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
pub(crate) fn FretboardStrings(
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
pub(crate) fn FretboardMarkers(layout: LayoutSnapshot, marker_positions: Vec<u8>) -> impl IntoView {
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
pub(crate) fn FretboardOverlays(layout: LayoutSnapshot) -> impl IntoView {
  let overlay_left = if layout.start_fret > layout.min_fret {
    let x_prev = if layout.start_fret == 0 {
      0.0
    } else {
      layout.positions[(layout.start_fret - 1).max(0)]
    };
    let x_curr = layout.positions[layout.start_fret];
    let playable_area_start = (x_prev + x_curr) / 2.0 - (x_curr - x_prev) / 4.0;
    let start_x = layout.absolute_to_viewbox_x(playable_area_start);
    let width = start_x - layout.effective_nut_width();
    Some(view! {
      <rect
        x=layout.effective_nut_width()
        y=layout.fret_margin
        width=width
        height=layout.svg_height - 2.0 * layout.fret_margin
        fill="#fff"
        opacity="0.35"
        style="pointer-events:none;"
      />
    })
  } else {
    None
  };

  let overlay_right = if layout.end_fret < layout.max_fret {
    let end_x = layout.absolute_to_viewbox_x(layout.positions[layout.end_fret]);
    let width = layout.svg_width - end_x;
    Some(view! {
      <rect
        x=end_x
        y=layout.fret_margin
        width=width
        height=layout.svg_height - 2.0 * layout.fret_margin
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

/// Single clickable area for one fret/string coordinate.
#[component]
fn FretboardClickableArea(layout: LayoutSnapshot, coord: FretCoord) -> impl IntoView {
  let string_y = layout.string_y(coord.string_idx);
  let (x, width) = if coord.fret_idx == 0 {
    // Nut rectangle
    if layout.has_nut {
      (0.0, layout.nut_width)
    } else {
      (0.0, 0.0)
    }
  } else {
    let prev = layout.positions[(coord.fret_idx as usize - 1).max(0)];
    let curr = layout.positions[coord.fret_idx as usize];
    let start = layout.absolute_to_viewbox_x((prev + curr) / 2.0 - (curr - prev) / 4.0);
    let end = layout.absolute_to_viewbox_x((prev + curr) / 2.0 + (curr - prev) / 4.0);
    (start, end - start)
  };
  view! {
    <rect
      x=x
      y=string_y - layout.string_spacing * 0.4
      width=width
      height=layout.string_spacing * 0.8
      fill="transparent"
      stroke="red"
      stroke-width="1"
      stroke-opacity="0.3"
    />
  }
}

/// Single note (circle + optional label) at a fret/string coordinate.
#[component]
fn FretboardNote(
  layout: Signal<LayoutSnapshot>,
  coord: FretCoord,
  state: Signal<FretState>,
) -> impl IntoView {
  let position = Signal::derive(move || layout.get().note_position(coord));
  move || {
    let (x, y) = match position.get() {
      Some(p) => p,
      None => return None,
    };
    let current_state = state.get();
    let (fill_color, radius, label) = match current_state {
      FretState::Hidden => ("transparent".to_string(), 0.0, None),
      FretState::Normal(color, label) => (color.as_str().to_string(), 12.0, Some(label)),
    };

    Some(view! {
      <g class="note" data-string=coord.string_idx data-fret=coord.fret_idx>
        {if radius > 0.0 {
          Some(view! { <circle cx=x cy=y r=radius fill=fill_color opacity="0.85" /> })
        } else {
          None
        }}
        {label
          .map(|text| {
            view! {
              <text
                x=x
                y=y
                text-anchor="middle"
                dominant-baseline="central"
                fill="white"
                font-size="8"
                font-weight="bold"
              >
                // style="pointer-events:none;user-select:none;"
                {text}
              </text>
            }
          })}
      </g>
    })
  }
}

#[component]
pub(crate) fn FretboardGrid(
  #[prop(into)] layout: Signal<LayoutSnapshot>,
  fret_states: Signal<HashMap<FretCoord, Signal<FretState>>>,
  /// Optional callback for fret click events
  click_cb: Option<Callback<FretClickEvent>>,
) -> impl IntoView {
  view! {
    <g class="interactive-layer">
      {
        let layout = layout.clone();
        move || {
          let states_map = fret_states.clone();
          (layout.get().min_fret..=layout.get().max_fret)
            .flat_map(|fret_idx| {
              let layout_cell = layout.clone();
              (0..layout.get().num_strings)
                .map(move |string_idx| {
                  let coord = FretCoord {
                    string_idx,
                    fret_idx: fret_idx as u8,
                  };
                  let state_sig_opt = states_map.get().get(&coord).cloned();
                  view! {
                    <g
                      class="cell-group"
                      data-fret=fret_idx
                      data-string=string_idx
                      style=format!(
                        "cursor: {};",
                        if click_cb.is_some() { "pointer" } else { "default" },
                      )
                      on:click=move |_| {
                        if let Some(click_cb) = click_cb.as_ref() {
                          click_cb.run(FretClickEvent { coord });
                        }
                      }
                    >
                      {move || {
                        click_cb
                          .as_ref()
                          .map(|_| {
                            view! {
                              <FretboardClickableArea layout=layout_cell.get() coord=coord />
                            }
                          })
                      }}
                      {state_sig_opt
                        .map(|state_signal| {
                          view! {
                            <FretboardNote
                              layout=layout_cell.clone()
                              coord=coord
                              state=state_signal
                            />
                          }
                        })}
                    </g>
                  }
                })
            })
            .collect_view()
        }
      }
    </g>
  }
}
