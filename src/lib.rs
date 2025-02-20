use leptos::{prelude::ElementChild, prelude::*};
use leptos_meta::*;
use leptos_router::{components::*, path};

mod music;
mod components;

use crate::components::fretboard::Fretboard;

#[component]
pub fn App() -> impl IntoView {

  provide_meta_context();


  view! {
    <Html attr:lang="en" attr:dir="ltr" />
    <Title text="♫ Rust Guitar App ♫" />
    
    <Meta charset="UTF-8" />
    <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
    
    
    <div>
      <h1 class="py-6 text-6xl font-bold text-center text-cyan-700">"Gitarren Griffbrett"</h1>
      <Fretboard num_frets=24 num_strings=6 />
    </div>
  }
}
