use leptos::prelude::*;

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
                                  on:click=move |_| on_cancel.run(())
                              >
                                  Cancel
                              </button>
                              <button
                                  class="px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600"
                                  on:click=move |_| on_confirm.run(())
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
  }
}
