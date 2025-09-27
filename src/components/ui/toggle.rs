use leptos::prelude::*;

#[component]
pub fn Toggle(
  #[prop(into)] is_checked: Signal<bool>,
  on_pressed: Callback<()>,
  checked_text: &'static str,
  unchecked_text: &'static str,
) -> impl IntoView {
  view! {
    <label class="inline-flex items-center cursor-pointer">
      <input
        type="checkbox"
        class="sr-only peer"
        checked=move || is_checked.get()
        on:input=move |_| on_pressed.run(())
      />

      <div class="relative w-11 h-6 bg-gray-200 rounded-full dark:bg-gray-700 dark:border-gray-600 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600 dark:peer-focus:ring-blue-800 dark:peer-checked:bg-blue-600"></div>
      <span class="text-sm font-medium text-gray-900 dark:text-gray-300 ms-3">
        {move || if is_checked.get() { checked_text } else { unchecked_text }}
      </span>
    </label>
  }
}
