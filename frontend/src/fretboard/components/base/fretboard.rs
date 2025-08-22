use crate::fretboard::{
  components::{
    base::{
      helper::{calculate_fret_positions, calculate_string_spacing, VisibleRange},
      layout::LayoutSnapshot,
      parts::{
        FretboardFrets, FretboardGrid, FretboardMarkers, FretboardNut, FretboardOverlays,
        FretboardStrings,
      },
    },
    visual_config::FretboardVisualConfig,
  },
  fretboard_model::{FretClickEvent, FretStateSignals, FretboardModel},
};
use leptos::prelude::*;
use shared::Note;

#[component]
pub fn FretboardViewModel(#[prop(into)] model: Signal<FretboardModel>) -> impl IntoView {
  let start_fret = Signal::derive(move || model.get().start_fret.get());
  let end_fret = Signal::derive(move || model.get().end_fret.get());
  let tuning = Signal::derive(move || model.get().tuning.get());
  let config = Signal::derive(move || model.get().config.get());
  let on_note_clicked = Signal::derive(move || model.get().on_note_clicked.get());
  let fret_states = Signal::derive(move || model.get().fret_states.get());
  view! { <Fretboard start_fret end_fret tuning config on_note_clicked fret_states /> }
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
  #[prop(into)]
  start_fret: Signal<usize>,
  /// Last fret in the active/playable range
  #[prop(into)]
  end_fret: Signal<usize>,
  /// Number of guitar strings (default: 6)
  #[prop(into)]
  tuning: Signal<Vec<Note>>,
  /// Visual configuration for fretboard display properties
  #[prop(into)]
  config: Signal<FretboardVisualConfig>,
  /// Optional callback for fret click events
  #[prop(into)]
  on_note_clicked: Signal<Option<Callback<FretClickEvent>>>,

  #[prop(into)] fret_states: Signal<FretStateSignals>,
) -> impl IntoView {
  // Create reactive signals from config values - using clone since Signal is Copy
  let svg_aspect_ratio = Signal::derive(move || config.get().svg_aspect_ratio);
  let fret_margin_percentage = Signal::derive(move || config.get().fret_margin_percentage);
  let nut_width = Signal::derive(move || config.get().nut_width);
  let extra_frets = Signal::derive(move || config.get().extra_frets);
  let marker_positions = Signal::derive(move || config.get().marker_positions.clone());
  let num_strings = Signal::derive(move || tuning.get().len() as u8);

  // Use a fixed base width for calculations, SVG will be scaled by CSS
  let base_svg_width = 800.0; // Fixed base width for consistent calculations
  let svg_width = Signal::derive(move || base_svg_width);
  let svg_height = Memo::new(move |_| svg_width.get() / svg_aspect_ratio.get());
  let fret_margin = Memo::new(move |_| svg_height.get() * fret_margin_percentage.get());

  let end_plus_extra_fret = Memo::new(move |_| end_fret.get() + extra_frets.get());

  // Calculate fret positions for the FULL fretboard
  let full_fret_positions =
    Memo::new(move |_| calculate_fret_positions(svg_width.get(), end_plus_extra_fret.get() as u8));

  // Calculate visible range - logic extracted to VisibleRange::new
  let visible_range = Memo::new(move |_| {
    VisibleRange::new(
      start_fret.get(),
      end_fret.get(),
      extra_frets.get(),
      end_plus_extra_fret.get(),
    )
  });

  let min_fret = Memo::new(move |_| visible_range.get().min_fret);
  let max_fret = Memo::new(move |_| visible_range.get().max_fret);

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
          let layout_snapshot = RwSignal::new(
            LayoutSnapshot::new(
              positions.clone(),
              min_f,
              max_f,
              start,
              end,
              current_num_strings,
              string_spacing,
              current_svg_width,
              current_svg_height,
              current_fret_margin,
              current_nut_width,
            ),
          );

          view! {
            // Conditionally render nut when fret 0 is visible
            {if layout_snapshot.get().has_nut {
              Some(
                view! {
                  <FretboardNut
                    nut_width=layout_snapshot.get().nut_width
                    fret_margin=current_fret_margin
                    svg_height=current_svg_height
                  />
                },
              )
            } else {
              None
            }}

            // Render all fret lines
            <FretboardFrets layout=layout_snapshot.get() />

            // Render string lines
            <FretboardStrings
              num_strings=current_num_strings
              string_spacing=string_spacing
              viewbox_width=viewbox_width
            />

            // Render fret markers
            <FretboardMarkers
              layout=layout_snapshot.get()
              marker_positions=current_marker_positions
            />

            // Render overlays for non-playable regions
            <FretboardOverlays layout=layout_snapshot.get() />

            // Grid: iterate frets/strings once and compose per-cell components
            <FretboardGrid
              layout=layout_snapshot.clone()
              tuning=tuning.clone()
              click_cb=on_note_clicked.clone().into()
              fret_states=fret_states
            />
          }
        }}
      </svg>
    </div>
  }
}
