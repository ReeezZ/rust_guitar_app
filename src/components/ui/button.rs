use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
  #[default]
  Primary,
  Secondary,
  Danger,
}

#[component]
pub fn Button(
  #[prop(into)] variant: Signal<ButtonVariant>,
  #[prop(into)] on_click: Callback<()>,
  children: Children,
) -> impl IntoView {
  let class = move || match variant.get() {
    ButtonVariant::Primary => "py-2 px-4 text-white bg-blue-500 rounded hover:bg-blue-600",
    ButtonVariant::Secondary => "py-2 px-4 text-gray-700 bg-gray-200 rounded hover:bg-gray-300",
    ButtonVariant::Danger => "py-2 px-4 text-white bg-red-500 rounded hover:bg-red-600",
  };

  view! {
    <button
      class=class
      on:click=move |_| {
        on_click.run(());
      }
    >
      {children()}
    </button>
  }
}
