use crate::fretboard_view_helper::calculate_fret_positions;
use leptos::{logging, prelude::*};
use leptos_use::{use_window_size, UseWindowSizeReturn};

/// A responsive SVG fretboard that displays a zoomed-in range of frets.
///
/// - Only the selected range (with 1.5 extra frets on each side, if possible) is shown.
/// - Areas outside the selected range are grayed out.
/// - The nut is always shown if the visible range includes fret 0.
/// - The view is zoomed to fill the SVG width with the selected range.
///
/// # Props
/// - `num_frets`: Total number of frets on the fretboard.
/// - `start_fret`: First fret in the selected range.
/// - `end_fret`: Last fret in the selected range (inclusive).
#[component]
pub fn SvgFretboard(start_fret: Signal<usize>, end_fret: Signal<usize>) -> impl IntoView {
  // Calculate the number of frets dynamically based on the highest end_fret value.
  let num_frets = end_fret.get().max(22); // Default to 22 if end_fret is lower, or use a different logic if needed.

  // Constants for SVG dimensions and scaling
  const SVG_WIDTH_RATIO: f64 = 0.9; // 90% of window width
  const SVG_ASPECT_RATIO: f64 = 3.0; // width / height
  const FRET_MARGIN_PERCENTAGE: f64 = 0.05; // 5% of svg_height
  const NUT_WIDTH: f64 = 14.0;
  const EXTRA_FRETS: f64 = 1.5;

  // Get reactive window dimensions.
  let UseWindowSizeReturn {
    width: window_width,
    height: _,
  } = use_window_size();

  // Calculate SVG dimensions based on window size, maintaining an aspect ratio.
  let svg_width = Memo::new(move |_| window_width.get() * SVG_WIDTH_RATIO);
  let svg_height = Memo::new(move |_| svg_width.get() / SVG_ASPECT_RATIO);

  let num_strings = 6;
  let fret_margin = Memo::new(move |_| svg_height.get() * FRET_MARGIN_PERCENTAGE);

  let fret_positions =
    Memo::new(move |_| calculate_fret_positions(svg_width.get(), num_frets as u8));

  let min_fret = Memo::new(move |_| {
    if start_fret.get() as f64 > EXTRA_FRETS {
      (start_fret.get() as f64 - EXTRA_FRETS).floor() as usize
    } else {
      0
    }
  });

  let max_fret =
    Memo::new(move |_| ((end_fret.get() as f64 + EXTRA_FRETS).ceil() as usize).min(num_frets));

  let start_pos = Memo::new(move |_| {
    let fret_pos = fret_positions.get();
    fret_pos[min_fret.get()]
  });
  let end_pos = Memo::new(move |_| {
    let fret_pos = fret_positions.get();
    fret_pos[max_fret.get()]
  });
  let fretboard_width = Memo::new(move |_| {
    let start_pos = start_pos.get();
    let end_pos = end_pos.get();
    end_pos - start_pos
  });

  view! {
    <svg
      width=move || svg_width.get()
      height=move || svg_height.get()
      viewBox=move || {
        let min_fret = if start_fret.get() as f64 > EXTRA_FRETS {
          (start_fret.get() as f64 - EXTRA_FRETS).floor() as usize
        } else {
          0
        };
        let max_fret = ((end_fret.get() as f64 + EXTRA_FRETS).ceil() as usize).min(num_frets);
        logging::log!(
          "min_fret: {}, max_fret: {}, Fret positions: {:?}",
          min_fret,
          max_fret,
          fret_positions
        );
        let x_min = fretboard_width.get();
        logging::log!("x_min: {}, width: {}", x_min, fretboard_width.get());
        format!("{} {} {} {}", start_pos.get(), 0, fretboard_width.get(), svg_height.get())
      }
      class="fretboard-svg"
      style="background: linear-gradient(90deg, #deb887 0%, #f5deb3 100%); border-radius: 8px; box-shadow: 0 2px 8px #0002; border: 1px solid #c00;"
    >
      {move || {
        let current_svg_width = svg_width.get();
        let current_svg_height = svg_height.get();
        let current_fret_margin = fret_margin.get();
        let string_spacing = current_svg_height / (num_strings as f64 + 1.0);
        let total_frets = num_frets;
        let start = start_fret.get();
        let end = end_fret.get();
        let full_svg_width = current_svg_width;
        let fret_positions = fret_positions.get();
        let min_fret = if start as f64 > EXTRA_FRETS {
          (start as f64 - EXTRA_FRETS).floor() as usize
        } else {
          0
        };
        let max_fret = ((end as f64 + EXTRA_FRETS).ceil() as usize).min(total_frets);
        let viewbox_width = if min_fret == 0 {
          full_svg_width
        } else {
          fret_positions[max_fret] - fret_positions[min_fret]
        };
        let nut = if min_fret == 0 {
          Some(

            // Key insight: Calculate fret positions for full width, nut is just positioned at start

            // ViewBox calculation for maximum space usage
            // When nut is visible: use full width
            // When nut is not visible: just the distance between visible frets
            // Draw nut at the leftmost position when visible
            view! {
              <rect
                x="0"
                y=current_fret_margin
                width=NUT_WIDTH
                height=current_svg_height - 2.0 * current_fret_margin
                fill="#f8f8f8"
                stroke="#222"
                stroke-width="5"
                rx="3"
              />
            },
          )
        } else {
          None
        };
        let frets = (min_fret..=max_fret)
          .map(|fret_no| {
            let x_pos_absolute = fret_positions[fret_no as usize];
            let x_pos = if min_fret == 0 {
              let remaining_width = full_svg_width - NUT_WIDTH;
              let scale_factor = remaining_width / full_svg_width;
              NUT_WIDTH + (x_pos_absolute * scale_factor)
            } else {
              x_pos_absolute - fret_positions[min_fret]
            };
            let is_playable = fret_no >= start && fret_no <= end;
            let color = if is_playable { "#444" } else { "#bbb" };
            let width = if is_playable { "5" } else { "3" };
            // Convert to viewBox coordinates
            // When nut is visible: offset fret positions to account for nut width
            // Scale the remaining space proportionally
            // When nut is not visible: fret position relative to min_fret

            view! {
              <line
                x1=x_pos
                y1=current_fret_margin
                x2=x_pos
                y2=current_svg_height - current_fret_margin
                stroke=color
                stroke-width=width
                opacity=if is_playable { "1.0" } else { "0.6" }
              />
            }
          })
          .collect_view();
        let strings = (0..num_strings)
          .map(|i| {
            let y_pos = (i as f64 + 1.0) * string_spacing;
            let string_thickness = 1.0 + (i as f64);

            // Draw strings across the visible viewBox area
            view! {
              <line
                x1="0"
                y1=y_pos
                // TODO we have to change this to use the max fret postion instead of full_svg_width
                x2=fretboard_width.get()
                y2=y_pos
                stroke="#888"
                stroke-width=string_thickness
              />
            }
          })
          .collect_view();
        let markers = (min_fret..=max_fret)
          .filter(|&fret| [3, 5, 7, 9, 12, 15, 17, 19, 21, 24].contains(&fret))
          .map(|fret| {
            let x_prev_fret_absolute = fret_positions[(fret - 1).max(0) as usize];
            let x_curr_fret_absolute = fret_positions[fret as usize];
            let x_absolute = (x_prev_fret_absolute + x_curr_fret_absolute) / 2.0;
            let x = if min_fret == 0 {
              let remaining_width = full_svg_width - NUT_WIDTH;
              let scale_factor = remaining_width / full_svg_width;
              NUT_WIDTH + (x_absolute * scale_factor)
            } else {
              x_absolute - fret_positions[min_fret]
            };
            let y = current_svg_height / 2.0;
            let r = if fret == 12 || fret == 24 { 8.0 } else { 6.0 };
            let y_offset = 28.0;
            let (cy1, cy2, op2) = if fret == 12 || fret == 24 {
              (y - y_offset, y + y_offset, 0.25)
            } else {
              (y, y + y_offset, 0.0)
            };
            // Convert to viewBox coordinates
            // When nut is visible: scale and offset marker position
            // When nut is not visible: marker position relative to min_fret

            view! {
              <g>
                <circle cx=x cy=cy1 r=r fill="#444" opacity="0.25" />
                <circle cx=x cy=cy2 r=r fill="#444" opacity=op2 />
              </g>
            }
          })
          .collect_view();
        let overlay_left = if min_fret > 0 {
          Some(
            // In the new coordinate system, show a small indicator at the left edge
            // to show that there are frets before the visible range
            view! {
              <rect
                x="0"
                y=current_fret_margin
                width="8"
                height=current_svg_height - 2.0 * current_fret_margin
                fill="#888"
                opacity="0.6"
                style="pointer-events:none;"
              />
              <text
                x="4"
                y=current_svg_height / 2.0
                text-anchor="middle"
                dominant-baseline="middle"
                font-size="12"
                fill="#fff"
                font-weight="bold"
              >
                "..."
              </text>
            },
          )
        } else {
          None
        };
        let playable_x_start = if min_fret == 0 {
          let remaining_width = full_svg_width - NUT_WIDTH;
          let scale_factor = remaining_width / full_svg_width;
          NUT_WIDTH + (fret_positions[start] * scale_factor)
        } else {
          fret_positions[start] - fret_positions[min_fret]
        };
        let overlay_left_playable = if start > min_fret {
          let x = if min_fret == 0 { NUT_WIDTH } else { 0.0 };
          let width = playable_x_start - x;
          Some(
            // Calculate playable range overlays in viewBox coordinates

            view! {
              <rect
                x=x
                y=current_fret_margin
                width=width
                height=current_svg_height - 2.0 * current_fret_margin
                fill="#fff"
                opacity="0.35"
                style="pointer-events:none;"
              />
            },
          )
        } else {
          None
        };
        let playable_x_end = if min_fret == 0 {
          let remaining_width = full_svg_width - NUT_WIDTH;
          let scale_factor = remaining_width / full_svg_width;
          NUT_WIDTH + (fret_positions[end] * scale_factor)
        } else {
          fret_positions[end] - fret_positions[min_fret]
        };
        let overlay_right_playable = if max_fret > end {
          let x = playable_x_end;
          let width = viewbox_width - x;
          Some(

            view! {
              <rect
                x=x
                y=current_fret_margin
                width=width
                height=current_svg_height - 2.0 * current_fret_margin
                fill="#fff"
                opacity="0.35"
                style="pointer-events:none;"
              />
            },
          )
        } else {
          None
        };
        let overlay_right_indicator = if max_fret < total_frets {
          Some(
            // Show right-side indicator if there are more frets beyond the visible range
            view! {
              <rect
                x=viewbox_width - 8.0
                y=current_fret_margin
                width="8"
                height=current_svg_height - 2.0 * current_fret_margin
                fill="#888"
                opacity="0.6"
                style="pointer-events:none;"
              />
              <text
                x=viewbox_width - 4.0
                y=current_svg_height / 2.0
                text-anchor="middle"
                dominant-baseline="middle"
                font-size="12"
                fill="#fff"
                font-weight="bold"
              >
                "..."
              </text>
            },
          )
        } else {
          None
        };

        view! {
          {nut}
          {frets}
          {strings}
          {markers}
          {overlay_left}
          {overlay_left_playable}
          {overlay_right_playable}
          {overlay_right_indicator}
        }
      }}
    </svg>
  }
}
