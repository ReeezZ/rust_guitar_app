use leptos::prelude::*;

use crate::components::ui::{Button, ButtonVariant};

#[component]
pub fn ExerciseTypeChangeConfirmation(
  show: ReadSignal<bool>,
  on_confirm: Callback<()>,
  on_cancel: Callback<()>,
) -> impl IntoView {
  view! {
    {move || {
      if show.get() {
        view! {
          <div class="flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-50">
            <div class="p-6 mx-4 max-w-md bg-white rounded-lg">
              <h3 class="mb-3 text-lg font-semibold text-gray-800">Confirm Exercise Type Change</h3>
              <p class="mb-4 text-gray-600">
                Changing the exercise type will reset type-specific settings (root note, scale type, fret range).
                Are you sure you want to continue?
              </p>
              <div class="flex justify-end space-x-3">
                <Button variant=ButtonVariant::Primary on_click=move || on_cancel.run(())>
                  Cancel
                </Button>
                <Button variant=ButtonVariant::Danger on_click=move || on_confirm.run(())>
                  Continue
                </Button>
              </div>
            </div>
          </div>
        }
          .into_any()
      } else {
        view! { <div></div> }.into_any()
      }
    }}
  }
}
