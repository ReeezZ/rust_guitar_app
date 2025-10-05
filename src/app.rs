use leptos::{prelude::ElementChild, prelude::*};
use leptos_meta::*;
use leptos_router::{components::*, path};
use leptos_use::{use_color_mode_with_options, ColorMode, UseColorModeOptions, UseColorModeReturn};

use crate::{
  components::navbar::Navbar,
  pages::{
    dev_page::DevPage, exercise_detail::ExerciseDetailPage, exercises::ExercisesPage,
    fretboard_config_examples::FretboardConfigExamples, fretboard_dev::FretboardDevPage,
    fretboard_scale::FretboardScalePage, fretboard_trainer::FretboardTrainerPage, home::Home,
    not_found::NotFound, scale_exercise::ScaleExercisePage,
  },
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <Link rel="icon" href="/favicon.ico" />
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <AutoReload options=options.clone() />
        <HydrationScripts options />
        <link rel="stylesheet" id="leptos" href="/pkg/rust_guitar_app.css" />
        <MetaTags />
      </head>
      <body>
        <App />
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  let UseColorModeReturn { mode, .. } =
    use_color_mode_with_options(UseColorModeOptions::default().cookie_enabled(true));

  view! {
    <>
      <Html attr:class=move || { if mode.get() == ColorMode::Dark { "dark" } else { "" } } />

      <div class="min-h-screen bg-gray-100 dark:text-gray-200 dark:bg-gray-800">
        <Router>
          <Title text="♫ Rust Guitar App ♫" />
          <Navbar />
          <main>
            <Routes fallback=|| view! { <NotFound /> }>
              <Route path=path!("/") view=Home />
              <Route path=path!("/dev") view=DevPage />
              <Route path=path!("/exercises") view=ExercisesPage />
              <Route path=path!("/exercises/:id") view=ExerciseDetailPage />
              <Route path=path!("/exercise/scale") view=ScaleExercisePage />
              <Route path=path!("/fretboard_trainer") view=FretboardTrainerPage />
              <Route path=path!("/fretboard_dev") view=FretboardDevPage />
              <Route path=path!("/fretboard_scale") view=FretboardScalePage />
              <Route path=path!("/fretboard_config_examples") view=FretboardConfigExamples />
            </Routes>
          </main>
        </Router>
      </div>
    </>
  }
}
