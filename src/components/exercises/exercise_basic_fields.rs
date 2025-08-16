use leptos::prelude::*;

#[component]
pub fn ExerciseBasicFields(
    name: ReadSignal<String>,
    on_name_change: Callback<String>,
    description: ReadSignal<String>, 
    on_description_change: Callback<String>,
    exercise_type: ReadSignal<String>,
    on_type_change: Callback<String>,
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            // Name field
            <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Name</label>
                <input
                    type="text"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    prop:value={move || name.get()}
                    on:input=move |e| on_name_change.run(event_target_value(&e))
                    placeholder="Enter exercise name"
                />
            </div>

            // Description field
            <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Description</label>
                <textarea
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    prop:value={move || description.get()}
                    on:input=move |e| on_description_change.run(event_target_value(&e))
                    placeholder="Enter exercise description (optional)"
                    rows="3"
                />
            </div>

            // Exercise type selector
            <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Exercise Type</label>
                <select
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    prop:value={move || exercise_type.get()}
                    on:change=move |e| on_type_change.run(event_target_value(&e))
                >
                    <option value="Technique">Technique</option>
                    <option value="Scale">Scale</option>
                    <option value="Triad">Triad</option>
                    <option value="Song">Song</option>
                </select>
            </div>
        </div>
    }
}
