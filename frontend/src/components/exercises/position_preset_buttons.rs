use leptos::prelude::*;

#[component]
pub fn PositionPresetButtons(#[prop(into)] on_preset_select: Callback<(u8, u8)>) -> impl IntoView {
  view! {
    <div class="p-1 mt-2 text-center bg-gray-50 rounded-md border border-gray-300">
      <label class="justify-center text-sm font-medium text-center text-gray-700">
        Position Presets
      </label>
      <div class="flex flex-shrink justify-center items-center">
        <button
          type="button"
          class="mr-1 text-sm font-medium text-gray-700 bg-white rounded-md border border-gray-300 hover:bg-gray-50 focus:ring-2 focus:ring-blue-500 focus:outline-none"
          on:click=move |_| on_preset_select.run((0, 4))
        >
          "R"
        </button>
        <button
          type="button"
          class="mr-1 text-sm font-medium text-gray-700 bg-white rounded-md border border-gray-300 hover:bg-gray-50 focus:ring-2 focus:ring-blue-500 focus:outline-none"
          on:click=move |_| on_preset_select.run((2, 6))
        >
          "1"
        </button>
        <button
          type="button"
          class="mr-1 font-medium text-gray-700 bg-white rounded-md border border-gray-300 hover:bg-gray-50 focus:ring-2 focus:ring-blue-500 focus:outline-none mr-1text-sm"
          on:click=move |_| on_preset_select.run((4, 8))
        >
          "2"
        </button>
        <button
          type="button"
          class="mr-1 font-medium text-gray-700 bg-white rounded-md border border-gray-300 hover:bg-gray-50 focus:ring-2 focus:ring-blue-500 focus:outline-none mr-1text-sm"
          on:click=move |_| on_preset_select.run((6, 10))
        >
          "3"
        </button>
        <button
          type="button"
          class="text-sm font-medium text-gray-700 bg-white rounded-md border border-gray-300 hover:bg-gray-50 focus:ring-2 focus:ring-blue-500 focus:outline-none"
          on:click=move |_| on_preset_select.run((8, 12))
        >
          "4"
        </button>
      </div>
    </div>
  }
}
