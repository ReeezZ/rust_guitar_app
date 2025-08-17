use leptos::{prelude::ElementChild, prelude::*};
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::{
  components::navbar::Navbar,
  pages::{
    exercise_detail::ExerciseDetail, exercises::ExercisesPage, fretboard_trainer::FretboardTrainer,
    home::Home, not_found::NotFound, fretboard::SvgFretboardPage,
    fretboard_scale::FretboardScalePage,
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
          <Route path=path!("/exercises") view=ExercisesPage />
          <Route path=path!("/exercises/:id") view=ExerciseDetail />
          <Route path=path!("/fretboard_trainer") view=FretboardTrainer />
          <Route path=path!("/svg_fretboard") view=SvgFretboardPage />
          <Route path=path!("/svg_fretboard_scale") view=FretboardScalePage />
        </Routes>
      </main>
    </Router>
  }
}
