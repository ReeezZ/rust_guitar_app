use crate::fretboard_view_helper::calculate_fret_positions;
use leptos::prelude::*;

/// Calculate string spacing for the given number of strings and SVG height
fn calculate_string_spacing(num_strings: u8, svg_height: f64) -> f64 {
  svg_height / (num_strings as f64 + 1.0)
}

/// Visible fret range including context frets
#[derive(Debug, Clone, PartialEq)]
struct VisibleRange {
  /// Minimum fret to display (includes context)
  min_fret: usize,
  /// Maximum fret to display (includes context)
  max_fret: usize,
}

impl VisibleRange {
  /// Calculate the visible fret range including extra context frets
  fn new(start_fret: usize, end_fret: usize, extra_frets: usize, max_frets: usize) -> Self {
    let min_fret = if start_fret > extra_frets {
      start_fret - extra_frets
    } else {
      0
    };
    let max_fret = (end_fret + extra_frets).min(max_frets);

    Self { min_fret, max_fret }
  }
}

/// Parameters for coordinate transformation and zoom scaling
#[derive(Debug, Clone, PartialEq)]
struct ZoomTransform {
  /// Starting position of the visible range in absolute coordinates
  range_start: f64,
  /// Scale factor to apply to coordinates
  scale_factor: f64,
  /// Whether the nut is visible (fret 0 is in range)
  has_nut: bool,
}

impl ZoomTransform {
  /// Create a new zoom transform for the given fret range and dimensions
  fn new(
    positions: &[f64],
    min_fret: usize,
    max_fret: usize,
    svg_width: f64,
    nut_width: f64,
  ) -> Self {
    let has_nut = min_fret == 0;

    // Physical range we want to display
    let range_start = if has_nut { 0.0 } else { positions[min_fret] };
    let range_end = positions[max_fret];
    let range_width = range_end - range_start;

    // Available width for fret content (accounting for nut if visible)
    let available_width = if has_nut {
      svg_width - nut_width
    } else {
      svg_width
    };

    // Scale factor to make the selected range fill the available width
    let scale_factor = available_width / range_width;

    Self {
      range_start,
      scale_factor,
      has_nut,
    }
  }

  /// Transform absolute fretboard coordinates to scaled viewbox coordinates
  fn to_viewbox_x(&self, absolute_x: f64, nut_width: f64) -> f64 {
    let offset = if self.has_nut { nut_width } else { 0.0 };
    offset + (absolute_x - self.range_start) * self.scale_factor
  }

  /// Get nut width to use in calculations (0 if nut not visible)
  fn effective_nut_width(&self, nut_width: f64) -> f64 {
    if self.has_nut {
      nut_width
    } else {
      0.0
    }
  }
}

/// Renders the nut (zero fret) when visible
#[component]
fn FretboardNut(
  /// Width of the nut in SVG units
  nut_width: f64,
  /// Top margin for the nut
  fret_margin: f64,
  /// Total SVG height
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
fn FretboardFrets(
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
fn FretboardStrings(
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
fn FretboardMarkers(
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
fn FretboardOverlays(
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
    let start_x = to_viewbox_x(positions[start_fret]);
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

/// Interactive SVG fretboard component that displays a zoomable guitar fretboard
///
/// # Props
///
/// * `start_fret` - Signal indicating the first fret in the active/playable range
/// * `end_fret` - Signal indicating the last fret in the active/playable range  
/// * Configuration props - Optional individual configuration values
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
/// Basic usage with default configuration:
///
/// ```rust
/// # use leptos::prelude::*;
/// # use rust_guitar_app::components::svg_fretboard::SvgFretboard;
///
/// // This would be inside a component
/// # fn example_usage() -> impl IntoView {
/// let start = RwSignal::new(3);
/// let end = RwSignal::new(7);
///
/// // The component usage (this is what users copy)
/// view! {
///     <SvgFretboard
///         start_fret=start.into()
///         end_fret=end.into()
///     />
/// }
/// # }
/// ```
///
/// With custom configuration:
///
/// ```rust
/// # use leptos::prelude::*;
/// # use rust_guitar_app::components::svg_fretboard::SvgFretboard;
///
/// # fn custom_config_example() -> impl IntoView {
/// let start = RwSignal::new(0);
/// let end = RwSignal::new(12);
/// let strings = RwSignal::new(4_u8); // Bass guitar
///
/// view! {
///     <SvgFretboard
///         start_fret=start.into()
///         end_fret=end.into()
///         num_strings=strings.read_only()
///         svg_aspect_ratio=Signal::derive(move || 4.0)
///     />
/// }
/// # }
/// ```
#[component]
pub fn SvgFretboard(
  /// First fret in the active/playable range
  start_fret: Signal<usize>,
  /// Last fret in the active/playable range
  end_fret: Signal<usize>,
  /// Number of guitar strings (default: 6)
  #[prop(optional, into)]
  num_strings: Option<Signal<u8>>,
  /// Maximum number of frets to display (default: 22)
  #[prop(optional, into)]
  max_frets: Option<Signal<usize>>,
  /// Width-to-height aspect ratio (default: 3.0)
  #[prop(optional, into)]
  svg_aspect_ratio: Option<Signal<f64>>,
  /// Percentage of SVG height used as margin (default: 0.05)
  #[prop(optional, into)]
  fret_margin_percentage: Option<Signal<f64>>,
  /// Width of the nut in SVG units (default: 14.0)
  #[prop(optional, into)]
  nut_width: Option<Signal<f64>>,
  /// Number of extra frets to show for context (default: 1)
  #[prop(optional, into)]
  extra_frets: Option<Signal<usize>>,
  /// Fret positions where markers should be displayed
  #[prop(optional, into)]
  marker_positions: Option<Signal<Vec<u8>>>,
) -> impl IntoView {
  // Use signals if provided, otherwise use default values
  let num_strings = num_strings.unwrap_or_else(|| Signal::derive(move || 6_u8));
  let max_frets = max_frets.unwrap_or_else(|| Signal::derive(move || 22_usize));
  let svg_aspect_ratio = svg_aspect_ratio.unwrap_or_else(|| Signal::derive(move || 3.0_f64));
  let fret_margin_percentage =
    fret_margin_percentage.unwrap_or_else(|| Signal::derive(move || 0.05_f64));
  let nut_width = nut_width.unwrap_or_else(|| Signal::derive(move || 14.0_f64));
  let extra_frets = extra_frets.unwrap_or_else(|| Signal::derive(move || 1_usize));
  let marker_positions = marker_positions
    .unwrap_or_else(|| Signal::derive(move || vec![3_u8, 5, 7, 9, 12, 15, 17, 19, 21, 24]));

  let num_frets = Memo::new(move |_| end_fret.get().max(max_frets.get()));

  // Use a fixed base width for calculations, SVG will be scaled by CSS
  let base_svg_width = 800.0; // Fixed base width for consistent calculations
  let svg_width = Signal::derive(move || base_svg_width);
  let svg_height = Memo::new(move |_| svg_width.get() / svg_aspect_ratio.get());
  let fret_margin = Memo::new(move |_| svg_height.get() * fret_margin_percentage.get());

  // Calculate fret positions for the FULL fretboard
  let full_fret_positions =
    Memo::new(move |_| calculate_fret_positions(svg_width.get(), num_frets.get() as u8));

  // Calculate visible range - logic extracted to VisibleRange::new
  let visible_range = Memo::new(move |_| {
    VisibleRange::new(
      start_fret.get(),
      end_fret.get(),
      extra_frets.get(),
      num_frets.get(),
    )
  });

  let min_fret = Memo::new(move |_| visible_range.get().min_fret);
  let max_fret = Memo::new(move |_| visible_range.get().max_fret);

  // Clean zoom transformation - calculation logic extracted to ZoomTransform::new
  let zoom_transform = Memo::new(move |_| {
    ZoomTransform::new(
      &full_fret_positions.get(),
      min_fret.get(),
      max_fret.get(),
      svg_width.get(),
      nut_width.get(),
    )
  });

  // Clean coordinate transformation function
  let to_viewbox_x = move |absolute_x: f64| -> f64 {
    let transform = zoom_transform.get();
    transform.to_viewbox_x(absolute_x, nut_width.get())
  };

  view! {
    <div class="flex justify-center items-center w-full">
      <svg
        viewBox=move || {
          let current_svg_width = svg_width.get();
          let current_svg_height = svg_height.get();
          format!("0 0 {} {}", current_svg_width, current_svg_height)
        }
        class="w-full max-w-full h-auto fretboard-svg"
        style="background: linear-gradient(90deg, #deb887 0%, #f5deb3 100%); border-radius: 8px; box-shadow: 0 2px 8px #0002; border: 1px solid #c00;"
      >
        {move || {
          let current_svg_height = svg_height.get();
          let current_fret_margin = fret_margin.get();
          let string_spacing = calculate_string_spacing(num_strings.get(), current_svg_height);
          let positions = full_fret_positions.get();
          let min_f = min_fret.get();
          let max_f = max_fret.get();
          let start = start_fret.get();
          let end = end_fret.get();
          let current_svg_width = svg_width.get();
          let viewbox_width = current_svg_width;
          let current_nut_width = nut_width.get();

          view! {
            // Conditionally render nut when fret 0 is visible
            {if zoom_transform.get().has_nut {
              Some(
                view! {
                  <FretboardNut
                    nut_width=current_nut_width
                    fret_margin=current_fret_margin
                    svg_height=current_svg_height
                  />
                },
              )
            } else {
              None
            }}

            // Render all fret lines
            <FretboardFrets
              min_fret=min_f
              max_fret=max_f
              start_fret=start
              end_fret=end
              positions=positions.clone()
              to_viewbox_x=to_viewbox_x
              fret_margin=current_fret_margin
              svg_height=current_svg_height
            />

            // Render string lines
            <FretboardStrings
              num_strings=num_strings.get()
              string_spacing=string_spacing
              viewbox_width=viewbox_width
            />

            // Render fret markers
            <FretboardMarkers
              min_fret=min_f
              max_fret=max_f
              marker_positions=marker_positions.get()
              positions=positions.clone()
              to_viewbox_x=to_viewbox_x
              svg_height=current_svg_height
            />

            // Render overlays for non-playable regions
            <FretboardOverlays
              min_fret=min_f
              max_fret=max_f
              start_fret=start
              end_fret=end
              positions=positions
              to_viewbox_x=to_viewbox_x
              nut_width=zoom_transform.get().effective_nut_width(current_nut_width)
              fret_margin=current_fret_margin
              svg_height=current_svg_height
              svg_width=current_svg_width
            />
          }
        }}
      </svg>
    </div>
  }
}
