use crate::components::fretboard::{
  base::{
    helper::{calculate_fret_positions, calculate_string_spacing},
    layout::LayoutSnapshot,
    parts::{
      FretboardFrets, FretboardGrid, FretboardMarkers, FretboardNut, FretboardOverlays,
      FretboardStrings,
    },
  },
  visual_config::FretboardVisualConfig,
};

use super::{FretClickEvent, FretStateSignals};
use leptos::prelude::*;
use shared::Note;

/// The FretStateSignals have to be carefully managed
/// Use FretboardViewModel by passing a FretboardModel to ensure proper updates
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

  /// The fret states must be prallocated. Use [`crate::models::fretboard_model::get_preallocated_fret_states()`] to create initial fret_states
  #[prop(into)]
  fret_states: Signal<FretStateSignals>,
) -> impl IntoView {
  // Create reactive signals from config values - using clone since Signal is Copy

  let num_strings: Memo<u8> = Memo::new(move |_| tuning.get().len() as u8);

  // Use a fixed base width for calculations, SVG will be scaled by CSS
  let svg_width = 800.0; // Fixed base width for consistent calculations
  let svg_height = Memo::new(move |_| svg_width / config.get().svg_aspect_ratio.get());
  let fret_margin =
    Memo::new(move |_| svg_height.get() * config.get().fret_margin_percentage.get());
  let marker_positions = Signal::derive(move || config.get().marker_positions.get());

  let extra_frets = Signal::derive(move || config.get().extra_frets.get());

  let min_visible_fret = Memo::new(move |_| start_fret.get().saturating_sub(extra_frets.get() + 1));
  let max_visible_fret = Memo::new(move |_| end_fret.get() + extra_frets.get());

  let full_fret_positions =
    Memo::new(move |_| calculate_fret_positions(svg_width, max_visible_fret.get() as u8 + 2));

  let string_spacing =
    Memo::new(move |_| calculate_string_spacing(num_strings.get(), svg_height.get()));

  let nut_width = Memo::new(move |_| config.get().nut_width.get());

  let has_nut = Memo::new(move |_| min_visible_fret.get() == 0);

  let layout = LayoutSnapshot::new(
    full_fret_positions.into(),
    // start_fret.into(),
    // end_fret.into(),
    min_visible_fret.into(),
    max_visible_fret.into(),
    num_strings.into(),
    string_spacing.into(),
    svg_width.into(),
    svg_height.into(),
    fret_margin.into(),
    nut_width.into(),
    has_nut.into(),
  );

  let viewbox_positions = Signal::derive(move || {
    layout
      .absolute_positions
      .get()
      .iter()
      .map(|&x| layout.abs_to_viewbox_x(x))
      .collect::<Vec<f64>>()
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
          if has_nut.get() {
            Some(
              view! {
                <FretboardNut
                  nut_width=nut_width.get()
                  fret_margin=fret_margin.get()
                  svg_height=svg_height.get()
                />
              },
            )
          } else {
            None
          }
        }}

        <FretboardFrets
          start_fret
          end_fret
          min_visible_fret
          max_visible_fret
          viewbox_positions
          fret_margin
          svg_height
        />

        <FretboardStrings
          num_strings=num_strings
          string_spacing=string_spacing
          viewbox_width=svg_width
        />
        {move || {
          view! {
            <FretboardMarkers
              svg_height
              viewbox_positions
              marker_positions
              min_visible_fret=min_visible_fret.get()
              max_visible_fret=max_visible_fret.get()
            />
          }
        }}

        <FretboardOverlays
          layout
          start_fret=start_fret
          end_fret=end_fret
          min_visible_fret
          max_visible_fret
        />
        <FretboardGrid
          layout
          min_visible_fret
          max_visible_fret
          tuning
          click_cb=on_note_clicked
          fret_states=fret_states
        />
      </svg>
    </div>
  }
}
