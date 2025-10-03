use std::str::FromStr;

use leptos::prelude::*;
use leptos_router::{
  hooks::use_query,
  params::{IntoParam, Params},
};

use crate::{
  components::exercises::practice_session::ScalePracticeSession,
  models::exercise::ScaleExercise,
  music::{heptatonic_scales::HeptaScaleType, Note, ScaleType},
};

impl IntoParam for Note {
  fn into_param(
    value: Option<&str>,
    name: &str,
  ) -> Result<Self, leptos_router::params::ParamsError> {
    if value.is_none() {
      return Err(leptos_router::params::ParamsError::MissingParam(
        name.to_string(),
      ));
    }

    Ok(Note::from_str(value.unwrap()).unwrap_or(Note::C))
  }
}

impl IntoParam for ScaleType {
  fn into_param(
    value: Option<&str>,
    name: &str,
  ) -> Result<Self, leptos_router::params::ParamsError> {
    if value.is_none() {
      return Err(leptos_router::params::ParamsError::MissingParam(
        name.to_string(),
      ));
    }

    Ok(ScaleType::from_str(value.unwrap()).unwrap_or(ScaleType::Hepatonic(HeptaScaleType::Major)))
  }
}

#[derive(Clone, Copy, Params, PartialEq, Debug)]
struct ScaleExerciseParams {
  root_note: Note,
  scale_type: ScaleType,
  min_fret: Option<u8>,
  max_fret: Option<u8>,
}

#[component]
pub fn ScaleExercisePage() -> impl IntoView {
  let query = use_query::<ScaleExerciseParams>();
  let scale = Signal::derive(move || {
    let scale_params = match query.get() {
      Ok(scale_params) => {
        leptos::logging::log!("OK: Got params: {:?}", scale_params);
        scale_params
      }
      Err(err) => {
        leptos::logging::log!("ERROR: Failed to get params: {:?}", err);
        ScaleExerciseParams {
          root_note: Note::C,
          scale_type: ScaleType::Hepatonic(HeptaScaleType::Major),
          min_fret: Some(0),
          max_fret: Some(12),
        }
      }
    };

    ScaleExercise {
      root_note: scale_params.root_note,
      scale_type: scale_params.scale_type,
      fret_range: (
        scale_params.min_fret.unwrap_or(0),
        scale_params.max_fret.unwrap_or(12),
      ),
    }
  });

  view! { <ScalePracticeSession exercise=scale></ScalePracticeSession> }
}
