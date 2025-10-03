use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonColor {
  Green,
  Purple,
  Yellow,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
  #[default]
  Primary,
  Secondary,
  Danger,
  Colored(ButtonColor),
}

// TODO Document this
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
      ButtonVariant::Primary => "text-white bg-blue-500 hover:bg-blue-600 hover:dark:bg-blue-700  hover:dark:border-black   border border-blue-600 dark:border-blue-700",
      ButtonVariant::Secondary => "dark:text-gray-200 hover:dark:bg-gray-800 dark:bg-gray-700 text-gray-700 bg-gray-200 hover:bg-gray-300 border border-gray-400 hover:dark:border-gray-600",
      ButtonVariant::Danger => "text-white bg-red-500 hover:bg-red-600 hover:dark:bg-red-700  hover:dark:border-black   border border-red-600 dark:border-red-700",
      ButtonVariant::Colored(color) => match color {
        ButtonColor::Purple=>"text-white bg-purple-500 hover:bg-purple-600 hover:dark:border-black border border-purple-600",
        ButtonColor::Green => "text-white bg-green-500 hover:bg-green-600 hover:dark:border-black border border-green-600 ",
        ButtonColor::Yellow => "text-white bg-yellow-500 hover:bg-yellow-600 hover:dark:border-black border border-yellow-600 "
      },
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
