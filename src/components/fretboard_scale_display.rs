use leptos::{component, logging::log, prelude::*};

use crate::{
  components::{fretboard_model::FretboardModel, fretboard::Fretboard},
  music::{
    heptatonic_scales::{HeptaScaleImpl, HeptaScaleType},
    notes::Note,
    scales::{Scale, ScaleType},
  },
};

#[component]
pub fn FretboardScaleDisplay(
  #[prop()] root_note: ReadSignal<Note>,
  #[prop()] scale_type: ReadSignal<ScaleType>,
  #[prop()] num_frets: u8,
) -> impl IntoView {
  let fretboard_model = RwSignal::new(FretboardModel::six_string_standard_tuning(num_frets));

  // Create an effect to update the fretboard whenever signals change
  Effect::new(move |_| {
    let current_root = root_note.get();
    let current_scale = scale_type.get();
    let current_scale = match current_scale {
      ScaleType::Hepatonic(scale) => scale,
      _ => HeptaScaleType::Major,
    };
    let scale = Scale::Heptatonic(HeptaScaleImpl::new(current_root, current_scale));
    log!("Updating fretboard with scale: {:?}", &scale);

    // Update the model by creating a new one
    // This assumes FretboardModel implements Clone
    fretboard_model.update(|model| {
      model.update_from_scale(&scale);
    });
  });

  view! { <Fretboard fretboard=fretboard_model /> }
}
