use leptos::{prelude::ElementChild, prelude::*};
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::{
  components::navbar::Navbar,
  pages::{
    fretboard_playground::FretboardPlayground, fretboard_trainer::FretboardTrainer, fretboard_viewer::FretboardViewer, guitar_v1::GuitarV1, home::Home, not_found::NotFound
  },
};
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
          <Route path=path!("/fretboard_viewer") view=FretboardViewer />
          <Route path=path!("/fretboard_playground") view=FretboardPlayground />
          <Route path=path!("/fretboard_trainer") view=FretboardTrainer />
        </Routes>
      </main>
    </Router>
  }
}
