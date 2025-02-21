use leptos::{prelude::ElementChild, prelude::*};
use leptos_meta::*;
use leptos_router::{components::*, path};

mod components;
mod pages;

use crate::components::fretboard::Fretboard;
use crate::components::navbar::Navbar;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

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
          <Route
            path=path!("/guitar_v1")
            view= || view! {
              <div>
                <h1 class="py-12 text-6xl font-bold text-center text-primary-rev trans">
                  "Gitarren Griffbrett"
                </h1>
                <Fretboard num_frets=24 num_strings=6 />
              </div>
            }
          />
        </Routes>
      </main>
    </Router>
  }
}
