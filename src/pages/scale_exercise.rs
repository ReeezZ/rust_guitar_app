use std::str::FromStr;

use leptos::prelude::*;
use leptos_router::{
  hooks::use_query,
  params::{IntoParam, Params},
};

use crate::music::{heptatonic_scales::HeptaScaleType, Note, ScaleType};

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

#[derive(Params, PartialEq, Debug)]
struct ScaleExerciseParams {
  root_note: Note,
  scale_type: ScaleType,
  start_fret: Option<u8>,
  end_fret: Option<u8>,
}

#[component]
pub fn ScaleExercisePage() -> impl IntoView {
  let query = use_query::<ScaleExerciseParams>();

  match query.read().as_ref() {
    Ok(scale_params) => {
      leptos::logging::log!("OK: Got params: {:?}", scale_params);
    }
    Err(err) => leptos::logging::log!("ERROR: Failed to get params: {:?}", err),
  }
  // TODO:
}
