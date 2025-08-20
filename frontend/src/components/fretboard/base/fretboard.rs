use std::collections::HashMap;

use crate::components::fretboard::base::helper::{
  calculate_string_spacing, VisibleRange, ZoomTransform,
};
use crate::components::fretboard::base::parts::{
  FretboardClickableAreas, FretboardFrets, FretboardMarkers, FretboardNotes, FretboardNut,
  FretboardOverlays, FretboardStrings,
};
use crate::components::fretboard::visual_config::FretboardVisualConfig;
use crate::fretboard_view_helper::calculate_fret_positions;
use crate::models::fretboard_model::FretCoord;
use crate::models::FretState;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct FretClickEvent {
  pub coord: FretCoord,
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
/// # use frontend::components::fretboard::base::Fretboard;
///
/// // This would be inside a component
/// # fn example_usage() -> impl IntoView {
/// let start = RwSignal::new(3);
/// let end = RwSignal::new(7);
///
/// // The component usage (this is what users copy)
/// view! {
///     <Fretboard
///         start_fret=start.into()
///         end_fret=end.into()
///     />
/// }
/// # }
/// ```
///
/// With custom configuration, pass a FretboardVisualConfig through the config prop.
#[component]
pub fn Fretboard(
  /// First fret in the active/playable range
  start_fret: Signal<usize>,
  /// Last fret in the active/playable range
  end_fret: Signal<usize>,
  /// Visual configuration for fretboard display properties
  #[prop(optional, into)]
  config: Option<Signal<FretboardVisualConfig>>,
  /// Optional callback for fret click events
  #[prop(optional)]
  on_fret_clicked: Option<Callback<FretClickEvent>>,

  #[prop(optional)] fret_states: Signal<HashMap<FretCoord, Signal<FretState>>>,
) -> impl IntoView {
  // Use provided config signal or create one with default
  let config_signal = config.unwrap_or_else(|| Signal::derive(FretboardVisualConfig::default));

  // Create reactive signals from config values - using clone since Signal is Copy
  let num_strings = Signal::derive(move || config_signal.get().num_strings);
  let max_frets = Signal::derive(move || config_signal.get().max_frets);
  let svg_aspect_ratio = Signal::derive(move || config_signal.get().svg_aspect_ratio);
  let fret_margin_percentage = Signal::derive(move || config_signal.get().fret_margin_percentage);
  let nut_width = Signal::derive(move || config_signal.get().nut_width);
  let extra_frets = Signal::derive(move || config_signal.get().extra_frets);
  let marker_positions = Signal::derive(move || config_signal.get().marker_positions.clone());

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
    let transform = zoom_transform.get_untracked();
    transform.to_viewbox_x(absolute_x, nut_width.get_untracked())
  };

  view! {
    <div class="flex justify-center items-center w-full">
      <svg
        viewBox=move || {
          let current_svg_width = svg_width.get();
          let current_svg_height = svg_height.get();
          format!("0 0 {current_svg_width} {current_svg_height}")
        }
        class="w-full max-w-full h-auto fretboard-svg"
        style="background: linear-gradient(90deg, #deb887 0%, #f5deb3 100%); border-radius: 8px; box-shadow: 0 2px 8px #0002; border: 1px solid #c00;"
      >
        {move || {
          let current_svg_height = svg_height.get();
          let current_fret_margin = fret_margin.get();
          let current_num_strings = num_strings.get();
          let string_spacing = calculate_string_spacing(current_num_strings, current_svg_height);
          let positions = full_fret_positions.get();
          let min_f = min_fret.get();
          let max_f = max_fret.get();
          let start = start_fret.get();
          let end = end_fret.get();
          let current_svg_width = svg_width.get();
          let viewbox_width = current_svg_width;
          let current_nut_width = nut_width.get();
          let current_marker_positions = marker_positions.get();
          let current_zoom_transform = zoom_transform.get();

          view! {
            // Conditionally render nut when fret 0 is visible
            {if current_zoom_transform.has_nut {
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
              num_strings=current_num_strings
              string_spacing=string_spacing
              viewbox_width=viewbox_width
            />

            // Render fret markers
            <FretboardMarkers
              min_fret=min_f
              max_fret=max_f
              marker_positions=current_marker_positions
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
              positions=positions.clone()
              to_viewbox_x=to_viewbox_x
              nut_width=current_zoom_transform.effective_nut_width(current_nut_width)
              fret_margin=current_fret_margin
              svg_height=current_svg_height
              svg_width=current_svg_width
            />

            // Render clickable areas if callback is provided
            {on_fret_clicked
              .map(move |callback| {
                view! {
                  <FretboardClickableAreas
                    min_fret=min_f
                    max_fret=max_f
                    num_strings=current_num_strings
                    string_spacing=string_spacing
                    positions=positions.clone()
                    to_viewbox_x=to_viewbox_x
                    has_nut=current_zoom_transform.has_nut
                    nut_width=current_nut_width
                    on_fret_clicked=callback
                  />
                }
              })}

            <FretboardNotes
              min_fret=min_f
              max_fret=max_f
              positions=positions
              to_viewbox_x=to_viewbox_x
              svg_height=current_svg_height
              frets=fret_states
            />
          }
        }}
      </svg>
    </div>
  }
}
