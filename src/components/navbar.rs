use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use leptos_use::{
  storage::{use_local_storage_with_options, use_session_storage, UseStorageOptions},
  use_color_mode, ColorMode, UseColorModeReturn,
};

use crate::components::ui::Toggle;

#[component]
pub fn NavbarLinks() -> impl IntoView {
  let location = use_location();
  let pathname = move || location.pathname.get();

  view! {
    <div class="flex flex-wrap flex-1 gap-y-1 gap-x-2 items-center min-w-0">
      <NavbarElement path="/" is_active=Signal::derive(move || pathname() == "/") text="Home" />
      <NavbarElement
        path="/exercises"
        is_active=Signal::derive(move || pathname() == "/exercises")
        text="Exercises"
      />
      <NavbarElement
        path="/fretboard_trainer"
        is_active=Signal::derive(move || pathname() == "/fretboard_trainer")
        text="Fretboard Trainer"
      />
      <NavbarElement
        path="/fretboard_dev"
        is_active=Signal::derive(move || pathname() == "/fretboard_dev")
        text="Fretboard Dev"
      />
      <NavbarElement
        path="/fretboard_scale"
        is_active=Signal::derive(move || pathname() == "/fretboard_scale")
        text="Scale Display"
      />
      <NavbarElement
        path="/fretboard_config_examples"
        text="Fretboard Config Examples"
        is_active=Signal::derive(move || pathname() == "/fretboard_config_examples")
      />
      <NavbarElement
        path="/about"
        text="About"
        is_active=Signal::derive(move || pathname() == "/about")
      />
    </div>
  }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
  let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

  view! {
    <Toggle
      is_checked=Signal::derive(move || mode.get() == ColorMode::Dark)
      on_pressed=Callback::new(move |_| {
        let new_mode = if mode.get() == ColorMode::Dark {
          ColorMode::Light
        } else {
          ColorMode::Dark
        };
        set_mode.set(new_mode);
      })
      checked_text="Dark"
      unchecked_text="Light"
    />
  }

  // view! {
  //   <label class="inline-flex items-center cursor-pointer">
  //     <input
  //       type="checkbox"
  //       class="sr-only peer"
  //       checked=move || is_dark.get()
  //       on:input=move |_| set_is_dark.set(!is_dark.get())
  //     />
  //
  //     <div class="relative w-11 h-6 bg-gray-200 rounded-full dark:bg-gray-700 dark:border-gray-600 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600 dark:peer-focus:ring-blue-800 dark:peer-checked:bg-blue-600"></div>
  //     <span class="text-sm font-medium text-gray-900 dark:text-gray-300 ms-3">
  //       {move || if is_dark.get() { "Dark" } else { "Light" }}
  //     </span>
  //   </label>
  // }
}

#[component]
pub fn Navbar() -> impl IntoView {
  view! {
    <div class="flex z-50 justify-between items-center p-4 grow h-fit">
      <NavbarLinks />
      <div class="pl-4 shrink-0">
        <ThemeToggle />
      </div>
    </div>
  }
}

#[component]
pub fn NavbarElement(
  #[prop()] path: &'static str,
  #[prop()] text: &'static str,
  #[prop(into)] is_active: Signal<bool>,
) -> impl IntoView {
  view! {
    <div
      class="flex justify-between p-2 text-center"
      class=(["dark:text-gray-800", "text-white", "bg-blue-500"], move || is_active.get())
      class=(["dark:hover:bg-gray-600", "hover:bg-gray-200"], move || !is_active.get())
    >

      <a href=path>
        <span>

          {text}
        </span>
      </a>

    </div>
  }
}
