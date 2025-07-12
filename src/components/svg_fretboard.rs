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
        let full_svg_width = svg_width.get();
        
        // Key insight: Calculate fret positions for full width, nut is just positioned at start
        let fret_positions = calculate_fret_positions(full_svg_width, total_frets as u8);
        let min_fret = if start as f64 > EXTRA_FRETS {
          (start as f64 - EXTRA_FRETS).floor() as usize
        } else {
          0
        };
        let max_fret = ((end as f64 + EXTRA_FRETS).ceil() as usize).min(total_frets);
        
        // ViewBox calculation for maximum space usage
        let (x_min, width) = if min_fret == 0 { 
          // When nut is visible: start at 0, use full width
          (0.0, full_svg_width)
        } else { 
          // When nut is not visible: start at min_fret position, end at max_fret position
          let start_pos = fret_positions[min_fret];
          let end_pos = fret_positions[max_fret];
          (start_pos, end_pos - start_pos)
        };
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
        let full_svg_width = current_svg_width;
        
        // Key insight: Calculate fret positions for full width, nut is just positioned at start
        let fret_positions = calculate_fret_positions(full_svg_width, total_frets as u8);
        let min_fret = if start as f64 > EXTRA_FRETS {
          (start as f64 - EXTRA_FRETS).floor() as usize
        } else {
          0
        };
        let max_fret = ((end as f64 + EXTRA_FRETS).ceil() as usize).min(total_frets);
        
        // ViewBox calculation for maximum space usage
        let viewbox_width = if min_fret == 0 { 
          // When nut is visible: use full width
          full_svg_width
        } else { 
          // When nut is not visible: just the distance between visible frets
          fret_positions[max_fret] - fret_positions[min_fret]
        };
        let nut = if min_fret == 0 {
          Some(
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
            // Convert to viewBox coordinates
            let x_pos = if min_fret == 0 {
              // When nut is visible: offset fret positions to account for nut width
              // Scale the remaining space proportionally
              let remaining_width = full_svg_width - NUT_WIDTH;
              let scale_factor = remaining_width / full_svg_width;
              NUT_WIDTH + (x_pos_absolute * scale_factor)
            } else {
              // When nut is not visible: fret position relative to min_fret
              x_pos_absolute - fret_positions[min_fret]
            };
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

            // Draw strings across the visible viewBox area
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
          .collect_view();
        let markers = (min_fret..=max_fret)
          .filter(|&fret| [3, 5, 7, 9, 12, 15, 17, 19, 21, 24].contains(&fret))
          .map(|fret| {
            let x_prev_fret_absolute = fret_positions[(fret - 1).max(0) as usize];
            let x_curr_fret_absolute = fret_positions[fret as usize];
            let x_absolute = (x_prev_fret_absolute + x_curr_fret_absolute) / 2.0;
            // Convert to viewBox coordinates
            let x = if min_fret == 0 {
              // When nut is visible: scale and offset marker position
              let remaining_width = full_svg_width - NUT_WIDTH;
              let scale_factor = remaining_width / full_svg_width;
              NUT_WIDTH + (x_absolute * scale_factor)
            } else {
              // When nut is not visible: marker position relative to min_fret
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

            view! {
              <g>
                <circle cx=x cy=cy1 r=r fill="#444" opacity="0.25" />
                <circle cx=x cy=cy2 r=r fill="#444" opacity=op2 />
              </g>
            }
          })
          .collect_view();
        let overlay_left = if min_fret > 0 {
          // In the new coordinate system, show a small indicator at the left edge
          // to show that there are frets before the visible range
          Some(
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
        // Calculate playable range overlays in viewBox coordinates
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
        // Show right-side indicator if there are more frets beyond the visible range
        let overlay_right_indicator = if max_fret < total_frets {
          Some(
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
