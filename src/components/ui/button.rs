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
      ButtonVariant::Primary => "text-white bg-blue-500 hover:bg-blue-600",
      ButtonVariant::Secondary => "dark:text-gray-200 hover:dark:bg-gray-800 dark:bg-gray-700 text-gray-700 bg-gray-200 hover:bg-gray-300",
      ButtonVariant::Danger => "text-white bg-red-500 hover:bg-red-600",
      ButtonVariant::Special => "text-white bg-purple-500 hover:bg-purple-600",
      }
  };

  view! {
    <button
      class=move || format!("{} justify-center py-2 px-4 rounded", class())
      title=title
      on:click=move |_| {
        on_click.run(());
      }
    >
      {children()}

    </button>
  }
}
