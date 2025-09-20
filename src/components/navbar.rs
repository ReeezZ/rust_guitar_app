use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn NavbarLinks() -> impl IntoView {
  let location = use_location();
  let pathname = move || location.pathname.get();

  view! {
    <ul class="flex items-center">
      <li>
        <NavbarElement path="/" is_active=Signal::derive(move || pathname() == "/") text="Home" />
      </li>
      <li>
        <NavbarElement
          path="/exercises"
          is_active=Signal::derive(move || pathname() == "/exercises")
          text="Exercises"
        />
      </li>
      <li>
        <NavbarElement
          path="/fretboard_trainer"
          is_active=Signal::derive(move || pathname() == "/fretboard_trainer")
          text="Fretboard Trainer"
        />
      </li>
      <li>
        <NavbarElement
          path="/fretboard_dev"
          is_active=Signal::derive(move || pathname() == "/fretboard_dev")
          text="Fretboard Dev"
        />
      </li>
      <li>
        <NavbarElement
          path="/fretboard_scale"
          is_active=Signal::derive(move || pathname() == "/fretboard_scale")
          text="Scale Display"
        />
      </li>
      <li>
        <NavbarElement
          path="/fretboard_config_examples"
          text="Fretboard Config Examples"
          is_active=Signal::derive(move || pathname() == "/fretboard_config_examples")
        />
      </li>
      <li>
        <NavbarElement
          path="/about"
          text="About"
          is_active=Signal::derive(move || pathname() == "/about")
        />
      </li>
    </ul>
  }
}

#[component]
pub fn Navbar() -> impl IntoView {
  view! {
    <div class="flex z-50 p-4 grow h-fit">
      <NavbarLinks />
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
    <a
      href=path
      class="p-3 m-1"
      class=(["text-white", "bg-blue-500"], move || is_active.get())
      class=(["hover:bg-gray-200"], move || !is_active.get())
    >
      <span>{text}</span>
    </a>
  }
}
