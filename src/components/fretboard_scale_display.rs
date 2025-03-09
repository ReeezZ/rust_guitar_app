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
  let fretboard_model = RwSignal::new(FretboardModel::six_string_standard_tuning(
    num_frets.get_untracked(),
  ));

  Effect::new(move |_| {
    let current_root = root_note.get();
    let current_scale_type = scale_type.get();
    let scale = Scale::new(current_root, current_scale_type);
    let num_frets = num_frets.get();

    log!(
      "FretboardScaleDisplay: Updating fretboard with scale: {:?} and num_frets {:?}",
      scale,
      num_frets
    );

    fretboard_model.update(|model| {
      model.update_num_frets(num_frets);
      model.update_from_scale(&scale);
    });
  });

  let on_fret_clicked = Callback::new(move |fret_click_event: FretClickEvent| {
    log!("Fret clicked: {:?}", fret_click_event);
  });

  view! { <Fretboard fretboard=fretboard_model on_fret_clicked=on_fret_clicked /> }
}
