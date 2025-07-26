use crate::fretboard_view_helper::calculate_fret_positions;
use leptos::prelude::*;
use leptos_use::{use_window_size, UseWindowSizeReturn};

/// Configuration for fretboard visual appearance and behavior
#[derive(Clone, Debug)]
pub struct FretboardConfig {
    /// Number of guitar strings (typically 6 for standard guitar, 4 for bass, 7 for extended range)
    pub num_strings: u8,
    /// Maximum number of frets to display (typically 22-24 for electric guitars)
    pub max_frets: usize,
    /// Ratio of SVG width to window width (0.0-1.0, default 0.9 = 90% of window width)
    pub svg_width_ratio: f64,
    /// Width-to-height aspect ratio of the fretboard (default 3.0 = 3:1 landscape)
    pub svg_aspect_ratio: f64,
    /// Percentage of SVG height used as margin above/below frets (default 0.05 = 5%)
    pub fret_margin_percentage: f64,
    /// Width of the nut in SVG units (the zero fret at the head of the guitar)
    pub nut_width: f64,
    /// Number of extra frets to show beyond the active range for context
    pub extra_frets: usize,
    /// Fret positions where visual markers (dots) should be displayed
    pub marker_positions: Vec<u8>,
}

impl Default for FretboardConfig {
    fn default() -> Self {
        Self {
            num_strings: 6,
            max_frets: 22,
            svg_width_ratio: 0.9,
            svg_aspect_ratio: 3.0,
            fret_margin_percentage: 0.05,
            nut_width: 14.0,
            extra_frets: 1,
            marker_positions: vec![3, 5, 7, 9, 12, 15, 17, 19, 21, 24],
        }
    }
}

/// Interactive SVG fretboard component that displays a zoomable guitar fretboard
/// 
/// # Props
/// 
/// * `start_fret` - Signal indicating the first fret in the active/playable range
/// * `end_fret` - Signal indicating the last fret in the active/playable range  
/// * `config` - Optional configuration for visual appearance and behavior
/// 
/// # Features
/// 
/// * Responsive design that adapts to window size
/// * Zoom functionality that focuses on the active fret range
/// * Visual indicators for playable vs non-playable regions
/// * Standard guitar fretboard markers (dots at 3rd, 5th, 7th, etc.)
/// * Configurable for different instrument types (bass, 7-string, etc.)
/// 
/// # Example
/// 
/// ```rust
/// use leptos::prelude::*;
/// 
/// let start = RwSignal::new(3);
/// let end = RwSignal::new(7);
/// 
/// view! {
///   <SvgFretboard 
///     start_fret=start.into() 
///     end_fret=end.into()
///   />
/// }
/// ```
#[component]
pub fn SvgFretboard(
    /// First fret in the active/playable range
    start_fret: Signal<usize>, 
    /// Last fret in the active/playable range
    end_fret: Signal<usize>,
    /// Optional visual and behavioral configuration (uses sensible defaults)
    #[prop(optional)]
    config: Option<FretboardConfig>,
) -> impl IntoView {
  let config = config.unwrap_or_default();
  let num_frets = end_fret.get().max(config.max_frets);

  let UseWindowSizeReturn {
    width: window_width,
    height: _,
  } = use_window_size();

  let svg_width = Memo::new(move |_| window_width.get() * config.svg_width_ratio);
  let svg_height = Memo::new(move |_| svg_width.get() / config.svg_aspect_ratio);
  let fret_margin = Memo::new(move |_| svg_height.get() * config.fret_margin_percentage);

  // Calculate fret positions for the FULL fretboard
  let full_fret_positions =
    Memo::new(move |_| calculate_fret_positions(svg_width.get(), num_frets as u8));

  // Calculate visible range
  let min_fret = Memo::new(move |_| {
    if start_fret.get() > config.extra_frets {
      start_fret.get() - config.extra_frets
    } else {
      0
    }
  });

  let max_fret = Memo::new(move |_| (end_fret.get() + config.extra_frets).min(num_frets));

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
      current_svg_width - config.nut_width
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
    let offset = if has_nut { config.nut_width } else { 0.0 };
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
          format!("0 0 {} {}", current_svg_width, current_svg_height)
        }
        class="fretboard-svg"
        style="background: linear-gradient(90deg, #deb887 0%, #f5deb3 100%); border-radius: 8px; box-shadow: 0 2px 8px #0002; border: 1px solid #c00;"
      >
        {move || {
          let current_svg_height = svg_height.get();
          let current_fret_margin = fret_margin.get();
          let string_spacing = current_svg_height / (config.num_strings as f64 + 1.0);
          let positions = full_fret_positions.get();
          let min_f = min_fret.get();
          let max_f = max_fret.get();
          let start = start_fret.get();
          let end = end_fret.get();
          let current_svg_width = svg_width.get();
          let (_, viewbox_width) = (0.0, current_svg_width);
          let nut = if min_f == 0 {
            Some(

              // Nut (only when fret 0 is visible)
              view! {
                <rect
                  x="0"
                  y=current_fret_margin
                  width=config.nut_width
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
          let frets = (min_f..=max_f)
            .map(|fret_no| {
              let absolute_x = positions[fret_no];
              let x_pos = to_viewbox_x(absolute_x);
              let is_playable = fret_no >= start && fret_no <= end;
              let color = if is_playable { "#444" } else { "#bbb" };
              let width = if is_playable { "5" } else { "3" };

              // Frets

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
          let strings = (0..config.num_strings)
            .map(|i| {
              let y_pos = (i as f64 + 1.0) * string_spacing;
              let string_thickness = 1.0 + (i as f64);

              // Strings - span the full viewBox width

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
          let markers = (min_f..=max_f)
            .filter(|&fret| config.marker_positions.contains(&(fret as u8)))
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

              // Markers

              view! {
                <g>
                  <circle cx=x cy=cy1 r=r fill="#444" opacity="0.25" />
                  <circle cx=x cy=cy2 r=r fill="#444" opacity=op2 />
                </g>
              }
            })
            .collect_view();
          let overlay_left = if start > min_f {
            let start_x = to_viewbox_x(positions[start]);
            let width = start_x - (if min_f == 0 { config.nut_width } else { 0.0 });
            Some(

              // Overlays for non-playable regions
              view! {
                <rect
                  x=if min_f == 0 { config.nut_width } else { 0.0 }
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
          let overlay_right = if end < max_f {
            let end_x = to_viewbox_x(positions[end]);
            let width = viewbox_width - end_x;
            Some(

              view! {
                <rect
                  x=end_x
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
