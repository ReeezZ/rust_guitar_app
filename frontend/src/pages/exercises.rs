use crate::components::exercise_manager::ExerciseManager;
use leptos::*;

#[component]
pub fn ExercisesPage() -> impl IntoView {
  view! {
      <ExerciseManager />
  }
}
