use crate::components::exercises::{ExerciseForm, FormMode};
use crate::components::practice_timer::PracticeTimer;
use crate::models::exercise::Exercise;
use crate::models::repository::{get_exercise_repository, ExerciseRepository};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ExerciseDetail() -> impl IntoView {
  let params = use_params_map();
  let exercise_id = move || params.read().get("id").unwrap_or_default();

  // Exercise state - using signal to track changes
  let (exercise, set_exercise) = signal(None::<Exercise>);
  let (is_editing, set_is_editing) = signal(false);

  // Inline description editing state
  let (is_editing_description, set_is_editing_description) = signal(false);
  let (description_edit_value, set_description_edit_value) = signal(String::new());

  // Load exercise on component mount
  Effect::new(move |_| {
    let id = exercise_id();
    if !id.is_empty() {
      let repo = get_exercise_repository();
      if let Ok(Some(ex)) = repo.find_by_id(&id) {
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

  // Handle inline description editing
  let start_description_edit = move |current_description: String| {
    set_description_edit_value.set(current_description);
    set_is_editing_description.set(true);
  };

  let save_description_edit = move || {
    if let Some(mut ex) = exercise.get() {
      let new_description = description_edit_value.get().trim().to_string();
      ex.description = if new_description.is_empty() {
        None
      } else {
        Some(new_description)
      };

      // Update in storage
      let repo = get_exercise_repository();
      if let Ok(()) = repo.update(&ex) {
        set_exercise.set(Some(ex));
        set_is_editing_description.set(false);
      }
    }
  };

  let cancel_description_edit = move || {
    set_is_editing_description.set(false);
  };

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

                                  // Description section with inline editing
                                  <div class="mb-4">
                                      {move || {
                                          if is_editing_description.get() {
                                              // Edit mode - show textarea and buttons
                                              view! {
                                                  <div class="space-y-2">
                                                      <textarea
                                                          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                                          prop:value={move || description_edit_value.get()}
                                                          on:input=move |e| set_description_edit_value.set(event_target_value(&e))
                                                          placeholder="Enter exercise description (optional)"
                                                          rows="3"
                                                      />
                                                      <div class="flex justify-end space-x-2">
                                                          <button
                                                              class="px-3 py-1 text-sm text-gray-600 border border-gray-300 rounded hover:bg-gray-50"
                                                              on:click=move |_| cancel_description_edit()
                                                          >
                                                              "Cancel"
                                                          </button>
                                                          <button
                                                              class="px-3 py-1 text-sm bg-blue-500 text-white rounded hover:bg-blue-600"
                                                              on:click=move |_| save_description_edit()
                                                          >
                                                              "Save"
                                                          </button>
                                                      </div>
                                                  </div>
                                              }.into_any()
                                          } else {
                                              // Display mode - show description or placeholder with edit button
                                              match ex.description.as_ref() {
                                                  Some(desc) => {
                                                      let desc_for_edit = desc.clone();
                                                      view! {
                                                          <div
                                                              class="group cursor-pointer p-3 rounded-lg border-2 border-transparent hover:border-gray-300 hover:bg-gray-50 transition-all duration-200 relative"
                                                              on:click=move |_| start_description_edit(desc_for_edit.clone())
                                                              title="Click to edit description"
                                                          >
                                                              <p class="text-gray-600 pr-8">{desc.clone()}</p>
                                                              <div class="absolute top-2 right-2 text-gray-400 group-hover:text-gray-600 transition-colors duration-200">
                                                                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                                                                  </svg>
                                                              </div>
                                                          </div>
                                                      }.into_any()
                                                  }
                                                  None => {
                                                      view! {
                                                          <div
                                                              class="cursor-pointer p-3 rounded-lg border-2 border-dashed border-gray-300 hover:border-gray-400 hover:bg-gray-50 transition-all duration-200"
                                                              on:click=move |_| start_description_edit(String::new())
                                                              title="Click to add description"
                                                          >
                                                              <p class="text-gray-400 hover:text-gray-600 italic transition-colors duration-200">
                                                                  "+ Add description"
                                                              </p>
                                                          </div>
                                                      }.into_any()
                                                  }
                                              }
                                          }
                                      }}
                                  </div>

                                  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                                      <div class="bg-blue-50 p-4 rounded">
                                          <h3 class="font-semibold text-blue-800">Exercise Type</h3>
                                          <p class="text-blue-600">{ex.exercise_type.type_name()}</p>
                                      </div>

                                      <div class="bg-green-50 p-4 rounded">
                                          <h3 class="font-semibold text-green-800">Details</h3>
                                          <p class="text-green-600">{ex.exercise_type.to_string()}</p>
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
