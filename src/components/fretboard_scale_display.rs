use leptos::{component, logging::log, prelude::*};

use crate::{
  components::{
    fretboard::{FretClickEvent, Fretboard},
    fretboard_model::FretboardModel,
  },
  music::{
    notes::Note,
    scales::{Scale, ScaleTrait, ScaleType},
  },
};

#[component]
pub fn FretboardScaleDisplay(
  #[prop()] root_note: ReadSignal<Note>,
  #[prop()] scale_type: ReadSignal<ScaleType>,
  #[prop()] num_frets: ReadSignal<u8>,
) -> impl IntoView {
  let fretboard_model = RwSignal::new(FretboardModel::six_string_standard_tuning(num_frets.get()));

  Effect::new(move |_| {
    let num_frets = num_frets.get();
    fretboard_model.get().update_num_frets(num_frets);
    fretboard_model
      .get()
      .update_from_scale(&Scale::new(root_note.get(), scale_type.get()));
  });

  // Create an effect to update the fretboard whenever signals change
  Effect::new(move |_| {
    let scale = Scale::new(root_note.get(), scale_type.get());
    log!("Updating fretboard with scale: {:?}", &scale);

    // Update the model by creating a new one
    // This assumes FretboardModel implements Clone
    fretboard_model.update(|model| {
      model.update_from_scale(&scale);
    });
  });

  let on_fret_clicked = Callback::new(move |fret_click_event: FretClickEvent| {
    log!("Fret clicked: {:?}", fret_click_event);
  });

  view! { <Fretboard fretboard=fretboard_model on_fret_clicked=on_fret_clicked /> }
}
