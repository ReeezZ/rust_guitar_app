use crate::components::fretboard::FretClickEvent;
use crate::components::fretboard_visual_config::FretboardVisualConfig;
use crate::components::musical_fretboard_config::{
  MusicalFretboardConfig, MusicalFretboardConfigSignals,
};
use crate::components::svg_fretboard_with_notes::SvgFretboardWithNotes;
use crate::fretboard_view_helper::calculate_fret_positions;
use crate::models::fretboard_model::FretCoord;
use crate::music::notes::Note;
use leptos::prelude::*;

/// SVG fretboard component for interval training exercises.
///
/// This component wraps `SvgFretboardWithNotes` and adds visual overlays for training:
/// - Green highlight for the reference note
/// - Red highlights for incorrect guesses
/// - Clear visual separation between model logic and presentation
///
/// # Example
///
/// ```rust
/// # use leptos::prelude::*;
/// # use rust_guitar_app::components::svg_fretboard_trainer::SvgFretboardTrainer;
/// # use rust_guitar_app::components::fretboard::FretClickEvent;
/// # use rust_guitar_app::models::fretboard_model::FretCoord;
/// # use rust_guitar_app::music::notes::Note;
///
/// # fn example() -> impl IntoView {
/// let reference_coord = RwSignal::new(Some(FretCoord { fret_idx: 2, string_idx: 1 }));
/// let reference_note = RwSignal::new(Some(Note::D));
/// let error_coords = RwSignal::new(vec![]);
/// let error_notes = RwSignal::new(vec![]);
///
/// let on_fret_clicked = Callback::new(move |event: FretClickEvent| {
///   leptos::logging::log!("Clicked note: {} at {:?}", event.note, event.coord);
/// });
///
/// view! {
///   <SvgFretboardTrainer
///     reference_note=reference_coord.into()
///     reference_note_name=reference_note.into()
///     error_notes=error_coords.into()
///     error_note_names=error_notes.into()
///     on_fret_clicked=on_fret_clicked
///   />
/// }
/// # }
/// ```
#[component]
pub fn SvgFretboardTrainer(
  /// The reference note coordinate to highlight in green (None = no highlight)
  reference_note: Signal<Option<FretCoord>>,
  /// The note at the reference coordinate (for display)
  reference_note_name: Signal<Option<Note>>,
  /// Coordinates of incorrect guesses to highlight in red
  error_notes: Signal<Vec<FretCoord>>,
  /// Notes at the error coordinates (for display)
  error_note_names: Signal<Vec<Note>>,

  // Configuration and interaction
  /// Fretboard visual configuration (optional, uses defaults if not provided)
  #[prop(optional)]
  config: Option<MusicalFretboardConfig>,
  /// Range of frets to display (optional, defaults to 0..=5)
  #[prop(optional, into)]
  fret_range: Option<Signal<std::ops::RangeInclusive<usize>>>,
  /// Callback for fret click events (enriched with note information)
  #[prop(optional)]
  on_fret_clicked: Option<Callback<FretClickEvent>>,
) -> impl IntoView {
  // Use provided config or create default
  let fretboard_config = config.unwrap_or_default();

  // Convert config to signals for the underlying components
  let config_signals = MusicalFretboardConfigSignals::from(fretboard_config);

  // Use provided fret_range or default to 0..=5
  let final_fret_range = fret_range.unwrap_or_else(|| Signal::derive(|| 0..=5));

  // Create visual config
  let visual_config = Signal::derive({
    let num_strings = config_signals.num_strings;
    let max_frets = config_signals.max_frets;
    let svg_aspect_ratio = config_signals.svg_aspect_ratio;
    let fret_margin_percentage = config_signals.fret_margin_percentage;
    let nut_width = config_signals.nut_width;
    let marker_positions = config_signals.marker_positions;
    let extra_frets = config_signals.extra_frets;

    move || FretboardVisualConfig {
      num_strings: num_strings.get(),
      max_frets: max_frets.get(),
      svg_aspect_ratio: svg_aspect_ratio.get(),
      fret_margin_percentage: fret_margin_percentage.get(),
      nut_width: nut_width.get(),
      marker_positions: marker_positions.get(),
      extra_frets: extra_frets.get(),
    }
  });

  // Calculate start and end frets from range
  let start_fret = Signal::derive({
    let range = final_fret_range;
    move || *range.get().start()
  });

  let end_fret = Signal::derive({
    let range = final_fret_range;
    move || *range.get().end()
  });

  // Calculate positions and transforms like the main component does
  let position_data = Signal::derive({
    let config = visual_config;
    let start = start_fret;
    let end = end_fret;

    move || {
      let cfg = config.get();
      let start_f = start.get();
      let end_f = end.get();

      // SVG dimensions (matching the main component)
      let svg_width = 1000.0;
      let svg_height = 300.0;

      // Calculate fret positions
      let scale_length = svg_width - cfg.nut_width;
      let positions = calculate_fret_positions(scale_length, cfg.max_frets as u8);

      // Calculate visible range with extra context
      let min_fret = if start_f > cfg.extra_frets {
        start_f - cfg.extra_frets
      } else {
        0
      };
      let max_fret = (end_f + cfg.extra_frets).min(cfg.max_frets);

      // Calculate zoom transform
      let has_nut = min_fret == 0;
      let range_start = if has_nut { 0.0 } else { positions[min_fret] };
      let range_end = positions[max_fret];
      let range_width = range_end - range_start;

      let available_width = if has_nut {
        svg_width - cfg.nut_width
      } else {
        svg_width
      };
      let scale_factor = available_width / range_width;

      // String spacing
      let string_spacing = svg_height / (cfg.num_strings as f64 + 1.0);

      (
        positions,
        min_fret,
        max_fret,
        has_nut,
        range_start,
        scale_factor,
        cfg.nut_width,
        string_spacing,
        svg_width,
        svg_height,
      )
    }
  });

  view! {
    <div class="relative">
      {move || {
        if let Some(callback) = on_fret_clicked {
          view! {
            <SvgFretboardWithNotes
              start_fret=start_fret
              end_fret=end_fret
              config=visual_config
              on_note_clicked=callback
            />
          }.into_any()
        } else {
          view! {
            <SvgFretboardWithNotes
              start_fret=start_fret
              end_fret=end_fret
              config=visual_config
            />
          }.into_any()
        }
      }}

      // Overlay SVG for reference and error highlights
      <svg
        class="absolute top-0 left-0 w-full h-full pointer-events-none"
        viewBox="0 0 1000 300"
        xmlns="http://www.w3.org/2000/svg"
      >
        // Reference note highlight (green)
        {move || {
          reference_note.get().map(|coord| {
            let (positions, _min_fret, _max_fret, has_nut, range_start, scale_factor, nut_width, string_spacing, _svg_width, _svg_height) = position_data.get();

            // Calculate position using the same logic as the main component
            let x = if coord.fret_idx == 0 {
              // Nut position
              nut_width / 2.0
            } else {
              // Fretted position - use midpoint between frets
              let fret_idx = coord.fret_idx as usize;
              let x_prev = if fret_idx == 0 { 0.0 } else { positions[(fret_idx - 1).max(0)] };
              let x_curr = positions[fret_idx];
              let x_center = (x_prev + x_curr) / 2.0;

              // Apply zoom transform
              let offset = if has_nut { nut_width } else { 0.0 };
              offset + (x_center - range_start) * scale_factor
            };

            let y = string_spacing * (coord.string_idx as f64 + 1.0);

            view! {
              <g>
                <circle
                  cx=x
                  cy=y
                  r="12"
                  fill="rgba(34, 197, 94, 0.7)"
                  stroke="rgb(34, 197, 94)"
                  stroke-width="2"
                />
                <text
                  x=x
                  y=y
                  text-anchor="middle"
                  dominant-baseline="central"
                  font-size="10"
                  font-weight="bold"
                  fill="white"
                >
                  {move || {
                    reference_note_name.get().map(|note| note.to_string()).unwrap_or_else(|| "?".to_string())
                  }}
                </text>
              </g>
            }
          })
        }}

        // Error note highlights (red)
        {move || {
          let coords = error_notes.get();
          let names = error_note_names.get();

          coords.into_iter().enumerate().map(|(idx, coord)| {
            let (positions, _min_fret, _max_fret, has_nut, range_start, scale_factor, nut_width, string_spacing, _svg_width, _svg_height) = position_data.get();

            // Calculate position using the same logic as the main component
            let x = if coord.fret_idx == 0 {
              // Nut position
              nut_width / 2.0
            } else {
              // Fretted position - use midpoint between frets
              let fret_idx = coord.fret_idx as usize;
              let x_prev = if fret_idx == 0 { 0.0 } else { positions[(fret_idx - 1).max(0)] };
              let x_curr = positions[fret_idx];
              let x_center = (x_prev + x_curr) / 2.0;

              // Apply zoom transform
              let offset = if has_nut { nut_width } else { 0.0 };
              offset + (x_center - range_start) * scale_factor
            };

            let y = string_spacing * (coord.string_idx as f64 + 1.0);
            let note_name = names.get(idx).map(|n| n.to_string()).unwrap_or_else(|| "?".to_string());

            view! {
              <g>
                <circle
                  cx=x
                  cy=y
                  r="12"
                  fill="rgba(239, 68, 68, 0.7)"
                  stroke="rgb(239, 68, 68)"
                  stroke-width="2"
                />
                <text
                  x=x
                  y=y
                  text-anchor="middle"
                  dominant-baseline="central"
                  font-size="10"
                  font-weight="bold"
                  fill="white"
                >
                  {note_name}
                </text>
              </g>
            }
          }).collect_view()
        }}
      </svg>
    </div>
  }
}
