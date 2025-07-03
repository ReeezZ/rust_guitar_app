use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
  view! {
      <div class="w-full flex justify-center">
          <h1>"Uh oh!" <br /> "We couldn't find that page!"</h1>
      </div>
  }
}
