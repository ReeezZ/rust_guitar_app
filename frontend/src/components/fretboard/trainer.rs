use crate::{
  components::fretboard::{FretState, FretboardViewModel},
  models::fretboard_model::{FretClickEvent, FretCoord, FretboardModel},
};
use leptos::prelude::*;
use shared::music::notes::Note;

#[component]
pub fn FretboardTrainer(
  /// The reference note coordinate to highlight in green (None = no highlight)
  #[prop(into)]
  reference_note: Signal<Option<FretCoord>>,
  /// The note at the reference coordinate (for display)
  #[prop(into)]
  reference_note_name: Signal<Option<Note>>,
  /// Coordinates of incorrect guesses to highlight in red
  #[prop(into)]
  error_notes: Signal<Vec<(FretCoord, FretState)>>,

  #[prop(into)] model: Signal<FretboardModel>,
) -> impl IntoView {
  view! {
    <div>
      <FretboardViewModel model />
    </div>
  }
}
