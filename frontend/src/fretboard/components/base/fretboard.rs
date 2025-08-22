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

  let num_strings: Memo<u8> = Memo::new(move |_| tuning.get().len() as u8);

  // Use a fixed base width for calculations, SVG will be scaled by CSS
  let svg_width = 800.0; // Fixed base width for consistent calculations
  let svg_height = Memo::new(move |_| svg_width / config.get().svg_aspect_ratio.get());
  let fret_margin =
    Memo::new(move |_| svg_height.get() * config.get().fret_margin_percentage.get());

  let extra_frets = Memo::new(move |_| config.get().extra_frets.get());
  let end_plus_extra_fret = Memo::new(move |_| end_fret.get() + extra_frets.get());

  // Calculate fret positions for the FULL fretboard
  let full_fret_positions =
    Memo::new(move |_| calculate_fret_positions(svg_width, end_plus_extra_fret.get() as u8));

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

  let layout = Memo::new(move |_| {
    let svg_height = svg_height.get();
    let fret_margin = fret_margin.get();
    let num_strings = num_strings.get();
    let config = config.get();

    LayoutSnapshot::new(
      full_fret_positions.get(),
      min_fret.get(),
      max_fret.get(),
      start_fret.get(),
      end_fret.get(),
      num_strings,
      calculate_string_spacing(num_strings, svg_height),
      svg_width,
      svg_height,
      fret_margin,
      config.nut_width.get(),
    )
  });

  view! {
    <div class="flex justify-center items-center w-full">
      <svg
        viewBox=move || {
          let current_svg_height = svg_height.get();
          format!("0 0 {svg_width} {current_svg_height}")
        }
        class="w-full max-w-full h-auto fretboard-svg"
        style="background: linear-gradient(90deg, #deb887 0%, #f5deb3 100%); border-radius: 8px; box-shadow: 0 2px 8px #0002; border: 1px solid #c00;"
      >
        {move || {
          let cfg = config.get();
          let layout_snap = layout.get();
          let nut_width = cfg.nut_width.get();
          let marker_positions = cfg.marker_positions.get();
          let fret_margin_val = fret_margin.get();
          let svg_h = svg_height.get();
          view! {
            {if layout_snap.has_nut {
              Some(
                view! {
                  <FretboardNut nut_width=nut_width fret_margin=fret_margin_val svg_height=svg_h />
                },
              )
            } else {
              None
            }}
            <FretboardFrets layout=layout_snap.clone() />
            <FretboardStrings
              num_strings=num_strings.get()
              string_spacing=layout_snap.string_spacing
              viewbox_width=svg_width
            />
            <FretboardMarkers layout=layout_snap.clone() marker_positions=marker_positions />
            <FretboardOverlays layout=layout_snap.clone() />
            <FretboardGrid
              layout=layout_snap.clone()
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
