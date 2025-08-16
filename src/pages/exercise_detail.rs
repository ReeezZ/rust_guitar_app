use crate::components::exercise_form::{ExerciseForm, FormMode};
use crate::components::practice_timer::PracticeTimer;
use crate::models::exercise::Exercise;
use crate::models::storage::load_exercise_by_id;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ExerciseDetail() -> impl IntoView {
  let params = use_params_map();
  let exercise_id = move || params.read().get("id").unwrap_or_default();

  // Exercise state - using signal to track changes
  let (exercise, set_exercise) = signal(None::<Exercise>);
  let (is_editing, set_is_editing) = signal(false);

  // Load exercise on component mount
  Effect::new(move |_| {
    let id = exercise_id();
    if !id.is_empty() {
      if let Some(ex) = load_exercise_by_id(&id) {
        set_exercise.set(Some(ex));
      }
    }
  });

  // Handle exercise update from form
  let handle_exercise_update = Callback::new(move |updated_exercise: Exercise| {
    set_exercise.set(Some(updated_exercise));
    set_is_editing.set(false);
  });

  // Handle form cancel
  let handle_edit_cancel = Callback::new(move |_: ()| {
    set_is_editing.set(false);
  });

  view! {
      <div class="container mx-auto px-4 py-8">
          {move || match exercise.get() {
              Some(ex) => {
                  if is_editing.get() {
                      view! {
                          <div class="mb-8">
                              <ExerciseForm
                                  mode={FormMode::Edit(ex)}
                                  on_save={handle_exercise_update}
                                  on_cancel={handle_edit_cancel}
                              />
                          </div>
                      }.into_any()
                  } else {
                      view! {
                          <div>
                              <div class="mb-8">
                                  <div class="flex justify-between items-start mb-4">
                                      <h1 class="text-3xl font-bold">{ex.name.clone()}</h1>
                                      <button
                                          class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600"
                                          on:click=move |_| set_is_editing.set(true)
                                      >
                                          "Edit Exercise"
                                      </button>
                                  </div>

                                  {ex.description.as_ref().map(|desc| {
                                      view! { <p class="text-gray-600 mb-4">{desc.clone()}</p> }
                                  })}

                                  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                                      <div class="bg-blue-50 p-4 rounded">
                                          <h3 class="font-semibold text-blue-800">Exercise Type</h3>
                                          <p class="text-blue-600">{ex.exercise_type.type_name()}</p>
                                      </div>

                                      <div class="bg-green-50 p-4 rounded">
                                          <h3 class="font-semibold text-green-800">Details</h3>
                                          <p class="text-green-600">{format!("{}", ex.exercise_type)}</p>
                                      </div>
                                  </div>

                                  {ex.exercise_type.get_fret_range().map(|(min, max)| {
                                      view! {
                                          <div class="bg-orange-50 p-3 rounded mb-6">
                                              <h3 class="font-semibold text-orange-800">Fret Range</h3>
                                              <p class="text-orange-600">Frets {min} - {max}</p>
                                          </div>
                                      }
                                  })}
                              </div>

                              // Practice Timer Section
                              <div class="bg-gray-50 p-6 rounded-lg">
                                  <h2 class="text-xl font-semibold mb-4">Practice Timer</h2>
                                  <PracticeTimer target_time={std::time::Duration::from_secs(15 * 60)} />
                              </div>
                          </div>
                      }.into_any()
                  }
              }
              None => view! {
                  <div class="text-center py-16">
                      <h2 class="text-2xl font-bold text-gray-600 mb-4">Exercise Not Found</h2>
                      <p class="text-gray-500 mb-8">The exercise you are looking for does not exist.</p>
                      <a href="/exercises" class="bg-blue-500 text-white px-6 py-2 rounded hover:bg-blue-600">
                          "Back to Exercises"
                      </a>
                  </div>
              }.into_any()
          }}
      </div>
  }
}
