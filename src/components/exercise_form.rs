use crate::models::{
  exercise::{Exercise, ExerciseType},
  storage,
};
use crate::music::{heptatonic_scales::HeptaScaleType, notes::Note, scales::ScaleType};
use crate::components::svg_fretboard_scale_display::SvgFretboardScaleDisplay;
use leptos::prelude::*;
use web_sys::console;

#[derive(Clone)]
pub enum FormMode {
  Create,
  Edit(Exercise),
}

#[component]
pub fn ExerciseForm(
  mode: FormMode,
  #[prop(optional)] on_save: Option<Callback<Exercise>>,
  #[prop(optional)] on_cancel: Option<Callback<()>>,
) -> impl IntoView {
  // Initialize form fields based on mode
  let (original_exercise, form_title) = match &mode {
    FormMode::Create => (None, "Create Exercise"),
    FormMode::Edit(exercise) => (Some(exercise.clone()), "Edit Exercise"),
  };

  // Clone mode for use in closures
  let mode_for_validation = mode.clone();
  let mode_for_save = mode.clone();
  let mode_for_button = mode;

  // Form fields with initial values
  let (name, set_name) = signal(
    original_exercise
      .as_ref()
      .map(|ex| ex.name.clone())
      .unwrap_or_default(),
  );

  let (description, set_description) = signal(
    original_exercise
      .as_ref()
      .and_then(|ex| ex.description.clone())
      .unwrap_or_default(),
  );

  let (exercise_type_str, set_exercise_type_str) = signal(
    original_exercise
      .as_ref()
      .map(|ex| ex.exercise_type.type_name().to_string())
      .unwrap_or_else(|| "Technique".to_string()),
  );

  // Extract initial values for Scale/Triad types
  let (initial_root_note, initial_scale_type, initial_fret_range) = original_exercise
    .as_ref()
    .map(|ex| match &ex.exercise_type {
      ExerciseType::Scale {
        root_note,
        scale_type,
        fret_range,
      } => (Some(*root_note), Some(*scale_type), Some(*fret_range)),
      ExerciseType::Triad {
        root_note,
        scale_type,
        fret_range,
      } => (Some(*root_note), Some(*scale_type), Some(*fret_range)),
      _ => (None, None, None),
    })
    .unwrap_or((None, None, None));

  let (root_note, set_root_note) = signal(initial_root_note.unwrap_or(Note::C));
  let (scale_type, set_scale_type) =
    signal(initial_scale_type.unwrap_or(ScaleType::Hepatonic(HeptaScaleType::Major)));
  let (min_fret, set_min_fret) = signal(initial_fret_range.map(|(min, _)| min).unwrap_or(0));
  let (max_fret, set_max_fret) = signal(initial_fret_range.map(|(_, max)| max).unwrap_or(12));

  // Validation and error states
  let (errors, set_errors) = signal(Vec::<String>::new());
  let (show_type_change_warning, set_show_type_change_warning) = signal(false);
  let (pending_type_change, set_pending_type_change) = signal(None::<String>);

  // Check if exercise type has changed (for edit mode warning)
  let type_changed = {
    let mode_clone = mode_for_validation.clone();
    move || {
      if let FormMode::Edit(ref original) = mode_clone {
        exercise_type_str.get() != original.exercise_type.type_name()
      } else {
        false
      }
    }
  };

  // Validation function
  let validate_form = {
    move || -> Vec<String> {
      let mut validation_errors = Vec::new();

      let name_val = name.get().trim().to_string();
      if name_val.is_empty() {
        validation_errors.push("Name is required".to_string());
      } else {
        // Check name uniqueness
        let exclude_id = if let FormMode::Edit(ref ex) = mode_for_validation {
          Some(ex.id.as_str())
        } else {
          None
        };

        if storage::exercise_name_exists(&name_val, exclude_id) {
          validation_errors.push("An exercise with this name already exists".to_string());
        }
      }

      // Validate fret range for Scale/Triad types
      let exercise_type = exercise_type_str.get();
      if exercise_type == "Scale" || exercise_type == "Triad" {
        let min = min_fret.get();
        let max = max_fret.get();

        if min > max {
          validation_errors.push("Minimum fret cannot be greater than maximum fret".to_string());
        }
        if min > 24 || max > 24 {
          validation_errors.push("Fret numbers cannot exceed 24".to_string());
        }
      }

      validation_errors
    }
  };

  // Handle exercise type change
  let handle_type_change = move |new_type: String| {
    if type_changed() {
      set_pending_type_change.set(Some(new_type));
      set_show_type_change_warning.set(true);
    } else {
      set_exercise_type_str.set(new_type);
    }
  };

  // Confirm type change
  let confirm_type_change = move || {
    if let Some(new_type) = pending_type_change.get() {
      set_exercise_type_str.set(new_type);
      set_show_type_change_warning.set(false);
      set_pending_type_change.set(None);
    }
  };

  // Cancel type change
  let cancel_type_change = move || {
    set_show_type_change_warning.set(false);
    set_pending_type_change.set(None);
  };

  // Create exercise type from form data
  let create_exercise_type = move || -> ExerciseType {
    match exercise_type_str.get().as_str() {
      "Scale" => ExerciseType::Scale {
        root_note: root_note.get(),
        scale_type: scale_type.get(),
        fret_range: (min_fret.get(), max_fret.get()),
      },
      "Triad" => ExerciseType::Triad {
        root_note: root_note.get(),
        scale_type: scale_type.get(),
        fret_range: (min_fret.get(), max_fret.get()),
      },
      "Song" => ExerciseType::Song,
      _ => ExerciseType::Technique,
    }
  };

  // Save handler
  let handle_save = move |_| {
    let validation_errors = validate_form();
    if !validation_errors.is_empty() {
      set_errors.set(validation_errors);
      return;
    }

    set_errors.set(Vec::new());

    let exercise = match &mode_for_save {
      FormMode::Create => Exercise::new(name.get().trim().to_string(), create_exercise_type())
        .with_description(description.get().trim().to_string()),
      FormMode::Edit(original) => {
        let mut updated = original.clone();
        updated.name = name.get().trim().to_string();
        updated.exercise_type = create_exercise_type();
        updated.description = if description.get().trim().is_empty() {
          None
        } else {
          Some(description.get().trim().to_string())
        };
        updated
      }
    };

    console::log_1(&format!("Saving exercise: {:?}", exercise).into());

    // Save to storage
    match &mode_for_save {
      FormMode::Create => {
        storage::save_exercise(&exercise);
      }
      FormMode::Edit(_) => {
        if let Err(e) = storage::update_exercise(&exercise) {
          set_errors.set(vec![format!("Failed to update exercise: {}", e)]);
          return;
        }
      }
    }

    // Call the callback
    if let Some(callback) = on_save {
      callback.run(exercise);
    }
  };

  // Cancel handler
  let handle_cancel = move |_| {
    if let Some(callback) = on_cancel {
      callback.run(());
    }
  };

  view! {
      <div class="bg-white p-6 rounded-lg border border-gray-200">
          <h3 class="text-lg font-semibold text-gray-800 mb-4">{form_title}</h3>

          // Error display
          {move || {
              let errors = errors.get();
              if errors.is_empty() {
                  view! { <div></div> }.into_any()
              } else {
                  view! {
                      <div class="mb-4 p-3 bg-red-50 border border-red-200 rounded">
                          {errors.into_iter().map(|error| {
                              view! { <div class="text-red-700 text-sm">{error}</div> }
                          }).collect::<Vec<_>>()}
                      </div>
                  }.into_any()
              }
          }}

          <div class="space-y-4">
              // Name field
              <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Name</label>
                  <input
                      type="text"
                      class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                      prop:value={move || name.get()}
                      on:input=move |e| set_name.set(event_target_value(&e))
                      placeholder="Enter exercise name"
                  />
              </div>

              // Description field
              <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Description</label>
                  <textarea
                      class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                      prop:value={move || description.get()}
                      on:input=move |e| set_description.set(event_target_value(&e))
                      placeholder="Enter exercise description (optional)"
                      rows="3"
                  />
              </div>

              // Exercise type selector
              <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Exercise Type</label>
                  <select
                      class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                      prop:value={move || exercise_type_str.get()}
                      on:change=move |e| handle_type_change(event_target_value(&e))
                  >
                      <option value="Technique">Technique</option>
                      <option value="Scale">Scale</option>
                      <option value="Triad">Triad</option>
                      <option value="Song">Song</option>
                  </select>
              </div>

              // Conditional fields for Scale and Triad types
              {move || {
                  let ex_type = exercise_type_str.get();
                  if ex_type == "Scale" || ex_type == "Triad" {
                      view! {
                          <div class="space-y-4 p-4 bg-gray-50 rounded-md">
                              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                  // Root note selector
                                  <div>
                                      <label class="block text-sm font-medium text-gray-700 mb-1">Root Note</label>
                                      <select
                                          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                          prop:value={move || format!("{:?}", root_note.get())}
                                          on:change=move |e| {
                                              let value = event_target_value(&e);
                                              if let Ok(note) = value.parse::<Note>() {
                                                  set_root_note.set(note);
                                              }
                                          }
                                      >
                                          <option value="C">C</option>
                                          <option value="Cs">"C#"</option>
                                          <option value="D">D</option>
                                          <option value="Ds">"D#"</option>
                                          <option value="E">E</option>
                                          <option value="F">F</option>
                                          <option value="Fs">"F#"</option>
                                          <option value="G">G</option>
                                          <option value="Gs">"G#"</option>
                                          <option value="A">A</option>
                                          <option value="As">"A#"</option>
                                          <option value="B">B</option>
                                      </select>
                                  </div>

                                  // Scale type selector (simplified for now)
                                  <div>
                                      <label class="block text-sm font-medium text-gray-700 mb-1">Scale Type</label>
                                      <select
                                          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                          on:change=move |e| {
                                              let value = event_target_value(&e);
                                              match value.as_str() {
                                                  "Major" => set_scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Major)),
                                                  "Minor" => set_scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Minor)),
                                                  _ => set_scale_type.set(ScaleType::Hepatonic(HeptaScaleType::Major)),
                                              }
                                          }
                                      >
                                          <option value="Major">Major</option>
                                          <option value="Minor">Natural Minor</option>
                                      </select>
                                  </div>
                              </div>

                              // Fret range
                              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                  <div>
                                      <label class="block text-sm font-medium text-gray-700 mb-1">Min Fret</label>
                                      <input
                                          type="number"
                                          min="0"
                                          max="24"
                                          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                          prop:value={move || min_fret.get().to_string()}
                                          on:input=move |e| {
                                              if let Ok(val) = event_target_value(&e).parse::<u8>() {
                                                  set_min_fret.set(val.min(24));
                                              }
                                          }
                                      />
                                  </div>
                                  <div>
                                      <label class="block text-sm font-medium text-gray-700 mb-1">Max Fret</label>
                                      <input
                                          type="number"
                                          min="0"
                                          max="24"
                                          class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                                          prop:value={move || max_fret.get().to_string()}
                                          on:input=move |e| {
                                              if let Ok(val) = event_target_value(&e).parse::<u8>() {
                                                  set_max_fret.set(val.min(24));
                                              }
                                          }
                                      />
                                  </div>
                              </div>

                              // Position presets
                              <div>
                                  <label class="block text-sm font-medium text-gray-700 mb-2">Position Presets</label>
                                  <div class="flex gap-2 flex-wrap">
                                      <button
                                          type="button"
                                          class="px-3 py-1.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                          on:click=move |_| { set_min_fret.set(0); set_max_fret.set(4); }
                                      >
                                          "R"
                                      </button>
                                      <button
                                          type="button"
                                          class="px-3 py-1.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                          on:click=move |_| { set_min_fret.set(2); set_max_fret.set(6); }
                                      >
                                          "1"
                                      </button>
                                      <button
                                          type="button"
                                          class="px-3 py-1.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                          on:click=move |_| { set_min_fret.set(4); set_max_fret.set(8); }
                                      >
                                          "2"
                                      </button>
                                      <button
                                          type="button"
                                          class="px-3 py-1.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                          on:click=move |_| { set_min_fret.set(6); set_max_fret.set(10); }
                                      >
                                          "3"
                                      </button>
                                      <button
                                          type="button"
                                          class="px-3 py-1.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                          on:click=move |_| { set_min_fret.set(8); set_max_fret.set(12); }
                                      >
                                          "4"
                                      </button>
                                  </div>
                                  <div class="text-xs text-gray-500 mt-1">
                                      "R: Root (0-4) • 1: First (2-6) • 2: Second (4-8) • 3: Third (6-10) • 4: Fourth (8-12)"
                                  </div>
                              </div>

                              // Fretboard preview for scales
                              <div>
                                  <label class="block text-sm font-medium text-gray-700 mb-2">Preview</label>
                                  <div class="bg-gray-50 rounded-lg p-4 max-w-2xl mx-auto">
                                      <SvgFretboardScaleDisplay
                                          fret_range={Signal::derive(move || min_fret.get() as usize..=max_fret.get() as usize)}
                                          root_note={Signal::derive(move || root_note.get())}
                                          scale_type={Signal::derive(move || scale_type.get())}
                                      />
                                  </div>
                              </div>
                          </div>
                      }.into_any()
                  } else {
                      view! { <div></div> }.into_any()
                  }
              }}
          </div>

          // Action buttons
          <div class="flex justify-end space-x-3 mt-6">
              <button
                  class="px-4 py-2 text-gray-600 border border-gray-300 rounded-md hover:bg-gray-50"
                  on:click=handle_cancel
              >
                  Cancel
              </button>
              <button
                  class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600"
                  on:click=handle_save
              >
                  {match mode_for_button { FormMode::Create => "Create", FormMode::Edit(_) => "Update" }}
              </button>
          </div>

          // Type change confirmation dialog
          {move || {
              if show_type_change_warning.get() {
                  view! {
                      <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                          <div class="bg-white p-6 rounded-lg max-w-md mx-4">
                              <h3 class="text-lg font-semibold text-gray-800 mb-3">Confirm Exercise Type Change</h3>
                              <p class="text-gray-600 mb-4">
                                  Changing the exercise type will reset type-specific settings (root note, scale type, fret range).
                                  Are you sure you want to continue?
                              </p>
                              <div class="flex justify-end space-x-3">
                                  <button
                                      class="px-4 py-2 text-gray-600 border border-gray-300 rounded-md hover:bg-gray-50"
                                      on:click=move |_| cancel_type_change()
                                  >
                                      Cancel
                                  </button>
                                  <button
                                      class="px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600"
                                      on:click=move |_| confirm_type_change()
                                  >
                                      Continue
                                  </button>
                              </div>
                          </div>
                      </div>
                  }.into_any()
              } else {
                  view! { <div></div> }.into_any()
              }
          }}
      </div>
  }
}
