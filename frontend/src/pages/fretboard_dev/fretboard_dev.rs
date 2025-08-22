use leptos::prelude::*;

use crate::{
  fretboard::{components::base::FretboardViewModel, fretboard_model::FretboardModel},
  pages::fretboard_dev::shared_model_demo::SharedModelDemo,
};

#[component]
pub fn FretboardDevPage() -> impl IntoView {
  let model = RwSignal::new(FretboardModel::default());
  view! {
    <>
      <FretboardViewModel model=model />
      <SharedModelDemo />
    </>
  }
}
