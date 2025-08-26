use crate::components::exercises::exercise_detail::ExerciseDetail;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ExerciseDetailPage() -> impl IntoView {
  let params = use_params_map();
  let exercise_id = move || params.read_untracked().get("id").unwrap_or_default();

  view! { <ExerciseDetail exercise_id=exercise_id() /> }
}
