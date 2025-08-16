use crate::models::{
  exercise::{Exercise, ExerciseType},
  storage,
};
use crate::music::{heptatonic_scales::HeptaScaleType, notes::Note, scales::ScaleType};
use leptos::prelude::*;

#[component]
pub fn ExerciseManager() -> impl IntoView {
  let (exercises, set_exercises) = signal(storage::load_exercises());
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
        show_form
          .get()
          .then(|| {
            view! {
              <div class="p-4 mb-6 bg-gray-50 rounded-lg">
                <h2 class="mb-4 text-xl font-semibold">"New Exercise"</h2>
                <div class="space-y-4">
                  <div>
                    <label class="block mb-2 text-sm font-medium text-gray-700">
                      "Exercise Name"
                    </label>
                    <input
                      type="text"
                      class="py-2 px-3 w-full rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                      placeholder="e.g., C Major Scale Practice"
                      prop:value=name
                      on:input=move |ev| set_name.set(event_target_value(&ev))
                    />
                  </div>

                  <div>
                    <label class="block mb-2 text-sm font-medium text-gray-700">
                      "Exercise Type"
                    </label>
                    <select
                      class="py-2 px-3 w-full rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
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
                      class="py-2 px-4 font-bold text-white bg-green-500 rounded hover:bg-green-700 disabled:bg-gray-400"
                      on:click=add_exercise
                      disabled=move || name.get().trim().is_empty()
                    >
                      "Add Exercise"
                    </button>
                  </div>
                </div>
              </div>
            }
          })
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
