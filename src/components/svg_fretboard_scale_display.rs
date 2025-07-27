use crate::components::fretboard::FretClickEvent;
use crate::components::musical_fretboard_config::{MusicalFretboardConfig, MusicalFretboardConfigSignals};
use crate::components::svg_fretboard_with_notes::SvgFretboardWithNotes;
use crate::fretboard_view_helper::calculate_fret_positions;
use crate::music::notes::Note;
use crate::music::scales::{Scale, ScaleTrait, ScaleType};
use leptos::prelude::*;

/// SVG fretboard component that adds scale visualization to the note-aware fretboard.
///
/// This component wraps `SvgFretboardWithNotes` and adds visual highlighting of scale notes
/// with different colors for root notes vs other scale notes.
///
/// The component now uses a simplified API with a `MusicalFretboardConfig` struct instead of
/// many individual props, making it easier to use and maintain.
///
/// # Example
///
/// ```rust
/// # use leptos::prelude::*;
/// # use rust_guitar_app::components::svg_fretboard_scale_display::SvgFretboardScaleDisplay;
/// # use rust_guitar_app::components::musical_fretboard_config::MusicalFretboardConfig;
/// # use rust_guitar_app::components::fretboard::FretClickEvent;
/// # use rust_guitar_app::music::notes::Note;
/// # use rust_guitar_app::music::scales::ScaleType;
/// # use rust_guitar_app::music::heptatonic_scales::HeptaScaleType;
///
/// # fn example() -> impl IntoView {
/// let start = RwSignal::new(3);
/// let end = RwSignal::new(7);
/// let root_note = RwSignal::new(Note::C);
/// let scale_type = RwSignal::new(ScaleType::Hepatonic(HeptaScaleType::Major));
///
/// // Optional: customize fretboard appearance
/// let config = MusicalFretboardConfig::default()
///   .with_aspect_ratio(2.5)
///   .with_num_strings(7);
///
/// let on_note_clicked = Callback::new(move |event: FretClickEvent| {
///   leptos::logging::log!("Clicked note: {} at fret {}, string {}",
///     event.note, event.coord.fret_idx, event.coord.string_idx);
/// });
///
/// view! {
///   <SvgFretboardScaleDisplay
///     start_fret=start.into()
///     end_fret=end.into()
///     root_note=root_note.into()
///     scale_type=scale_type.into()
///     config=config
///     on_note_clicked=on_note_clicked
///   />
/// }
/// # }
/// ```
#[component]
pub fn SvgFretboardScaleDisplay(
  /// First fret in the active/playable range
  start_fret: Signal<usize>,
  /// Last fret in the active/playable range
  end_fret: Signal<usize>,

  // Scale configuration
  /// Root note of the scale
  root_note: Signal<Note>,
  /// Type of scale to display
  scale_type: Signal<ScaleType>,

  // Configuration and interaction
  /// Fretboard visual configuration (optional, uses defaults if not provided)
  #[prop(optional)]
  config: Option<MusicalFretboardConfig>,
  /// Callback for note click events (enriched with note information)
  #[prop(optional)]
  on_note_clicked: Option<Callback<FretClickEvent>>,
) -> impl IntoView {
  // Use provided config or create default
  let fretboard_config = config.unwrap_or_default();
  
  // Extract visual config before moving the full config
  let visual_config = fretboard_config.visual.clone();
  
  // Convert config to signals for the underlying components
  let config_signals = MusicalFretboardConfigSignals::from(fretboard_config);

  // Create the scale from root note and scale type
  let scale = Memo::new(move |_| Scale::new(root_note.get(), scale_type.get()));

  view! {
    <div class="relative">
      // Base fretboard with note awareness
      <SvgFretboardWithNotes
        start_fret=start_fret
        end_fret=end_fret
        tuning=config_signals.tuning
        on_note_clicked=on_note_clicked.unwrap_or_else(|| Callback::new(|_| {}))
        config=visual_config
      />

      // Scale note overlays
      <ScaleNoteOverlays
        scale=scale
        tuning=config_signals.tuning
        start_fret=start_fret
        end_fret=end_fret
        extra_frets=config_signals.extra_frets
        num_strings=config_signals.num_strings
        svg_aspect_ratio=config_signals.svg_aspect_ratio
        _fret_margin_percentage=config_signals.fret_margin_percentage
        nut_width=config_signals.nut_width
      />
    </div>
  }
}

/// Component that renders scale note overlays on top of the fretboard
#[component]
fn ScaleNoteOverlays(
  /// The scale to display
  scale: Memo<Scale>,
  /// Guitar tuning
  tuning: Signal<Vec<Note>>,
  /// Fretboard range parameters
  start_fret: Signal<usize>,
  end_fret: Signal<usize>,
  extra_frets: Signal<usize>,
  num_strings: Signal<u8>,
  svg_aspect_ratio: Signal<f64>,
  _fret_margin_percentage: Signal<f64>,
  nut_width: Signal<f64>,
) -> impl IntoView {
  // The key insight: we need to use the SAME coordinate system as the underlying SVG fretboard
  // This means using the same base width, height calculations, and zoom transforms

  view! {
    <div class="absolute inset-0 pointer-events-none">
      <svg
        viewBox=move || {
          let base_width = 800.0;
          let height = base_width / svg_aspect_ratio.get();
          format!("0 0 {} {}", base_width, height)
        }
        class="w-full h-full"
      >
        {move || {
          let current_scale = scale.get();
          let current_tuning = tuning.get();
          let current_start_fret = start_fret.get();
          let current_end_fret = end_fret.get();
          let current_extra_frets = extra_frets.get();
          let current_num_strings = num_strings.get();
          let base_width = 800.0;
          let height = base_width / svg_aspect_ratio.get();
          let current_nut_width = nut_width.get();

          // Use the SAME calculation logic as the main SVG fretboard
          let string_spacing = height / (current_num_strings as f64 + 1.0);

          // Calculate visible range using the same logic as SVG fretboard
          let min_visible = if current_start_fret > current_extra_frets {
            current_start_fret - current_extra_frets
          } else {
            0
          };
          let max_visible = (current_end_fret + current_extra_frets).min(22);

          // Calculate full fret positions (same as SVG fretboard)
          let full_fret_positions = calculate_fret_positions(base_width, 22);

          // Calculate zoom transform (same logic as SVG fretboard ZoomTransform::new)
          let has_nut = min_visible == 0;
          let range_start = if has_nut { 0.0 } else { full_fret_positions[min_visible] };
          let range_end = full_fret_positions[max_visible];
          let range_width = range_end - range_start;
          let available_width = if has_nut { base_width - current_nut_width } else { base_width };
          let scale_factor = available_width / range_width;

          // Transform function (matches SVG fretboard's to_viewbox_x)
          let to_viewbox_x = |absolute_x: f64| -> f64 {
            let offset = if has_nut { current_nut_width } else { 0.0 };
            offset + (absolute_x - range_start) * scale_factor
          };

          // Generate overlays for each visible string and fret
          let mut overlays = Vec::new();

          for string_idx in 0..current_num_strings {
            // Convert SVG string index to tuning array index (reverse order)
            let tuning_index = (current_tuning.len() - 1) - (string_idx as usize);
            if let Some(base_note) = current_tuning.get(tuning_index) {
              for fret_idx in min_visible..=max_visible {
                let note = base_note.add_steps(fret_idx);

                if current_scale.contains_note(note) {
                  let is_root = current_scale.root_note().map_or(false, |root| root == note);

                  // Calculate Y position (same as SVG fretboard string calculation)
                  let y = (string_idx as f64 + 1.0) * string_spacing;

                  // Calculate X position using the same transform as SVG fretboard
                  let x = if fret_idx == 0 {
                    // Nut position: center of nut width
                    current_nut_width / 2.0
                  } else {
                    // Fret position: place in the MIDDLE of the fret space (where you finger the note)
                    // This matches the clickable area positioning: (x_prev + x_curr) / 2.0
                    let x_prev = if fret_idx == 0 { 0.0 } else { full_fret_positions[(fret_idx - 1).max(0)] };
                    let x_curr = full_fret_positions[fret_idx];
                    let finger_position = (x_prev + x_curr) / 2.0;
                    to_viewbox_x(finger_position)
                  };

                  let (fill_color, stroke_color, radius) = if is_root {
                    ("#ff4444", "#cc0000", 12.0) // Red for root notes
                  } else {
                    ("#4444ff", "#0000cc", 8.0) // Blue for other scale notes
                  };

                  overlays.push(view! {
                    <g>
                      <circle
                        cx=x
                        cy=y
                        r=radius
                        fill=fill_color
                        stroke=stroke_color
                        stroke-width="2"
                        opacity="0.8"
                      />
                      <text
                        x=x
                        y=y + 1.0
                        text-anchor="middle"
                        dominant-baseline="central"
                        fill="white"
                        font-size="8"
                        font-weight="bold"
                      >
                        {note.to_string()}
                      </text>
                    </g>
                  });
                }
              }
            }
          }

          overlays.into_iter().collect_view()
        }}
      </svg>
    </div>
  }
}
