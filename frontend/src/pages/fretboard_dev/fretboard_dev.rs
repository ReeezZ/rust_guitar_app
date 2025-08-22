use leptos::prelude::*;

use crate::{
  fretboard::{
    components::with_notes::FretboardWithNotesViewModel, with_notes_model::FretboardWithNotesModel,
  },
  pages::fretboard_dev::shared_model_demo::SharedModelDemo,
};

#[component]
pub fn FretboardDevPage() -> impl IntoView {
  let model = RwSignal::new(FretboardWithNotesModel::default());
  view! {
    <>
      <FretboardWithNotesViewModel model=model />
      <SharedModelDemo />
    </>
  }
}
