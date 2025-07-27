use leptos::prelude::*;
use crate::components::svg_fretboard::{SvgFretboard, SvgFretClickEvent};
use crate::components::fretboard::FretClickEvent;
use crate::models::fretboard_model::{FretState, FretboardModel};
use crate::music::notes::Note;

/// SVG fretboard component that adds note awareness to the base SvgFretboard.
/// 
/// This component wraps the visual-only `SvgFretboard` and enriches click events
/// with musical note information by calculating notes from fret coordinates and tuning.
///
/// # Example
///
/// ```rust
/// # use leptos::prelude::*;
/// # use rust_guitar_app::components::svg_fretboard_with_notes::SvgFretboardWithNotes;
/// # use rust_guitar_app::components::fretboard::FretClickEvent;
///
/// # fn example() -> impl IntoView {
/// let start = RwSignal::new(3);
/// let end = RwSignal::new(7);
///
/// let on_note_clicked = Callback::new(move |event: FretClickEvent| {
///   leptos::logging::log!("Clicked note: {} at fret {}, string {}", 
///     event.note, event.coord.fret_idx, event.coord.string_idx);
/// });
///
/// view! {
///   <SvgFretboardWithNotes
///     start_fret=start.into()
///     end_fret=end.into()
///     on_note_clicked=on_note_clicked
///   />
/// }
/// # }
/// ```
#[component]
pub fn SvgFretboardWithNotes(
  /// First fret in the active/playable range
  start_fret: Signal<usize>,
  /// Last fret in the active/playable range
  end_fret: Signal<usize>,
  
  // Musical properties
  /// Guitar tuning (defaults to standard: E-A-D-G-B-E from lowest to highest string)
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
  let tuning = tuning.unwrap_or_else(|| {
    Signal::derive(move || FretboardModel::standard_tuning())
  });

  // Use signals if provided, otherwise use default values (same as SvgFretboard defaults)
  let resolved_num_strings = num_strings.unwrap_or_else(|| Signal::derive(move || 6_u8));
  let resolved_max_frets = max_frets.unwrap_or_else(|| Signal::derive(move || 22_usize));
  let resolved_svg_aspect_ratio = svg_aspect_ratio.unwrap_or_else(|| Signal::derive(move || 3.0_f64));
  let resolved_fret_margin_percentage = fret_margin_percentage.unwrap_or_else(|| Signal::derive(move || 0.05_f64));
  let resolved_nut_width = nut_width.unwrap_or_else(|| Signal::derive(move || 14.0_f64));
  let resolved_extra_frets = extra_frets.unwrap_or_else(|| Signal::derive(move || 1_usize));
  let resolved_marker_positions = marker_positions.unwrap_or_else(|| Signal::derive(move || vec![3_u8, 5, 7, 9, 12, 15, 17, 19, 21, 24]));

  // Handle coordinate-to-note conversion
  let on_svg_fret_clicked = Callback::new(move |svg_event: SvgFretClickEvent| {
    if let Some(callback) = on_note_clicked {
      let tuning_vec = tuning.get();
      let string_idx = svg_event.coord.string_idx;
      let fret_idx = svg_event.coord.fret_idx;
      
      // Calculate note from tuning and fret position
      // NOTE: Reverse the string index because SVG layout is top-to-bottom (high to low)
      // but tuning array is low-to-high [E, A, D, G, H, E]
      let tuning_index = (tuning_vec.len() - 1) - (string_idx as usize);
      if let Some(base_note) = tuning_vec.get(tuning_index) {
        let note = base_note.add_steps(fret_idx as usize);
        
        // Create enriched event with note information
        let fret_click_event = FretClickEvent {
          note,
          coord: svg_event.coord,
          fret_state: FretState::Normal, // Default state, could be enhanced later
        };
        
        callback.run(fret_click_event);
      }
    }
  });

  view! {
    <SvgFretboard
      start_fret=start_fret
      end_fret=end_fret
      num_strings=resolved_num_strings
      max_frets=resolved_max_frets
      svg_aspect_ratio=resolved_svg_aspect_ratio
      fret_margin_percentage=resolved_fret_margin_percentage
      nut_width=resolved_nut_width
      extra_frets=resolved_extra_frets
      marker_positions=resolved_marker_positions
      on_fret_clicked=on_svg_fret_clicked
    />
  }
}
