use crate::components::fretboard::FretClickEvent;
use crate::components::fretboard_visual_config::FretboardVisualConfig;
use crate::components::musical_fretboard_config::{MusicalFretboardConfig, MusicalFretboardConfigSignals};
use crate::components::svg_fretboard_with_notes::SvgFretboardWithNotes;
use crate::fretboard_view_helper::calculate_fret_positions;
use crate::models::fretboard_model::FretCoord;
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
///
/// # fn example() -> impl IntoView {
/// let reference_coord = RwSignal::new(Some(FretCoord { fret_idx: 2, string_idx: 1 }));
/// let error_coords = RwSignal::new(vec![]);
///
/// let on_fret_clicked = Callback::new(move |event: FretClickEvent| {
///   leptos::logging::log!("Clicked note: {} at {:?}", event.note, event.coord);
/// });
///
/// view! {
///   <SvgFretboardTrainer
///     reference_note=reference_coord.into()
///     error_notes=error_coords.into()
///     on_fret_clicked=on_fret_clicked
///   />
/// }
/// # }
/// ```
#[component]
pub fn SvgFretboardTrainer(
  /// The reference note coordinate to highlight in green (None = no highlight)
  reference_note: Signal<Option<FretCoord>>,
  /// Coordinates of incorrect guesses to highlight in red
  error_notes: Signal<Vec<FretCoord>>,
  
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

  // Create fret positions for overlay calculations
  let fret_positions = Signal::derive({
    let config = visual_config;
    move || {
      let cfg = config.get();
      let svg_width = 1000.0; // Default SVG width
      calculate_fret_positions(svg_width - cfg.nut_width, cfg.max_frets as u8)
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
            let positions = fret_positions.get();
            let config = visual_config.get();
            let string_spacing = 300.0 / (config.num_strings as f64 + 1.0);
            
            // Calculate position
            let x = if coord.fret_idx == 0 {
              config.nut_width / 2.0
            } else {
              config.nut_width + positions[coord.fret_idx as usize] - (positions[1] - positions[0]) / 2.0
            };
            let y = string_spacing * (coord.string_idx as f64 + 1.0);
            
            view! {
              <circle
                cx=x
                cy=y
                r="12"
                fill="rgba(34, 197, 94, 0.7)"
                stroke="rgb(34, 197, 94)"
                stroke-width="2"
              />
            }
          })
        }}
        
        // Error note highlights (red)
        {move || {
          error_notes.get().into_iter().map(|coord| {
            let positions = fret_positions.get();
            let config = visual_config.get();
            let string_spacing = 300.0 / (config.num_strings as f64 + 1.0);
            
            // Calculate position
            let x = if coord.fret_idx == 0 {
              config.nut_width / 2.0
            } else {
              config.nut_width + positions[coord.fret_idx as usize] - (positions[1] - positions[0]) / 2.0
            };
            let y = string_spacing * (coord.string_idx as f64 + 1.0);
            
            view! {
              <circle
                cx=x
                cy=y
                r="12"
                fill="rgba(239, 68, 68, 0.7)"
                stroke="rgb(239, 68, 68)"
                stroke-width="2"
              />
            }
          }).collect_view()
        }}
      </svg>
    </div>
  }
}
