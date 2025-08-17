use crate::components::fretboard::base::{FretClickEvent, Fretboard};
use crate::components::fretboard::visual_config::FretboardVisualConfig;
use crate::models::fretboard_model::{FretCoord, FretState, FretboardModel};
use shared::music::notes::Note;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct FretClickEventWithNote {
  pub note: Note,
  pub coord: FretCoord,
  pub fret_state: FretState,
}

/// SVG fretboard component that adds note awareness to the base SvgFretboard.
///
/// This component wraps the visual-only `SvgFretboard` and enriches click events
/// with musical note information by calculating notes from fret coordinates and tuning.
///
/// # Example - Basic Usage
///
/// ```rust
/// # use leptos::prelude::*;
/// # use rust_guitar_app::components::svg_fretboard_with_notes::SvgFretboardWithNotes;
/// # use rust_guitar_app::components::fretboard::base::FretClickEvent;
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
///
/// # Example - With Custom Visual Configuration
///
/// ```rust
/// # use leptos::prelude::*;
/// # use rust_guitar_app::components::svg_fretboard_with_notes::SvgFretboardWithNotes;
/// # use rust_guitar_app::components::fretboard::visual_config::FretboardVisualConfig;
/// # use rust_guitar_app::components::fretboard::base::FretClickEvent;
///
/// # fn example() -> impl IntoView {
/// let start = RwSignal::new(0);
/// let end = RwSignal::new(12);
///
/// // Bass guitar with wider aspect ratio for better visibility
/// let bass_config = FretboardVisualConfig::bass_guitar()
///   .with_aspect_ratio(4.0)
///   .with_max_frets(24);
///
/// view! {
///   <SvgFretboardWithNotes
///     start_fret=start.into()
///     end_fret=end.into()
///     config=bass_config
///   />
/// }
/// # }
/// ```
#[component]
pub fn FretboardWithNotes(
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
  on_note_clicked: Option<Callback<FretClickEventWithNote>>,

  // Visual configuration
  /// Visual configuration for fretboard display properties
  #[prop(optional, into)]
  config: Option<Signal<FretboardVisualConfig>>,
) -> impl IntoView {
  // Use default tuning if not provided (standard guitar tuning)
  let tuning = tuning.unwrap_or_else(|| Signal::derive(FretboardModel::standard_tuning));

  // Handle coordinate-to-note conversion
  let on_svg_fret_clicked = Callback::new(move |svg_event: FretClickEvent| {
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
        let fret_click_event = FretClickEventWithNote {
          note,
          coord: svg_event.coord,
          fret_state: FretState::Normal, // Default state, could be enhanced later
        };

        callback.run(fret_click_event);
      }
    }
  });

  view! {
    <Fretboard
      start_fret=start_fret
      end_fret=end_fret
      config=config.unwrap_or_else(|| Signal::derive(FretboardVisualConfig::default))
      on_fret_clicked=on_svg_fret_clicked
    />
  }
}
