use leptos::prelude::*;
use shared::{music::heptatonic_scales::HeptaScaleType, Note, Scale, ScaleTrait, ScaleType};

use crate::{
  fretboard::{components::base::FretboardViewModel, fretboard_model::FretboardModel},
  pages::fretboard_dev::shared_model_demo::SharedModelDemo,
};

#[component]
pub fn FretboardDevPage() -> impl IntoView {
  let model = RwSignal::new(FretboardModel::default());
  model.update(|model| {
    model.update_from_scale(Scale::new(
      Note::C,
      ScaleType::Hepatonic(HeptaScaleType::Major),
    ));
  });
  view! {
    <>
      <FretboardViewModel model=model />
      <SharedModelDemo />
    </>
  }
}
