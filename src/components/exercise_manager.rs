use crate::components::exercise_form::{ExerciseForm, FormMode};
use crate::models::{exercise::Exercise, repository::{get_exercise_repository, ExerciseRepository}};
use leptos::prelude::*;

#[component]
pub fn ExerciseManager() -> impl IntoView {
  let (exercises, set_exercises) = signal({
    let repo = get_exercise_repository();
    repo.find_all().unwrap_or_default()
  });
  let (show_form, set_show_form) = signal(false);
  let (show_delete_confirmation, set_show_delete_confirmation) = signal(false);
  let (pending_delete_exercise, set_pending_delete_exercise) = signal(None::<(String, String)>);

  // Show delete confirmation dialog
  let show_delete_dialog = move |exercise_id: String, exercise_name: String| {
    set_pending_delete_exercise.set(Some((exercise_id, exercise_name)));
    set_show_delete_confirmation.set(true);
  };

  // Confirm deletion
  let confirm_delete = move || {
    if let Some((exercise_id, _)) = pending_delete_exercise.get() {
      let repo = get_exercise_repository();
      let _ = repo.delete(&exercise_id); // Ignore errors for now
      set_exercises.update(|exercises| exercises.retain(|e| e.id != exercise_id));
      set_show_delete_confirmation.set(false);
      set_pending_delete_exercise.set(None);
    }
  };

  // Cancel deletion
  let cancel_delete = move || {
    set_show_delete_confirmation.set(false);
    set_pending_delete_exercise.set(None);
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
                mode=FormMode::Create
                on_save=handle_exercise_save
                on_cancel=handle_form_cancel
              />
            </div>
          }
            .into_any()
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
                  href=format!("/exercises/{id}", id = exercise.id)
                  class="flex justify-center items-center py-1.5 px-3 text-sm font-medium text-white bg-blue-500 rounded hover:bg-blue-700"
                >
                  "View"
                </a>
                <button
                  class="py-1.5 px-2 text-sm font-medium text-red-500 hover:text-red-700"
                  on:click={
                    let exercise_id = exercise.id.clone();
                    let exercise_name = exercise.name.clone();
                    move |_| show_delete_dialog(exercise_id.clone(), exercise_name.clone())
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

      // Delete confirmation dialog
      {move || {
        if show_delete_confirmation.get() {
          if let Some((_, exercise_name)) = pending_delete_exercise.get() {
            view! {
              <div class="flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-50">
                <div class="p-6 mx-4 max-w-md bg-white rounded-lg">
                  <h3 class="mb-3 text-lg font-semibold text-gray-800">Delete Exercise</h3>
                  <p class="mb-4 text-gray-600">
                    "Are you sure you want to delete '"
                    <span class="font-semibold">{exercise_name}</span> "'?"
                  </p>
                  <p class="mb-6 text-sm text-gray-500">"This action cannot be undone."</p>
                  <div class="flex justify-end space-x-3">
                    <button
                      class="py-2 px-4 text-gray-600 rounded-md border border-gray-300 hover:bg-gray-50"
                      on:click=move |_| cancel_delete()
                    >
                      Cancel
                    </button>
                    <button
                      class="py-2 px-4 text-white bg-red-500 rounded-md hover:bg-red-600"
                      on:click=move |_| confirm_delete()
                    >
                      Delete
                    </button>
                  </div>
                </div>
              </div>
            }
              .into_any()
          } else {
            view! { <div></div> }.into_any()
          }
        } else {
          view! { <div></div> }.into_any()
        }
      }}
    </div>
  }
}
