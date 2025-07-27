use crate::components::fretboard::FretClickEvent;
use crate::components::svg_fretboard_with_notes::SvgFretboardWithNotes;
use crate::fretboard_view_helper::calculate_fret_positions;
use crate::models::fretboard_model::FretboardModel;
use crate::music::notes::Note;
use crate::music::scales::{Scale, ScaleTrait, ScaleType};
use leptos::prelude::*;

/// SVG fretboard component that adds scale visualization to the note-aware fretboard.
///
/// This component wraps `SvgFretboardWithNotes` and adds visual highlighting of scale notes
/// with different colors for root notes vs other scale notes.
///
/// # Example
///
/// ```rust
/// # use leptos::prelude::*;
/// # use rust_guitar_app::components::svg_fretboard_scale_display::SvgFretboardScaleDisplay;
/// # use rust_guitar_app::components::fretboard::FretClickEvent;
/// # use rust_guitar_app::music::notes::Note;
/// # use rust_guitar_app::music::scales::ScaleType;
///
/// # fn example() -> impl IntoView {
/// let start = RwSignal::new(3);
/// let end = RwSignal::new(7);
/// let root_note = RwSignal::new(Note::C);
/// let scale_type = RwSignal::new(ScaleType::Hepatonic(HeptaScaleType::Major));
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

  // Musical properties (passed through to SvgFretboardWithNotes)
  /// Guitar tuning (defaults to standard: E-A-D-G-H-E from lowest to highest string)
  #[prop(optional, into)]
  tuning: Option<Signal<Vec<Note>>>,
  /// Callback for note click events (enriched with note information)
  #[prop(optional)]
  on_note_clicked: Option<Callback<FretClickEvent>>,

  // All SvgFretboard visual props passed through
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
  // Use default tuning if not provided (standard guitar tuning)
  let tuning = tuning.unwrap_or_else(|| Signal::derive(move || FretboardModel::standard_tuning()));

  // Use signals if provided, otherwise use default values (same as SvgFretboard defaults)
  let resolved_num_strings = num_strings.unwrap_or_else(|| Signal::derive(move || 6_u8));
  let resolved_max_frets = max_frets.unwrap_or_else(|| Signal::derive(move || 22_usize));
  let resolved_svg_aspect_ratio =
    svg_aspect_ratio.unwrap_or_else(|| Signal::derive(move || 3.0_f64));
  let resolved_fret_margin_percentage =
    fret_margin_percentage.unwrap_or_else(|| Signal::derive(move || 0.05_f64));
  let resolved_nut_width = nut_width.unwrap_or_else(|| Signal::derive(move || 14.0_f64));
  let resolved_extra_frets = extra_frets.unwrap_or_else(|| Signal::derive(move || 1_usize));
  let resolved_marker_positions = marker_positions
    .unwrap_or_else(|| Signal::derive(move || vec![3_u8, 5, 7, 9, 12, 15, 17, 19, 21, 24]));

  // Create the scale from root note and scale type
  let scale = Memo::new(move |_| Scale::new(root_note.get(), scale_type.get()));

  view! {
    <div class="relative">
      // Base fretboard with note awareness
      <SvgFretboardWithNotes
        start_fret=start_fret
        end_fret=end_fret
        tuning=tuning
        on_note_clicked=on_note_clicked.unwrap_or_else(|| Callback::new(|_| {}))
        num_strings=resolved_num_strings
        max_frets=resolved_max_frets
        svg_aspect_ratio=resolved_svg_aspect_ratio
        fret_margin_percentage=resolved_fret_margin_percentage
        nut_width=resolved_nut_width
        extra_frets=resolved_extra_frets
        marker_positions=resolved_marker_positions
      />

      // Simple scale note overlays
      <ScaleNoteOverlays
        scale=scale
        tuning=tuning
        start_fret=start_fret
        end_fret=end_fret
        extra_frets=resolved_extra_frets
        num_strings=resolved_num_strings
        svg_aspect_ratio=resolved_svg_aspect_ratio
        fret_margin_percentage=resolved_fret_margin_percentage
        nut_width=resolved_nut_width
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
  fret_margin_percentage: Signal<f64>,
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
