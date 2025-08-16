use leptos::prelude::*;

#[component]
pub fn PositionPresetButtons(
    on_preset_select: Callback<(u8, u8)>,
) -> impl IntoView {
    view! {
        <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Position Presets</label>
            <div class="flex gap-2 flex-wrap">
                <button
                    type="button"
                    class="px-3 py-1.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    on:click=move |_| on_preset_select.run((0, 4))
                >
                    "R"
                </button>
                <button
                    type="button"
                    class="px-3 py-1.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    on:click=move |_| on_preset_select.run((2, 6))
                >
                    "1"
                </button>
                <button
                    type="button"
                    class="px-3 py-1.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    on:click=move |_| on_preset_select.run((4, 8))
                >
                    "2"
                </button>
                <button
                    type="button"
                    class="px-3 py-1.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    on:click=move |_| on_preset_select.run((6, 10))
                >
                    "3"
                </button>
                <button
                    type="button"
                    class="px-3 py-1.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    on:click=move |_| on_preset_select.run((8, 12))
                >
                    "4"
                </button>
            </div>
            <div class="text-xs text-gray-500 mt-1">
                "R: Root (0-4) • 1: First (2-6) • 2: Second (4-8) • 3: Third (6-10) • 4: Fourth (8-12)"
            </div>
        </div>
    }
}
