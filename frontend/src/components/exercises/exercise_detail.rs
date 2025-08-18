use crate::components::exercises::{ExerciseForm, FormMode};
use crate::components::PracticeSession;
use crate::models::repository::{get_exercise_repository, ExerciseRepository};
use leptos::prelude::*;
use shared::models::exercise::{Exercise, ExerciseType};

#[component]
pub fn ExerciseDetail(#[prop(into)] exercise_id: String) -> impl IntoView {
  // Exercise state - using signal to track changes
  let (exercise, set_exercise) = signal(None::<Exercise>);
  let (is_editing, set_is_editing) = signal(false);

  // Inline description editing state
  let (is_editing_description, set_is_editing_description) = signal(false);
  let (description_edit_value, set_description_edit_value) = signal(String::new());

  // Inline title editing state
  let (is_editing_title, set_is_editing_title) = signal(false);
  let (title_edit_value, set_title_edit_value) = signal(String::new());

  // Load exercise on component mount
  Effect::new(move |_| {
    let id = exercise_id.clone();
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

  // Generate title from exercise type
  let generate_title_from_exercise = move |exercise_type: &ExerciseType| -> String {
    match exercise_type {
      ExerciseType::Scale {
        root_note,
        scale_type,
        ..
      } => {
        format!("{} {} Scale", root_note, scale_type)
      }
      ExerciseType::Triad {
        root_note,
        scale_type,
        ..
      } => {
        format!("{} {} Triad", root_note, scale_type)
      }
      _ => "Exercise".to_string(),
    }
  };

  // Handle inline title editing
  let start_title_edit = move |current_title: String| {
    set_title_edit_value.set(current_title);
    set_is_editing_title.set(true);
  };

  let save_title_edit = move || {
    if let Some(mut ex) = exercise.get() {
      let new_title = title_edit_value.get().trim().to_string();
      if !new_title.is_empty() {
        ex.name = new_title;

        // Update in storage
        let repo = get_exercise_repository();
        if let Ok(()) = repo.update(&ex) {
          set_exercise.set(Some(ex));
          set_is_editing_title.set(false);
        }
      }
    }
  };

  let cancel_title_edit = move || {
    set_is_editing_title.set(false);
  };

  let generate_title = move || {
    if let Some(ex) = exercise.get() {
      let generated_title = generate_title_from_exercise(&ex.exercise_type);
      set_title_edit_value.set(generated_title);
    }
  };

  view! {
    <div class="container py-8 px-4 mx-auto">
      {move || match exercise.get() {
        Some(ex) => {
          let ex_for_title = ex.clone();
          let ex_for_description = ex.clone();
          let ex_for_practice = ex.clone();
          if is_editing.get() {

            view! {
              <div class="mb-8">
                <ExerciseForm
                  mode=FormMode::Edit(ex)
                  on_save=handle_exercise_update
                  on_cancel=handle_edit_cancel
                />
              </div>
            }
              .into_any()
          } else {
            view! {
              <div>
                <div class="mb-8">
                  <div class="flex justify-between items-start mb-4">
                    // Title section with inline editing
                    <div class="flex-1 mr-4">
                      {move || {
                        let ex_title = ex_for_title.clone();
                        if is_editing_title.get() {
                          // Edit mode - show input and buttons
                          view! {
                            <div class="space-y-2">
                              <input
                                type="text"
                                class="w-full text-3xl font-bold bg-transparent border-b-2 border-blue-500 focus:outline-none"
                                prop:value=move || title_edit_value.get()
                                on:input=move |e| {
                                  set_title_edit_value.set(event_target_value(&e))
                                }
                                placeholder="Enter exercise title"
                              />
                              <div class="flex justify-between items-center">
                                <div class="flex space-x-2">
                                  {move || {
                                    let ex_title_inner = ex_title.clone();
                                    if let Some(ref ex_title_check) = Some(ex_title_inner.clone()) {
                                      match ex_title_check.exercise_type {
                                        ExerciseType::Scale { .. } | ExerciseType::Triad { .. } => {
                                          // Only show generate button for Scale and Triad types
                                          view! {
                                            <button
                                              class="py-1 px-3 text-sm text-blue-600 rounded border border-blue-600 hover:bg-blue-50"
                                              on:click=move |_| generate_title()
                                              title="Generate title from exercise details"
                                            >
                                              "Generate"
                                            </button>
                                          }
                                            .into_any()
                                        }
                                        _ => view! { <div></div> }.into_any(),
                                      }
                                    } else {
                                      view! { <div></div> }.into_any()
                                    }
                                  }}
                                </div>
                                <div class="flex space-x-2">
                                  <button
                                    class="py-1 px-3 text-sm text-gray-600 rounded border border-gray-300 hover:bg-gray-50"
                                    on:click=move |_| cancel_title_edit()
                                  >
                                    "Cancel"
                                  </button>
                                  <button
                                    class="py-1 px-3 text-sm text-white bg-blue-500 rounded hover:bg-blue-600"
                                    on:click=move |_| save_title_edit()
                                  >
                                    "Save"
                                  </button>
                                </div>
                              </div>
                            </div>
                          }
                            .into_any()
                        } else {
                          let title_for_edit = ex_title.name.clone();
                          // Display mode - show title with hover effect
                          view! {
                            <h1
                              class="relative text-3xl font-bold transition-colors duration-200 cursor-pointer hover:text-blue-600 group"
                              on:click=move |_| start_title_edit(title_for_edit.clone())
                              title="Click to edit title"
                            >
                              {ex_title.name.clone()}
                              <svg
                                class="inline-block ml-2 w-5 h-5 text-gray-400 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                              >
                                <path
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                  stroke-width="2"
                                  d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                                />
                              </svg>
                            </h1>
                          }
                            .into_any()
                        }
                      }}
                    </div>

                    <button
                      class="py-2 px-4 text-white bg-blue-500 rounded-md hover:bg-blue-600"
                      on:click=move |_| set_is_editing.set(true)
                    >
                      "Edit Exercise"
                    </button>
                  </div>

                  // Description section with inline editing
                  <div class="mb-4">
                    {move || {
                      let ex_desc = ex_for_description.clone();
                      if is_editing_description.get() {
                        // Edit mode - show textarea and buttons
                        view! {
                          <div class="space-y-2">
                            <textarea
                              class="py-2 px-3 w-full rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                              prop:value=move || description_edit_value.get()
                              on:input=move |e| {
                                set_description_edit_value.set(event_target_value(&e))
                              }
                              placeholder="Enter exercise description (optional)"
                              rows="3"
                            />
                            <div class="flex justify-end space-x-2">
                              <button
                                class="py-1 px-3 text-sm text-gray-600 rounded border border-gray-300 hover:bg-gray-50"
                                on:click=move |_| cancel_description_edit()
                              >
                                "Cancel"
                              </button>
                              <button
                                class="py-1 px-3 text-sm text-white bg-blue-500 rounded hover:bg-blue-600"
                                on:click=move |_| save_description_edit()
                              >
                                "Save"
                              </button>
                            </div>
                          </div>
                        }
                          .into_any()
                      } else {
                        match ex_desc.description.as_ref() {
                          Some(desc) => {
                            let desc_for_edit = desc.clone();
                            // Display mode - show description or placeholder with edit button
                            view! {
                              <div
                                class="relative p-3 rounded-lg border-2 border-transparent transition-all duration-200 cursor-pointer hover:bg-gray-50 hover:border-gray-300 group"
                                on:click=move |_| start_description_edit(desc_for_edit.clone())
                                title="Click to edit description"
                              >
                                <p class="pr-8 text-gray-600">{desc.clone()}</p>
                                <div class="absolute top-2 right-2 text-gray-400 transition-colors duration-200 group-hover:text-gray-600">
                                  <svg
                                    class="w-4 h-4"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                  >
                                    <path
                                      stroke-linecap="round"
                                      stroke-linejoin="round"
                                      stroke-width="2"
                                      d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                                    />
                                  </svg>
                                </div>
                              </div>
                            }
                              .into_any()
                          }
                          None => {
                            view! {
                              <div
                                class="p-3 rounded-lg border-2 border-gray-300 border-dashed transition-all duration-200 cursor-pointer hover:bg-gray-50 hover:border-gray-400"
                                on:click=move |_| start_description_edit(String::new())
                                title="Click to add description"
                              >
                                <p class="italic text-gray-400 transition-colors duration-200 hover:text-gray-600">
                                  "+ Add description"
                                </p>
                              </div>
                            }
                              .into_any()
                          }
                        }
                      }
                    }}
                  </div>

                  // Colored configuration buttons with dropdowns - REMOVED
                  // Now handled by the PracticeSession component

                </div>

                // Practice Session Section
                <div class="p-6 bg-gray-50 rounded-lg">
                  <PracticeSession
                    target_time=std::time::Duration::from_secs(15 * 60)
                    exercise=ex_for_practice
                    on_exercise_update=Callback::new(move |updated_exercise: Exercise| {
                      // Update the exercise in the repository
                      let repo = get_exercise_repository();
                      if let Err(e) = repo.update(&updated_exercise) {
                        leptos::logging::error!("Failed to update exercise: {:?}", e);
                      } else {
                        // Update the local signal
                        set_exercise.set(Some(updated_exercise));
                      }
                    })
                  />
                </div>
              </div>
            }
              .into_any()
          }
        }
        None => {

          // Description section with inline editing
          // Edit mode - show textarea and buttons
          // Display mode - show description or placeholder with edit button

          // Practice Session Section
          view! {
            <div class="py-16 text-center">
              <h2 class="mb-4 text-2xl font-bold text-gray-600">Exercise Not Found</h2>
              <p class="mb-8 text-gray-500">The exercise you are looking for does not exist.</p>
              <a
                href="/exercises"
                class="py-2 px-6 text-white bg-blue-500 rounded hover:bg-blue-600"
              >
                "Back to Exercises"
              </a>
            </div>
          }
            .into_any()
        }
      }}
    </div>
  }
}
