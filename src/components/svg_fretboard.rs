use crate::fretboard_view_helper::calculate_fret_positions;
use leptos::prelude::*;
use leptos_use::{use_window_size, UseWindowSizeReturn};

#[component]
pub fn SvgFretboard(start_fret: Signal<usize>, end_fret: Signal<usize>) -> impl IntoView {
  let num_frets = end_fret.get().max(22);
  
  const SVG_WIDTH_RATIO: f64 = 0.9;
  const SVG_ASPECT_RATIO: f64 = 3.0;
  const FRET_MARGIN_PERCENTAGE: f64 = 0.05;
  const NUT_WIDTH: f64 = 14.0;
  const EXTRA_FRETS: f64 = 1.5;

  let UseWindowSizeReturn { width: window_width, height: _ } = use_window_size();
  
  let svg_width = Memo::new(move |_| window_width.get() * SVG_WIDTH_RATIO);
  let svg_height = Memo::new(move |_| svg_width.get() / SVG_ASPECT_RATIO);
  let num_strings = 6;
  let fret_margin = Memo::new(move |_| svg_height.get() * FRET_MARGIN_PERCENTAGE);

  // Calculate fret positions for the FULL fretboard
  let full_fret_positions = Memo::new(move |_| calculate_fret_positions(svg_width.get(), num_frets as u8));

  // Calculate visible range
  let min_fret = Memo::new(move |_| {
    if start_fret.get() as f64 > EXTRA_FRETS {
      (start_fret.get() as f64 - EXTRA_FRETS).floor() as usize
    } else {
      0
    }
  });

  let max_fret = Memo::new(move |_| 
    ((end_fret.get() as f64 + EXTRA_FRETS).ceil() as usize).min(num_frets)
  );

  // KEY FIX: Calculate scaling parameters for zoom effect
  let zoom_params = Memo::new(move |_| {
    let positions = full_fret_positions.get();
    let min_f = min_fret.get();
    let max_f = max_fret.get();
    let current_svg_width = svg_width.get();
    
    // Physical range we want to display
    let range_start = if min_f == 0 { 0.0 } else { positions[min_f] };
    let range_end = positions[max_f];
    let range_width = range_end - range_start;
    
    // Available width for fret content (accounting for nut if visible)
    let available_width = if min_f == 0 {
      current_svg_width - NUT_WIDTH
    } else {
      current_svg_width
    };
    
    // Scale factor to make the selected range fill the available width
    let scale_factor = available_width / range_width;
    
    (range_start, scale_factor, min_f == 0)
  });

  // Transform absolute coordinates to scaled viewBox coordinates
  let to_viewbox_x = move |absolute_x: f64| -> f64 {
    let (range_start, scale_factor, has_nut) = zoom_params.get();
    let offset = if has_nut { NUT_WIDTH } else { 0.0 };
    offset + (absolute_x - range_start) * scale_factor
  };

  view! {
    <div class="flex justify-center items-center w-full">
      <svg
        width=move || svg_width.get()
        height=move || svg_height.get()
        viewBox=move || {
          let current_svg_width = svg_width.get();
          let current_svg_height = svg_height.get();
          // Always use the full SVG width for maximum zoom effect
          format!("0 0 {} {}", current_svg_width, current_svg_height)
        }
        class="fretboard-svg"
        style="background: linear-gradient(90deg, #deb887 0%, #f5deb3 100%); border-radius: 8px; box-shadow: 0 2px 8px #0002; border: 1px solid #c00;"
      >
      {move || {
        let current_svg_height = svg_height.get();
        let current_fret_margin = fret_margin.get();
        let string_spacing = current_svg_height / (num_strings as f64 + 1.0);
        let positions = full_fret_positions.get();
        let min_f = min_fret.get();
        let max_f = max_fret.get();
        let start = start_fret.get();
        let end = end_fret.get();
        let current_svg_width = svg_width.get();
        let (_, viewbox_width) = (0.0, current_svg_width);

        // Nut (only when fret 0 is visible)
        let nut = if min_f == 0 {
          Some(view! {
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
          })
        } else {
          None
        };

        // Frets
        let frets = (min_f..=max_f)
          .map(|fret_no| {
            let absolute_x = positions[fret_no];
            let x_pos = to_viewbox_x(absolute_x);
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

        // Strings - span the full viewBox width
        let strings = (0..num_strings)
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
          .collect_view();

        // Markers
        let markers = (min_f..=max_f)
          .filter(|&fret| [3, 5, 7, 9, 12, 15, 17, 19, 21, 24].contains(&fret))
          .map(|fret| {
            let x_prev = positions[(fret - 1).max(0)];
            let x_curr = positions[fret];
            let x_center = (x_prev + x_curr) / 2.0;
            let x = to_viewbox_x(x_center);
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

        // Overlays for non-playable regions
        let overlay_left = if start > min_f {
          let start_x = to_viewbox_x(positions[start]);
          let width = start_x - (if min_f == 0 { NUT_WIDTH } else { 0.0 });
          Some(view! {
            <rect
              x=if min_f == 0 { NUT_WIDTH } else { 0.0 }
              y=current_fret_margin
              width=width
              height=current_svg_height - 2.0 * current_fret_margin
              fill="#fff"
              opacity="0.35"
              style="pointer-events:none;"
            />
          })
        } else {
          None
        };

        let overlay_right = if end < max_f {
          let end_x = to_viewbox_x(positions[end]);
          let width = viewbox_width - end_x;
          Some(view! {
            <rect
              x=end_x
              y=current_fret_margin
              width=width
              height=current_svg_height - 2.0 * current_fret_margin
              fill="#fff"
              opacity="0.35"
              style="pointer-events:none;"
            />
          })
        } else {
          None
        };

        view! {
          {nut}
          {frets}
          {strings}
          {markers}
          {overlay_left}
          {overlay_right}
        }
      }}
    </svg>
    </div>
  }
}
