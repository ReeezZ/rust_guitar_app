use crate::components::ui::{Button, ButtonVariant};
use leptos::prelude::*;

pub struct SongFormData {
  name: String,
  // TODO: cross check in exercises model song
  // current BPM
  // target BPM
  // links
  // description
}

#[component]
pub fn SongExerciseForm(
  #[prop(optional)] on_save: Option<Callback<SongFormData>>,
  #[prop(optional)] on_cancel: Option<Callback<()>>,
) -> impl IntoView {
  // Clone mode for use in closures

  // Form fields with initial values
  let name = RwSignal::new(String::new());

  let description = RwSignal::new(String::new());

  // Validation and error states
  let (errors, set_errors) = signal(Vec::<String>::new());

  // Maybe this is a candidate for ErrorBoundary: https://book.leptos.dev/view/07_errors.html
  // Validation function
  let validate_form = {
    move || -> Vec<String> {
      let mut validation_errors = Vec::new();

      let name_val = name.get().trim().to_string();
      if name_val.is_empty() {
        validation_errors.push("Name is required".to_string());
      } else {
        // TODO: could check if name already exists

        // TODO: atually implement persistence
        // let repo = get_exercise_repository();
        // if repo.name_exists(&name_val, exclude_id).unwrap_or(false) {
        //   validation_errors.push("An exercise with this name already exists".to_string());
        // }
      }

      validation_errors
    }
  };

  // Save handler
  let handle_save = move || {
    let validation_errors = validate_form();
    if !validation_errors.is_empty() {
      set_errors.set(validation_errors);
      return;
    }

    set_errors.set(Vec::new());
    let exercise = SongFormData {
      name: name.get().trim().to_string(),
    };
    // Save to storage
    // TODO: should move this to parent i guess, we hage the callback already
    // let repo = get_exercise_repository();

    // Call the callback
    if let Some(callback) = on_save {
      callback.run(exercise);
    }
  };

  // Cancel handler
  let handle_cancel = move || {
    if let Some(callback) = on_cancel {
      callback.run(());
    }
  };

  view! {
    <div class="p-6 rounded-lg border border-gray-200">
      <h3 class="mb-4 text-lg font-semibold">"New song for practice"</h3>

      // Error display
      {move || {
        let errors = errors.get();
        if errors.is_empty() {
          view! { <div></div> }.into_any()
        } else {
          view! {
            <div class="p-3 mb-4 bg-red-50 rounded border border-red-200 dark:bg-red-700 dark:border-red-800">
              {errors
                .into_iter()
                .map(|error| {
                  view! { <div class="text-sm text-red-700 dark:text-red-200">{error}</div> }
                })
                .collect::<Vec<_>>()}
            </div>
          }
            .into_any()
        }
      }}

      <div class="space-y-4">
        // Basic form fields - inline (simple enough to not need separate component)
        <div>
          <label class="block mb-1 text-sm font-medium">Name</label>
          <input
            type="text"
            class="py-2 px-3 w-full rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            prop:value=move || name.get()
            on:input=move |e| name.set(event_target_value(&e))
            placeholder="Enter song name"
          />
        </div>

        <div>
          <label class="block mb-1 text-sm font-medium">Description</label>
          <textarea
            class="py-2 px-3 w-full rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
            prop:value=move || description.get()
            on:input=move |e| description.set(event_target_value(&e))
            placeholder="Enter song description (optional)"
            rows="3"
          />
        </div>

      // Type-specific fields - kept as separate component (complex conditional logic)
      </div>

      // Action buttons - inline (just 2 buttons, simpler than separate component)
      <div class="flex justify-end mt-6 space-x-3">
        <Button variant=ButtonVariant::Secondary on_click=handle_cancel>
          "Cancel"
        </Button>
        <Button variant=ButtonVariant::Primary on_click=handle_save>
          "Create"
        </Button>

      </div>
    </div>
  }
}
