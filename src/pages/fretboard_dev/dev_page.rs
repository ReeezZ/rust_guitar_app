use crate::music::{heptatonic_scales::HeptaScaleType, Note, Scale, ScaleType};
use leptos::prelude::*;

use crate::{
  components::fretboard::FretboardModelAdapter,
  models::fretboard::{FretboardModel, FretboardModelExt},
  pages::fretboard_dev::shared_model_demo::SharedModelDemo,
};

#[component]
pub fn FretboardDevPage() -> impl IntoView {
  let model = RwSignal::new(FretboardModel::default());

  Effect::new(move || {
    model.with(|model| {
      model.update_from_scale(Scale::new(
        Note::C,
        ScaleType::Hepatonic(HeptaScaleType::Major),
      ));
    });
  });
  view! {
    <>
      <FretboardModelAdapter model=model />
      <SharedModelDemo />
    </>
  }
}
