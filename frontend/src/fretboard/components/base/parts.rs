use crate::fretboard::{
  components::base::{helper::FretState, layout::LayoutSnapshot},
  fretboard_model::{FretClickEvent, FretCoord, FretStateSignals},
};

use leptos::prelude::*;
use shared::Note;

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
pub(crate) fn FretboardFrets(
  start_fret: Signal<usize>,
  end_fret: Signal<usize>,
  #[prop(into)] min_visible_fret: Signal<usize>,
  #[prop(into)] max_visible_fret: Signal<usize>,
  viewbox_positions: Signal<Vec<f64>>,
  #[prop(into)] fret_margin: Signal<f64>,
  #[prop(into)] svg_height: Signal<f64>,
) -> impl IntoView {
  view! {
    <For
      each=move || { min_visible_fret.get()..max_visible_fret.get() }
      key=move |fret_no| (*fret_no)
      let(fret_no)
    >
      {move || {
        let is_out_of_bounds = Memo::new(move |_| { fret_no >= viewbox_positions.get().len() });
        if is_out_of_bounds.get() {
          leptos::logging::warn!("Skipping fret line for fret {} as out of bounds", fret_no);
          return None;
        }
        leptos::logging::log!("Rendering fret line for fret {}", fret_no);
        let x_pos = Memo::new(move |_| viewbox_positions.get()[fret_no]);
        let is_playable = Memo::new(move |_| {
          fret_no >= start_fret.get() && fret_no <= end_fret.get()
        });
        let color = Signal::derive(move || if is_playable.get() { "#444" } else { "#bbb" });
        let width = Signal::derive(move || if is_playable.get() { "5" } else { "3" });
        Some(

          view! {
            <line
              x1=x_pos
              y1=fret_margin
              x2=x_pos
              y2=move || svg_height.get() - fret_margin.get()
              stroke=color
              stroke-width=width
              opacity=move || if is_playable.get() { "1.0" } else { "0.6" }
            />
          },
        )
      }}
    </For>
  }
}

/// Renders horizontal string lines
#[component]
pub(crate) fn FretboardStrings(
  /// Number of strings on the instrument
  #[prop(into)]
  num_strings: Signal<u8>,
  /// Spacing between strings
  #[prop(into)]
  string_spacing: Signal<f64>,
  /// Total viewbox width
  viewbox_width: f64,
) -> impl IntoView {
  view! {
    <For
      each=move || (0..num_strings.get())
      key=move |string_no| (*string_no, string_spacing.get().round() as isize)
      let(string_no)
    >
      {
        let y_pos = (string_no as f64 + 1.0) * string_spacing.get();
        let string_thickness = 1.0 + (string_no as f64);

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
      }
    </For>
  }
}

/// Renders fret position markers (dots)
#[component]
pub(crate) fn FretboardMarkers(
  #[prop(into)] svg_height: Signal<f64>,
  #[prop(into)] viewbox_positions: Signal<Vec<f64>>,
  #[prop(into)] marker_positions: Signal<Vec<usize>>,
  min_visible_fret: usize,
  max_visible_fret: usize,
) -> impl IntoView {
  view! {
    <For
      each=move || {
        (min_visible_fret..max_visible_fret)
          .filter(move |fret| marker_positions.get().contains(fret))
      }
      key=move |fret| (*fret, min_visible_fret, max_visible_fret)
      let(fret)
    >
      {move || {
        let is_out_of_bounds = Memo::new(move |_| { fret >= viewbox_positions.get().len() });
        if is_out_of_bounds.get() {
          leptos::logging::warn!("Skipping marker for fret {} as out of bounds", fret);
          return None;
        }
        let x = Memo::new(move |_| {
          let viewbox_positions = viewbox_positions.get();
          let x_prev = viewbox_positions[fret.checked_sub(1).unwrap_or(0)];
          let x_curr = viewbox_positions[fret];
          (x_prev + x_curr) / 2.0
        });
        let r = if fret == 12 || fret == 24 { 8.0 } else { 6.0 };
        let y_coords_and_opacity = Signal::derive(move || {
          let y = svg_height.get() / 2.0;
          let y_offset = 28.0;
          if fret == 12 || fret == 24 {
            (y - y_offset, y + y_offset, 0.25)
          } else {
            (y, y + y_offset, 0.0)
          }
        });
        let cy1 = Signal::derive(move || y_coords_and_opacity.get().0);
        let cy2 = Signal::derive(move || y_coords_and_opacity.get().1);
        let op2 = Signal::derive(move || y_coords_and_opacity.get().2);
        Some(

          view! {
            <g>
              <circle cx=x cy=cy1 r=r fill="#444" opacity="0.25" />
              <circle cx=x cy=cy2 r=r fill="#444" opacity=op2 />
            </g>
          },
        )
      }}
    </For>
  }
}

/// Renders semi-transparent overlays for non-playable regions
#[component]
pub(crate) fn FretboardOverlays(
  layout: LayoutSnapshot,
  start_fret: Signal<usize>,
  end_fret: Signal<usize>,
  #[prop(into)] min_visible_fret: Signal<usize>,
  #[prop(into)] max_visible_fret: Signal<usize>,
) -> impl IntoView {
  let layout_clone = layout.clone();
  let overlay_left = move || {
    if start_fret.get() > min_visible_fret.get() {
      let x_prev = if start_fret.get() == 0 {
        0.0
      } else {
        layout.absolute_positions.get()[(start_fret.get() - 1).max(0)]
      };
      let x_curr = layout.absolute_positions.get()[start_fret.get()];
      let playable_area_start = (x_prev + x_curr) / 2.0 - (x_curr - x_prev) / 4.0;
      let start_x = layout.absolute_to_viewbox_x(playable_area_start);
      let width = start_x - layout.effective_nut_width();
      Some(view! {
        <rect
          x=layout.effective_nut_width()
          y=layout.fret_margin.get()
          width=width
          height=layout.svg_height.get() - 2.0 * layout.fret_margin.get()
          fill="#fff"
          opacity="0.35"
          style="pointer-events:none;"
        />
      })
    } else {
      None
    }
  };

  let layout = layout_clone;
  let overlay_right = move || {
    if end_fret.get() < max_visible_fret.get() {
      let end_x = layout.absolute_to_viewbox_x(layout.absolute_positions.get()[end_fret.get()]);
      let width = layout.svg_width.get() - end_x;
      Some(view! {
        <rect
          x=end_x
          y=layout.fret_margin.get()
          width=width
          height=layout.svg_height.get() - 2.0 * layout.fret_margin.get()
          fill="#fff"
          opacity="0.35"
          style="pointer-events:none;"
        />
      })
    } else {
      None
    }
  };

  view! {
    {overlay_left}
    {overlay_right}
  }
}

/// Single clickable area for one fret/string coordinate.
#[component]
fn FretboardClickableArea(layout: LayoutSnapshot, coord: FretCoord) -> impl IntoView {
  let string_y = Memo::new(move |_| layout.string_y(coord.string_idx));
  let x_and_width = Memo::new(move |_| {
    if coord.fret_idx == 0 {
      // Nut rectangle
      if layout.has_nut.get() {
        (0.0, layout.nut_width.get())
      } else {
        (0.0, 0.0)
      }
    } else {
      let prev = layout.absolute_positions.get()[(coord.fret_idx as usize - 1).max(0)];
      let curr = layout.absolute_positions.get()[coord.fret_idx as usize];
      let start = layout.absolute_to_viewbox_x((prev + curr) / 2.0 - (curr - prev) / 4.0);
      let end = layout.absolute_to_viewbox_x((prev + curr) / 2.0 + (curr - prev) / 4.0);
      (start, end - start)
    }
  });
  let x = Memo::new(move |_| x_and_width.get().0);
  let width = Memo::new(move |_| x_and_width.get().1);
  let height = Memo::new(move |_| layout.string_spacing.get() * 0.8);
  let y = Memo::new(move |_| string_y.get() - layout.string_spacing.get() * 0.4);
  view! {
    <rect
      x=x
      y=y
      width=width
      height=height
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
  layout: LayoutSnapshot,
  coord: FretCoord,
  #[prop(into)] fret_state: Signal<FretState>,
) -> impl IntoView {
  move || {
    let (x, y) = match layout.note_position(coord) {
      Some(p) => p,
      None => return None,
    };

    let memo = Memo::new(move |_| match fret_state.get() {
      FretState::Hidden => ("transparent".to_string(), 0.0, None),
      FretState::Normal(color, label) => (color.as_str().to_string(), 12.0, Some(label)),
    });
    let fill_color = Signal::derive(move || memo.get().0);
    let radius = Signal::derive(move || memo.get().1);
    let label = Signal::derive(move || memo.get().2);

    Some(view! {
      <g class="note" data-string=coord.string_idx data-fret=coord.fret_idx>
        {if radius.get() > 0.0 {
          Some(view! { <circle cx=x cy=y r=radius.get() fill=fill_color.get() opacity="0.85" /> })
        } else {
          None
        }}
        {label
          .get()
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
  #[prop(into)] layout: LayoutSnapshot,
  #[prop(into)] min_visible_fret: Signal<usize>,
  #[prop(into)] max_visible_fret: Signal<usize>,
  fret_states: Signal<FretStateSignals>,
  tuning: Signal<Vec<Note>>,
  /// Optional callback for fret click events
  #[prop(into)]
  click_cb: Signal<Option<Callback<FretClickEvent>>>,
) -> impl IntoView {
  view! {
    <For
      each=move || min_visible_fret.get()..max_visible_fret.get()
      key=|fret_idx| *fret_idx
      let(fret_idx)
    >
      <For
        each=move || (0..layout.num_strings.get())
        key=move |string_idx| (*string_idx as usize, fret_idx)
        let(string_idx)
      >
        {
          let coord = FretCoord {
            string_idx,
            fret_idx: fret_idx as u8,
          };
          let fret_state = fret_states
            .with_untracked(|borrowed| {
              borrowed
                .get(&coord)
                .expect(
                  format!("Preallocated signals for all coordinates. Tried accessing {:?}", coord)
                    .as_str(),
                )
                .clone()
            });
          let handle_click = move |_| {
            if let Some(click_cb) = click_cb.get().as_ref() {
              let note = tuning
                .get()
                .get(string_idx as usize)
                .expect("Bounds checking on model construction")
                .add_steps(fret_idx);
              click_cb.run(FretClickEvent { coord, note });
            }
          };
          view! {
            <g
              class="cell-group"
              data-fret=fret_idx
              data-string=string_idx
              style=format!(
                "cursor: {};",
                if click_cb.get().is_some() { "pointer" } else { "default" },
              )
              on:click=handle_click
            >
              {move || {
                if click_cb.get().is_some() {
                  Some(view! { <FretboardClickableArea layout=layout coord=coord /> })
                } else {
                  None
                }
              }}
              <FretboardNote layout=layout coord=coord fret_state />
            </g>
          }
        }
      </For>

    </For>
  }
}
