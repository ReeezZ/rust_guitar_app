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
          href="/guitar_v1"
          class=move || { if pathname() == "/guitar_v1" { "nav-link active" } else { "nav-link" } }
        >
          <span>Guitar V1</span>
        </a>
      </li>
      <li>
        <a
          href="/fretboard_viewer"
          class=move || {
            if pathname() == "/fretboard_viewer" { "nav-link active" } else { "nav-link" }
          }
        >
          <span>Fretboard viewer</span>
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
