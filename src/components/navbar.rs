use leptos::prelude::*;
use leptos_router::hooks::use_location;
use leptos_use::{use_color_mode, ColorMode, UseColorModeReturn};

use crate::components::ui::Toggle;

#[component]
pub fn NavbarLinks() -> impl IntoView {
  let location = use_location();
  let pathname = move || location.pathname.get();

  view! {
    <div class="flex flex-wrap flex-1 items-center min-w-0">
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
        path="/dev"
        text="Dev"
        is_active=Signal::derive(move || pathname() == "/dev")
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
        let new_mode = if mode.get() != ColorMode::Dark {
          ColorMode::Dark
        } else {
          ColorMode::Light
        };
        set_mode.set(new_mode);
      })
      checked_text="Dark"
      unchecked_text="Light"
    />
  }
}

#[component]
pub fn Navbar() -> impl IntoView {
  view! {
    <div class="flex z-50 justify-between items-center px-4 mb-2 grow h-fit">
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
        <span>{text}</span>
      </a>

    </div>
  }
}
