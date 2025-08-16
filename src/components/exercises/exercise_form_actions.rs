use leptos::prelude::*;
use super::FormMode;

#[component]
pub fn ExerciseFormActions(
    mode: FormMode,
    on_save: Callback<()>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let button_text = match mode {
        FormMode::Create => "Create",
        FormMode::Edit(_) => "Update",
    };

    view! {
        <div class="flex justify-end space-x-3 mt-6">
            <button
                class="px-4 py-2 text-gray-600 border border-gray-300 rounded-md hover:bg-gray-50"
                on:click=move |_| on_cancel.run(())
            >
                Cancel
            </button>
            <button
                class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600"
                on:click=move |_| on_save.run(())
            >
                {button_text}
            </button>
        </div>
    }
}
