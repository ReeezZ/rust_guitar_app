use crate::models::{
  exercise::{Exercise, ExerciseType},
  storage,
};
use crate::music::{heptatonic_scales::HeptaScaleType, notes::Note, scales::ScaleType};
use leptos::prelude::*;

#[component]
pub fn ExerciseManager() -> impl IntoView {
  let (exercises, set_exercises) = signal(Vec::<Exercise>::new());
  let (show_form, set_show_form) = signal(false);

  // Form fields
  let (name, set_name) = signal(String::new());
  let (exercise_type_str, set_exercise_type_str) = signal("Technique".to_string());

  let add_exercise = move |_| {
    web_sys::console::log_1(&"Add exercise button clicked!".into());

    let exercise_type = match exercise_type_str.get().as_str() {
      "Scale" => ExerciseType::Scale {
        root_note: Note::C,
        scale_type: ScaleType::Hepatonic(HeptaScaleType::Major),
        fret_range: (0, 12),
      },
      "Triad" => ExerciseType::Triad {
        root_note: Note::C,
        scale_type: ScaleType::Hepatonic(HeptaScaleType::Major),
        fret_range: (0, 5),
      },
      "Song" => ExerciseType::Song,
      _ => ExerciseType::Technique,
    };

    let exercise = Exercise::new(name.get(), exercise_type);
    web_sys::console::log_1(&format!("Created exercise: {:?}", exercise).into());

    storage::save_exercise(&exercise);
    web_sys::console::log_1(&"Saved exercise to storage".into());

    // Add to our list
    set_exercises.update(|exercises| exercises.push(exercise));
    web_sys::console::log_1(&"Updated exercises list".into());

    // Reset form
    set_name.set(String::new());
    set_show_form.set(false);
  };

  let delete_exercise = move |exercise_id: String| {
    storage::delete_exercise(&exercise_id);
    set_exercises.update(|exercises| exercises.retain(|e| e.id != exercise_id));
  };

  view! {
      <div class="max-w-4xl mx-auto p-6">
          <div class="flex justify-between items-center mb-6">
              <h1 class="text-3xl font-bold text-gray-800">"My Exercises"</h1>
              <button
                  class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                  on:click=move |_| set_show_form.set(!show_form.get())
              >
                  {move || if show_form.get() { "Cancel" } else { "Add Exercise" }}
              </button>
          </div>

          // Add Exercise Form
          {move || show_form.get().then(|| view! {
              <div class="bg-gray-50 p-4 rounded-lg mb-6">
                  <h2 class="text-xl font-semibold mb-4">"New Exercise"</h2>
                  <div class="space-y-4">
                      <div>
                          <label class="block text-sm font-medium text-gray-700 mb-2">
                              "Exercise Name"
                          </label>
                          <input
                              type="text"
                              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                              placeholder="e.g., C Major Scale Practice"
                              prop:value=name
                              on:input=move |ev| set_name.set(event_target_value(&ev))
                          />
                      </div>

                      <div>
                          <label class="block text-sm font-medium text-gray-700 mb-2">
                              "Exercise Type"
                          </label>
                          <select
                              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                              on:change=move |ev| set_exercise_type_str.set(event_target_value(&ev))
                          >
                              <option value="Technique">"Technique"</option>
                              <option value="Scale">"Scale"</option>
                              <option value="Triad">"Triad"</option>
                              <option value="Song">"Song"</option>
                          </select>
                      </div>

                      <div class="flex space-x-2">
                          <button
                              class="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded disabled:bg-gray-400"
                              on:click=add_exercise
                              disabled=move || name.get().trim().is_empty()
                          >
                              "Add Exercise"
                          </button>
                      </div>
                  </div>
              </div>
          })}

          // Exercise List
          <div class="space-y-4">
              <For
                  each=move || exercises.get()
                  key=|exercise| exercise.id.clone()
                  let:exercise
              >
                  <div class="bg-white border border-gray-200 rounded-lg p-4 shadow-sm">
                      <div class="flex justify-between items-start">
                          <div>
                              <h3 class="text-lg font-semibold text-gray-800">{exercise.name.clone()}</h3>
                              <p class="text-sm text-gray-600 mt-1">
                                  "Type: " {exercise.exercise_type.type_name()}
                              </p>
                              {exercise.description.as_ref().map(|desc| view! {
                                  <p class="text-sm text-gray-500 mt-1">{desc.clone()}</p>
                              })}
                          </div>
                          <div class="flex space-x-2">
                              <a
                                  href={format!("/exercises/{}", exercise.id)}
                                  class="bg-blue-500 hover:bg-blue-700 text-white font-medium text-sm py-1 px-3 rounded"
                              >
                                  "View"
                              </a>
                              <button
                                  class="text-red-500 hover:text-red-700 font-medium text-sm"
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

              {move || exercises.get().is_empty().then(|| view! {
                  <div class="text-center py-8 text-gray-500">
                      <p>"No exercises yet. Create your first exercise to get started!"</p>
                  </div>
              })}
          </div>
      </div>
  }
}
