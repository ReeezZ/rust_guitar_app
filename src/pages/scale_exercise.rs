use leptos::prelude::*;
use leptos_router::hooks::query_signal;

use crate::{
  components::exercises::scale::scale_practice_session::ScalePracticeSession,
  music::{heptatonic_scales::HeptaScaleType, Note, ScaleType},
};

#[component]
pub fn ScaleExercisePage() -> impl IntoView {
  // let scale_exercise_params = use_query::<ScaleExerciseParams>();
  let (root_note, set_root_note) = query_signal::<Note>("root_note");
  let (scale_type, set_scale_type) = query_signal::<ScaleType>("scale_type");
  let (min_fret, set_min_fret) = query_signal::<u8>("min_fret");
  let (max_fret, set_max_fret) = query_signal::<u8>("max_fret");

  let root_note = Signal::derive(move || root_note.get().unwrap_or(Note::C));
  let scale_type = Signal::derive(move || {
    scale_type
      .get()
      .unwrap_or(ScaleType::Hepatonic(HeptaScaleType::Major))
  });
  let min_fret = Signal::derive(move || min_fret.get().unwrap_or(0));
  let max_fret = Signal::derive(move || max_fret.get().unwrap_or(12));

  view! {
    <ScalePracticeSession
      root_note
      on_root_note_changed=Callback::new(move |note| set_root_note.set(Some(note)))
      scale_type
      on_scale_type_changed=Callback::new(move |scale| set_scale_type.set(Some(scale)))
      min_fret
      on_min_fret_changed=Callback::new(move |fret| set_min_fret.set(Some(fret)))
      max_fret
      on_max_fret_changed=Callback::new(move |fret| set_max_fret.set(Some(fret)))
    ></ScalePracticeSession>
  }
}
