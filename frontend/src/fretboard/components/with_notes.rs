use leptos::prelude::*;
use shared::music::notes::Note;

use crate::fretboard::{
  base_model::{FretClickEvent, FretStateSignals},
  components::{base::Fretboard, visual_config::FretboardVisualConfig},
  with_notes_model::FretboardWithNotesModel,
  FretCoord,
};

#[derive(Clone, Copy, Debug)]
pub struct FretClickEventWithNote {
  pub note: Note,
  pub coord: FretCoord,
}

#[component]
pub fn FretboardWithNotesViewModel(model: FretboardWithNotesModel) -> impl IntoView {
  view! {
    <FretboardWithNotes
      start_fret=model.start_fret
      end_fret=model.end_fret
      num_strings=model.num_strings
      tuning=model.tuning
      on_note_clicked=model.on_note_clicked
      config=model.config.into()
      fret_states=model.fret_states
    />
  }

  // Build the view using the start and end fret signals
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
/// # use frontend::components::fretboard::with_notes::{FretboardWithNotes, FretClickEventWithNote};
///
/// # fn example() -> impl IntoView {
/// let start = RwSignal::new(3);
/// let end = RwSignal::new(7);
///
/// let on_note_clicked = Callback::new(move |event: FretClickEventWithNote| {
///   leptos::logging::log!("Clicked note: {} at fret {}, string {}",
///     event.note, event.coord.fret_idx, event.coord.string_idx);
/// });
///
/// view! {
///   <FretboardWithNotes
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
/// # use frontend::components::fretboard::with_notes::FretboardWithNotes;
/// # use frontend::components::fretboard::visual_config::FretboardVisualConfig;
/// # use frontend::components::fretboard::base::FretClickEvent;
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
///   <FretboardWithNotes
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
  #[prop(into)]
  start_fret: Signal<usize>,
  /// Last fret in the active/playable range
  #[prop(into)]
  end_fret: Signal<usize>,
  /// Number of guitar strings (default: 6)
  #[prop(into)]
  num_strings: Signal<u8>,

  // Musical properties
  /// Guitar tuning (defaults to standard: E-A-D-G-B-E from lowest to highest string)
  #[prop(optional, into)]
  tuning: Option<Signal<Vec<Note>>>,
  /// Callback for note click events (enriched with note information)
  #[prop(optional, into)]
  on_note_clicked: Signal<Option<Callback<FretClickEventWithNote>>>,

  /// Visual configuration for fretboard display properties
  config: Signal<FretboardVisualConfig>,

  /// Callback for fret click events
  #[prop(optional, into)]
  fret_states: Signal<FretStateSignals>,
) -> impl IntoView {
  // Use default tuning if not provided (standard guitar tuning)
  let tuning = tuning.unwrap_or_else(|| Signal::derive(FretboardWithNotesModel::standard_tuning));

  // Handle coordinate-to-note conversion
  let on_fret_clicked = Signal::derive(move || {
    if let Some(callback) = on_note_clicked.get() {
      Some(Callback::new(move |svg_event: FretClickEvent| {
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
          };

          callback.run(fret_click_event);
        }
      }))
    } else {
      None
    }
  });

  view! {
    <Fretboard
      start_fret=start_fret
      end_fret=end_fret
      num_strings=num_strings
      config=config
      on_fret_clicked=on_fret_clicked
      fret_states=fret_states
    />
  }
}
