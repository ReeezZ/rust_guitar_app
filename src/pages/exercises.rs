use crate::components::{
  exercise_manager::SongsList, exercises::scale::start_scale::StartScaleDialog,
};
use leptos::prelude::*;

#[component]
pub fn ExercisesPage() -> impl IntoView {
  view! {
    <StartScaleDialog />
    <SongsList />
  }
}
