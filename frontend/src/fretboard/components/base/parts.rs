// (No HashMap needed in per-cell components currently.)

use crate::fretboard::{
  components::base::{
    helper::FretState,
    layout::{self, LayoutSnapshot},
  },
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
  layout: LayoutSnapshot,
  start_fret: Signal<usize>,
  end_fret: Signal<usize>,
  #[prop(into)] min_visible_fret: Signal<usize>,
  #[prop(into)] max_visible_fret: Signal<usize>,
) -> impl IntoView {
  view! {
    <For
      each=move || {
        leptos::logging::log!(
          "Rendering frets: min_visible_fret={}, max_visible_fret={}",
          min_visible_fret.get(),
          max_visible_fret.get(),
        );
        (min_visible_fret.get()..max_visible_fret.get()).collect::<Vec<_>>()
      }
      key=move |fret_no| (
        *fret_no,
        min_visible_fret.get(),
        max_visible_fret.get(),
        start_fret.get(),
        end_fret.get(),
      )
      let(fret_no)
    >
      {
        let layout = layout.clone();
        move || {
          let checked_fret_index = fret_no.checked_sub(min_visible_fret.get());
          leptos::logging::log!(
            "Rendering fret line: fret_no={}, min_visible_fret={:?}, max_visible_fret={:?}, checked_fret_index={:?}, start_fret={:?}, end_fret={:?}",
            fret_no,
            min_visible_fret.get(),
            max_visible_fret.get(),
            checked_fret_index,
            start_fret.get(),
            end_fret.get()
          );
          if let Some(index) = checked_fret_index {
            if index > max_visible_fret.get() {
              leptos::logging::log!("weird edge case Fret index {} out of bounds, skipping", index);
              return None;
            }
          } else {
            return None;
          };
          let absolute_x = layout.positions.get()[checked_fret_index.unwrap()];
          let x_pos = layout.absolute_to_viewbox_x(absolute_x);
          let is_playable = fret_no >= start_fret.get() && fret_no <= end_fret.get();
          let color = if is_playable { "#444" } else { "#bbb" };
          let width = if is_playable { "5" } else { "3" };
          Some(
            // return None;

            view! {
              <line
                x1=x_pos
                y1=layout.fret_margin.get()
                x2=x_pos
                y2=layout.svg_height.get() - layout.fret_margin.get()
                stroke=color
                stroke-width=width
                opacity=if is_playable { "1.0" } else { "0.6" }
              />
            },
          )
        }
      }
    </For>
  }
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
  view! {
    <For each=move || (0..num_strings) key=|string_no| *string_no let(string_no)>
      {
        let y_pos = (string_no as f64 + 1.0) * string_spacing;
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
  #[prop(into)] layout: LayoutSnapshot,
  #[prop(into)] marker_positions: Signal<Vec<usize>>,
  #[prop(into)] min_visible_fret: Signal<usize>,
  #[prop(into)] max_visible_fret: Signal<usize>,
) -> impl IntoView {
  view! {
    <For
      each=move || {
        (min_visible_fret.get()..max_visible_fret.get())
          .filter(move |fret| marker_positions.get().contains(fret))
      }
      key=|fret| *fret
      let(fret)
    >
      {move || {
        let checked_fret_index = fret.checked_sub(min_visible_fret.get());
        leptos::logging::log!(
          "Rendering fret line: fret_no={}, min_visible_fret={:?}, max_visible_fret={:?}, checked_fret_index={:?}",
            fret,
            min_visible_fret.get(),
            max_visible_fret.get(),
            checked_fret_index
        );
        let fret_index = match checked_fret_index {
          Some(index) => index,
          None => return None,
        };
        let x_prev = layout.positions.get()[(fret_index - 1).max(0)];
        let x_curr = layout.positions.get()[fret_index];
        let x_center = (x_prev + x_curr) / 2.0;
        let x = layout.absolute_to_viewbox_x(x_center);
        let y = layout.svg_height.get() / 2.0;
        let r = if fret_index == 12 || fret_index == 24 { 8.0 } else { 6.0 };
        let y_offset = 28.0;
        let (cy1, cy2, op2) = if fret_index == 12 || fret_index == 24 {
          (y - y_offset, y + y_offset, 0.25)
        } else {
          (y, y + y_offset, 0.0)
        };
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
        layout.positions.get()[(start_fret.get() - 1).max(0)]
      };
      let x_curr = layout.positions.get()[start_fret.get()];
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
      let end_x = layout.absolute_to_viewbox_x(layout.positions.get()[end_fret.get()]);
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
      let prev = layout.positions.get()[(coord.fret_idx as usize - 1).max(0)];
      let curr = layout.positions.get()[coord.fret_idx as usize];
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
  state: Signal<Option<FretState>>,
) -> impl IntoView {
  move || {
    let (x, y) = match layout.note_position(coord) {
      Some(p) => p,
      None => return None,
    };
    let current_state = match state.get() {
      Some(fret_state) => fret_state,
      None => return None,
    };

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
  #[prop(into)] layout: LayoutSnapshot,
  #[prop(into)] min_visible_fret: Signal<usize>,
  #[prop(into)] max_visible_fret: Signal<usize>,
  fret_states: Signal<FretStateSignals>,
  tuning: Signal<Vec<Note>>,
  /// Optional callback for fret click events
  click_cb: Signal<Option<Callback<FretClickEvent>>>,
) -> impl IntoView {
  view! {
    <For
      each=move || min_visible_fret.get()..=max_visible_fret.get()
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
          let fret_state = Memo::new(move |_| {
            fret_states
              .with(move |fret_states| {
                match fret_states.get(&coord) {
                  Some(fret_state) => Some(fret_state.clone()),
                  None => None,
                }
              })
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
              {move || {
                view! { <FretboardNote layout=layout coord=coord state=fret_state.into() /> }
              }}
            </g>
          }
        }
      </For>

    </For>
  }
}
