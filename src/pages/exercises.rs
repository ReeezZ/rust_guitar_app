use crate::components::{exercise_manager::SongsList, exercises::start_scale::StartScale};
use leptos::prelude::*;

#[component]
pub fn ExercisesPage() -> impl IntoView {
  view! {
    <StartScale />
    <SongsList />
  }
}
