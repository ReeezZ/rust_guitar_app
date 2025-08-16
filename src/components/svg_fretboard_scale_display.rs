use crate::components::fretboard::FretClickEvent;
use crate::components::fretboard_visual_config::FretboardVisualConfig;
use crate::components::musical_fretboard_config::{
  MusicalFretboardConfig, MusicalFretboardConfigSignals,
};
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
/// The component uses a clean range-based API where you specify exactly which frets should
/// show scale notes. This makes it perfect for focused practice on specific fret positions.
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
/// let fret_range = RwSignal::new(3..=7);  // Focus on frets 3-7
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
///     fret_range=fret_range.into()
///     root_note=root_note.into()
///     scale_type=scale_type.into()
///     config=config
///     on_note_clicked=on_note_clicked
///   />
/// }
/// # }
/// ```
///
/// # Range Examples
///
/// - `0..=5` - Open position including open strings (perfect for Em, Am, etc.)
/// - `1..=5` - First position excluding open strings (focus on fingered notes)
/// - `3..=7` - Higher position practice (barre chord area)
/// - `0..=0` - Just open strings (useful for chord tone analysis)
#[component]
pub fn SvgFretboardScaleDisplay(
  /// Range of frets to include in scale display (e.g., 0..=5 includes open strings, 1..=5 excludes them)
  fret_range: Signal<std::ops::RangeInclusive<usize>>,

  // Scale configuration
  /// Root note of the scale
  root_note: Signal<Note>,
  /// Type of scale to display
  scale_type: Signal<ScaleType>,

  // Configuration and interaction
  /// Fretboard visual configuration (optional, uses defaults if not provided)
  #[prop(optional)]
  config: Option<MusicalFretboardConfig>,
  /// Number of extra frets to show for context (optional, defaults to config value)
  #[prop(optional, into)]
  extra_frets: Option<Signal<usize>>,
  /// Callback for note click events (enriched with note information)
  #[prop(optional)]
  on_note_clicked: Option<Callback<FretClickEvent>>,
) -> impl IntoView {
  // Use provided config or create default
  let fretboard_config = config.unwrap_or_default();

  // Convert config to signals for the underlying components
  let config_signals = MusicalFretboardConfigSignals::from(fretboard_config);

  // Use provided extra_frets or fall back to config value
  let final_extra_frets = extra_frets.unwrap_or_else(|| config_signals.extra_frets);

  // Create visual config that updates with extra_frets
  let visual_config = Signal::derive({
    let num_strings = config_signals.num_strings;
    let max_frets = config_signals.max_frets;
    let svg_aspect_ratio = config_signals.svg_aspect_ratio;
    let fret_margin_percentage = config_signals.fret_margin_percentage;
    let nut_width = config_signals.nut_width;
    let marker_positions = config_signals.marker_positions;
    move || FretboardVisualConfig {
      num_strings: num_strings.get(),
      max_frets: max_frets.get(),
      svg_aspect_ratio: svg_aspect_ratio.get(),
      fret_margin_percentage: fret_margin_percentage.get(),
      nut_width: nut_width.get(),
      extra_frets: final_extra_frets.get(),
      marker_positions: marker_positions.get(),
      ..Default::default()
    }
  });

  // Extract start and end frets from the range for underlying components
  let start_fret = Signal::derive(move || *fret_range.get().start());
  let end_fret = Signal::derive(move || *fret_range.get().end());

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
        fret_range=fret_range
        extra_frets=final_extra_frets
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
  /// Range of frets to show scale notes on
  fret_range: Signal<std::ops::RangeInclusive<usize>>,
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
          let current_fret_range = fret_range.get();
          let current_extra_frets = extra_frets.get();
          let current_num_strings = num_strings.get();
          let base_width = 800.0;
          let height = base_width / svg_aspect_ratio.get();
          let current_nut_width = nut_width.get();

          // Extract start and end from range for visual calculations (extra frets, etc.)
          let current_start_fret = *current_fret_range.start();
          let current_end_fret = *current_fret_range.end();

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
              // Only render scale notes in the specified fret range (clean and simple!)
              for fret_idx in current_fret_range.clone() {
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
                    // Fret position: place in the middle of the playable area between fret lines
                    // This matches the clickable area positioning: (x_prev + x_curr) / 2.0
                    let x_prev = if fret_idx == 0 { 0.0 } else { full_fret_positions[(fret_idx - 1).max(0)] };
                    let x_curr = full_fret_positions[fret_idx];
                    let playable_position = (x_prev + x_curr) / 2.0;
                    to_viewbox_x(playable_position)
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
