use leptos::prelude::*;

use crate::{ pages::fretboard_dev::shared_model_demo::SharedModelDemo};

#[component]
pub fn FretboardDevPage() -> impl IntoView {
  view! { <SharedModelDemo /> }
}
