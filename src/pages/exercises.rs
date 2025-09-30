use crate::{
  components::{
    exercise_manager::SongsList,
    exercises::{start_scale::StartScale, StartScaleExerciseConfiguration},
    ui::{Button, ButtonVariant},
  },
  music::{heptatonic_scales::HeptaScaleType, Note, ScaleType},
};
use leptos::prelude::*;

#[component]
pub fn ExercisesPage() -> impl IntoView {
  view! {
    <StartScale />
    <SongsList />
  }
}
