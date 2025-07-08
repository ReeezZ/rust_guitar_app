use crate::fretboard_view_helper::calculate_fret_positions;
use leptos::prelude::*;
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
  let UseWindowSizeReturn { width, height: _ } = use_window_size();

  // Calculate SVG dimensions based on window size, maintaining an aspect ratio.
  let svg_width = Memo::new(move |_| width.get() * SVG_WIDTH_RATIO);
  let svg_height = Memo::new(move |_| svg_width.get() / SVG_ASPECT_RATIO);

  let num_strings = 6;
  let fret_margin = Memo::new(move |_| svg_height.get() * FRET_MARGIN_PERCENTAGE);

  view! {
    <svg
      width=move || svg_width.get()
      height=move || svg_height.get()
      viewBox=move || {
        let total_frets = num_frets;
        let start = start_fret.get();
        let end = end_fret.get();
        let playable_width = (svg_width.get() - NUT_WIDTH) * 1.5;
        let fret_positions = calculate_fret_positions(playable_width, total_frets as u8);
        let min_fret = if start as f64 > EXTRA_FRETS {
          (start as f64 - EXTRA_FRETS).floor() as usize
        } else {
          0
        };
        let max_fret = ((end as f64 + EXTRA_FRETS).ceil() as usize).min(total_frets);
        let x_min = fret_positions[min_fret];
        let x_max = fret_positions[max_fret];
        let width = x_max - x_min + NUT_WIDTH;
        format!("{} {} {} {}", x_min, 0, width, svg_height.get())
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
        let playable_width = (current_svg_width - NUT_WIDTH) * 1.5;
        let fret_positions = calculate_fret_positions(playable_width, total_frets as u8);
        let min_fret = if start as f64 > EXTRA_FRETS {
          (start as f64 - EXTRA_FRETS).floor() as usize
        } else {
          0
        };
        let max_fret = ((end as f64 + EXTRA_FRETS).ceil() as usize).min(total_frets);
        let nut = if min_fret == 0 {
          Some(

            // Draw nut if visible
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
        // Draw frets: visually distinguish between playable (selected) and non-playable frets
        let frets = (min_fret..=max_fret)
          .map(|fret_no| {
            let x_pos_relative_to_nut = fret_positions[fret_no as usize];
            let x_pos = x_pos_relative_to_nut + NUT_WIDTH;
            let is_playable = fret_no >= start && fret_no <= end;
            let color = if is_playable { "#444" } else { "#bbb" };
            let width = if is_playable { "5" } else { "3" };
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

            // Draw strings
            view! {
              <line
                x1="0"
                y1=y_pos
                x2=current_svg_width
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
            let x_prev_fret_relative = fret_positions[(fret - 1).max(0) as usize];
            let x_curr_fret_relative = fret_positions[fret as usize];
            let x_relative = (x_prev_fret_relative + x_curr_fret_relative) / 2.0;
            let x = x_relative + NUT_WIDTH;
            let y = current_svg_height / 2.0;
            let r = if fret == 12 || fret == 24 { 8.0 } else { 6.0 };
            let y_offset = 28.0;
            let (cy1, cy2, op2) = if fret == 12 || fret == 24 {
              (y - y_offset, y + y_offset, 0.25)
            } else {
              (y, y + y_offset, 0.0)
            };

            // Draw markers (dots) for visible frets
            view! {
              <g>
                <circle cx=x cy=cy1 r=r fill="#444" opacity="0.25" />
                <circle cx=x cy=cy2 r=r fill="#444" opacity=op2 />
              </g>
            }
          })
          .collect_view();
        // Always draw the left overlay if min_fret > 0 (covers everything before the visible range, including the nut)
        let overlay_left = if min_fret > 0 {
          let x = 0.0;
          let width = fret_positions[min_fret] + NUT_WIDTH;
          Some(
            view! {
              <rect
                x=x
                y=current_fret_margin
                width=width
                height=current_svg_height - 2.0 * current_fret_margin
                fill="#888"
                opacity="0.45" // Stronger opacity for clearer distinction
                style="pointer-events:none;"
                stroke="#444"
                stroke-dasharray="8,4"
              />
            },
          )
        } else {
          None
        };
        // Overlay for non-playable (non-selected) frets on the left
        let playable_x_start = fret_positions[start] + NUT_WIDTH;
        let overlay_left_playable = if start > min_fret {
          let x = fret_positions[min_fret] + NUT_WIDTH;
          let width = playable_x_start - x;
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
        // Overlay for non-playable frets on the right
        let playable_x_end = fret_positions[end] + NUT_WIDTH;
        let overlay_right_playable = if max_fret > end {
          let x = playable_x_end;
          let width = fret_positions[max_fret] + NUT_WIDTH - x;
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
        // Overlay for non-playable frets on the right (grey box after the white box)
        let overlay_right_grey = if max_fret > end {
          let x = fret_positions[end] + NUT_WIDTH + (fret_positions[max_fret] + NUT_WIDTH - (fret_positions[end] + NUT_WIDTH));
          let width = current_svg_width - x;
          if width > 0.0 {
            Some(
              view! {
                <rect
                  x=x
                  y=current_fret_margin
                  width=width
                  height=current_svg_height - 2.0 * current_fret_margin
                  fill="#888"
                  opacity="0.45"
                  style="pointer-events:none;"
                  stroke="#444"
                  stroke-dasharray="8,4"
                />
              },
            )
          } else {
            None
          }
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
          {overlay_right_grey}
        }
      }}
    </svg>
  }
}
