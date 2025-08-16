use crate::components::exercise_form::{ExerciseForm, FormMode};
use crate::models::{exercise::Exercise, storage};
use leptos::prelude::*;

#[component]
pub fn ExerciseManager() -> impl IntoView {
  let (exercises, set_exercises) = signal(storage::load_exercises());
  let (show_form, set_show_form) = signal(false);

  let delete_exercise = move |exercise_id: String| {
    storage::delete_exercise(&exercise_id);
    set_exercises.update(|exercises| exercises.retain(|e| e.id != exercise_id));
  };

  // Handle exercise save from form
  let handle_exercise_save = Callback::new(move |exercise: Exercise| {
    set_exercises.update(|exercises| {
      // Remove existing exercise if updating, then add the new one
      exercises.retain(|e| e.id != exercise.id);
      exercises.push(exercise);
    });
    set_show_form.set(false);
  });

  // Handle form cancel
  let handle_form_cancel = Callback::new(move |_: ()| {
    set_show_form.set(false);
  });

  view! {
    <div class="p-6 mx-auto max-w-4xl">
      <div class="flex justify-between items-center mb-6">
        <h1 class="text-3xl font-bold text-gray-800">"My Exercises"</h1>
        <button
          class="py-2 px-4 font-bold text-white bg-blue-500 rounded hover:bg-blue-700"
          on:click=move |_| set_show_form.set(!show_form.get())
        >
          {move || if show_form.get() { "Cancel" } else { "Add Exercise" }}
        </button>
      </div>

      // Add Exercise Form
      {move || {
        if show_form.get() {
          view! {
            <div class="mb-6">
              <ExerciseForm
                mode={FormMode::Create}
                on_save={handle_exercise_save}
                on_cancel={handle_form_cancel}
              />
            </div>
          }.into_any()
        } else {
          view! { <div></div> }.into_any()
        }
      }}

      // Exercise List
      <div class="space-y-4">
        <For each=move || exercises.get() key=|exercise| exercise.id.clone() let:exercise>
          <div class="p-4 bg-white rounded-lg border border-gray-200 shadow-sm">
            <div class="flex justify-between items-center">
              <div>
                <h3 class="text-lg font-semibold text-gray-800">{exercise.name.clone()}</h3>
                <p class="mt-1 text-sm text-gray-600">
                  "Type: " {exercise.exercise_type.type_name()}
                </p>
                {exercise
                  .description
                  .as_ref()
                  .map(|desc| view! { <p class="mt-1 text-sm text-gray-500">{desc.clone()}</p> })}
              </div>
              <div class="flex items-center space-x-2">
                <a
                  href=format!("/exercises/{}", exercise.id)
                  class="flex justify-center items-center py-1.5 px-3 text-sm font-medium text-white bg-blue-500 rounded hover:bg-blue-700"
                >
                  "View"
                </a>
                <button
                  class="py-1.5 px-2 text-sm font-medium text-red-500 hover:text-red-700"
                  on:click={
                    let exercise_id = exercise.id.clone();
                    move |_| delete_exercise(exercise_id.clone())
                  }
                >
                  "Delete"
                </button>
              </div>
            </div>
          </div>
        </For>

        {move || {
          exercises
            .get()
            .is_empty()
            .then(|| {
              view! {
                <div class="py-8 text-center text-gray-500">
                  <p>"No exercises yet. Create your first exercise to get started!"</p>
                </div>
              }
            })
        }}
      </div>
    </div>
  }
}
