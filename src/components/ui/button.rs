use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
  #[default]
  Primary,
  Secondary,
  Danger,
  Special,
}

// TODO: Document this
// TODO: Test this
#[component]
pub fn Button(
  #[prop(into)] variant: Signal<ButtonVariant>,
  #[prop(optional, into)] on_click: Option<Callback<()>>,
  #[prop(optional)] title: String,
  #[prop(optional, into)] disabled: Signal<bool>,
  children: Children,
) -> impl IntoView {
  let class = move || {
    if disabled.get() {
      return "opacity-50 cursor-not-allowed bg-gray-400 text-white";
    }
    match variant.get() {
      ButtonVariant::Primary => "text-white bg-blue-500 hover:bg-blue-600",
      ButtonVariant::Secondary => "dark:text-gray-200 hover:dark:bg-gray-800 dark:bg-gray-700 text-gray-700 bg-gray-200 hover:bg-gray-300",
      ButtonVariant::Danger => "text-white bg-red-500 hover:bg-red-600 hover:dark:bg-red-700",
      ButtonVariant::Special => "text-white bg-purple-500 hover:bg-purple-600",
      }
  };

  view! {
    <button
      disabled=disabled
      class=move || format!("{} justify-center py-2 px-4 m-2 rounded", class())
      title=title
      on:click=move |_| {
        if let Some(on_click) = on_click {
          if disabled.get() {
            return;
          }
          on_click.run(());
        }
      }
    >
      {children()}

    </button>
  }
}
