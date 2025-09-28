use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
  #[default]
  Primary,
  Secondary,
  Danger,
  Special,
}

#[component]
pub fn Button(
  #[prop(into)] variant: Signal<ButtonVariant>,
  #[prop(into)] on_click: Callback<()>,
  #[prop(optional)] title: String,
  children: Children,
) -> impl IntoView {
  let class = move || {
    match variant.get() {
    ButtonVariant::Primary => "py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600",
    ButtonVariant::Secondary => {
      "py-2 px-4 dark:text-gray-200 hover:dark:bg-gray-800 dark:bg-gray-700 text-gray-700 bg-gray-200 rounded hover:bg-gray-300"
    }
    ButtonVariant::Danger => "py-2 px-4 text-white bg-red-500 rounded hover:bg-red-600",
    ButtonVariant::Special => "py-2 px-4 text-white bg-purple-500 rounded hover:bg-purple-600",
  }
  };

  view! {
    <button
      class=class
      title=title
      on:click=move |_| {
        on_click.run(());
      }
    >
      {children()}
    </button>
  }
}
