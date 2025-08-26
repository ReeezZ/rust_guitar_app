use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use leptos_use::storage::use_local_storage;

#[component]
pub fn ThemeToggle() -> impl IntoView {
  let (is_dark, set_is_dark, _remove_is_dark) =
    use_local_storage::<bool, FromToStringCodec>("is_dark");

  view! {
    <label class="switch" style="float: right; margin-right: 1rem">
      <input
        name="color-scheme"
        checked=move || is_dark.get()
        on:input=move |_| set_is_dark.set(!is_dark.get())
        type="checkbox"
      />
      <div class="slider round"></div>
    </label>
  }
}

#[component]
pub fn NavbarLinks() -> impl IntoView {
  let location = use_location();
  let pathname = move || location.pathname.get();

  view! {
    <ul>
      <li>
        <a href="/" class=move || if pathname() == "/" { "nav-link active" } else { "nav-link" }>
          Home
        </a>
      </li>
      <li>
        <a
          href="/exercises"
          class=move || { if pathname() == "/exercises" { "nav-link active" } else { "nav-link" } }
        >
          <span>Exercises</span>
        </a>
      </li>
      <li>
        <a
          href="/fretboard_trainer"
          class=move || {
            if pathname() == "/fretboard_trainer" { "nav-link active" } else { "nav-link" }
          }
        >
          <span>Fretboard Trainer</span>
        </a>
      </li>
      <li>
        <a
          href="/fretboard_dev"
          class=move || {
            if pathname() == "/fretboard_dev" { "nav-link active" } else { "nav-link" }
          }
        >
          <span>Fretboard Dev</span>
        </a>
      </li>
      <li>
        <a
          href="/fretboard_scale"
          class=move || {
            if pathname() == "/fretboard_scale" { "nav-link active" } else { "nav-link" }
          }
        >
          <span>Scale Display</span>
        </a>
      </li>
      <li>
        <a
          href="/fretboard_config_examples"
          class=move || {
            if pathname() == "/fretboard_config_examples" { "nav-link active" } else { "nav-link" }
          }
        >
          <span>Fretboard Config Examples</span>
        </a>
      </li>
      <li>
        <a
          href="/about"
          class=move || { if pathname() == "/about" { "nav-link active" } else { "nav-link" } }
        >
          <span>About</span>
        </a>
      </li>
    </ul>
  }
}

#[component]
pub fn Navbar() -> impl IntoView {
  view! {
    <div class="navbar">
      <NavbarLinks />
      <ThemeToggle />
    </div>
  }
}
