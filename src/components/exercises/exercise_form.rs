use crate::components::exercises::{
  ExerciseBasicFields,
  ExerciseFormActions,
  ExerciseTypeChangeConfirmation,
  ExerciseTypeSpecificFields,
};
use crate::models::{
  exercise::{Exercise, ExerciseType},
  repository::{get_exercise_repository, ExerciseRepository},
};
use crate::music::{heptatonic_scales::HeptaScaleType, notes::Note, scales::ScaleType};
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

        let repo = get_exercise_repository();
        if repo.name_exists(&name_val, exclude_id).unwrap_or(false) {
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

    console::log_1(&format!("Saving exercise: {exercise:?}").into());

    // Save to storage
    let repo = get_exercise_repository();
    match &mode_for_save {
      FormMode::Create => {
        if let Err(e) = repo.save(&exercise) {
          set_errors.set(vec![format!("Failed to save exercise: {e}")]);
          return;
        }
      }
      FormMode::Edit(_) => {
        if let Err(e) = repo.update(&exercise) {
          set_errors.set(vec![format!("Failed to update exercise: {e}")]);
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
              // Basic form fields - extracted to component
              <ExerciseBasicFields
                  name={name.into()}
                  on_name_change={Callback::new(move |value| set_name.set(value))}
                  description={description.into()}
                  on_description_change={Callback::new(move |value| set_description.set(value))}
                  exercise_type={exercise_type_str.into()}
                  on_type_change={Callback::new(move |new_type| handle_type_change(new_type))}
              />

              // Type-specific fields - extracted to component  
              <ExerciseTypeSpecificFields
                  exercise_type={exercise_type_str.into()}
                  root_note={root_note.into()}
                  on_root_note_change={Callback::new(move |note| set_root_note.set(note))}
                  scale_type={scale_type.into()}
                  on_scale_type_change={Callback::new(move |scale| set_scale_type.set(scale))}
                  min_fret={min_fret.into()}
                  on_min_fret_change={Callback::new(move |fret| set_min_fret.set(fret))}
                  max_fret={max_fret.into()}
                  on_max_fret_change={Callback::new(move |fret| set_max_fret.set(fret))}
              />
          </div>

          // Action buttons - extracted to component
          <ExerciseFormActions
              mode={mode_for_button}
              on_save={Callback::new(move |_| handle_save(()))}
              on_cancel={Callback::new(move |_| handle_cancel(()))}
          />

          // Type change confirmation dialog - extracted to component
          <ExerciseTypeChangeConfirmation
              show={show_type_change_warning.into()}
              on_confirm={Callback::new(move |_| confirm_type_change())}
              on_cancel={Callback::new(move |_| cancel_type_change())}
          />
      </div>
  }
}
