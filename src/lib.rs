use leptos::{prelude::ElementChild, prelude::*};
use leptos_meta::*;
use leptos_router::{components::*, path};

mod components;
mod pages;

use crate::components::navbar::Navbar;
use crate::pages::not_found::NotFound;
use crate::pages::{guitar_v1::GuitarV1, home::Home};

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  view! {
    <Html attr:lang="en" attr:dir="ltr" />
    <Title text="♫ Rust Guitar App ♫" />

    <Meta charset="UTF-8" />
    <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

    <Router>
      <Navbar />
      <main>
        <Routes fallback=|| view! { <NotFound /> }>
          <Route path=path!("/") view=Home />
          <Route path=path!("/guitar_v1") view=GuitarV1 />
        </Routes>
      </main>
    </Router>
  }
}
