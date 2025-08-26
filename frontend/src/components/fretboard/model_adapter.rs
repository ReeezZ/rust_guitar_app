use leptos::prelude::*;

use super::{FretClickEvent, Fretboard};
use crate::models::fretboard::model::FretboardModel;

#[component]
pub fn FretboardModelAdapter(
  #[prop(into)] model: Signal<FretboardModel>,
  /// Optional callback for fret click events (passed separately, not from model)
  #[prop(optional, into)]
  on_note_clicked: Option<Callback<FretClickEvent>>,
) -> impl IntoView {
  let start_fret = Signal::derive(move || model.with(|m| m.get_start_fret()).get());
  let end_fret = Signal::derive(move || model.with(|m| m.get_end_fret()).get());
  let tuning = Signal::derive(move || model.with(|m| m.get_tuning()).get());
  let config = Signal::derive(move || model.with(|m| m.get_config()).get());
  let fret_states = Signal::derive(move || model.with(|m| m.get_fret_states()).get());

  // Convert optional callback to signal
  let on_note_clicked_signal = Signal::derive(move || on_note_clicked);

  view! {
    <Fretboard
      start_fret
      end_fret
      tuning
      config
      on_note_clicked=on_note_clicked_signal
      fret_states
    />
  }
}
