use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
  view! {
    <ErrorBoundary fallback=|errors| {
      view! {
        <h1>"Uh oh! Something went wrong!"</h1>

        <p>"Errors: "</p>
        // Render a list of errors as strings - good for development purposes
        <ul>
          {move || {
            errors.get().into_iter().map(|(_, e)| view! { <li>{e.to_string()}</li> }).collect_view()
          }}

        </ul>
      }
    }>

      <div class="flex justify-center w-full">
        <h1 class="text-4xl font-bold m-[100px] trans">
          "Welcome to our little Guitar App!  ¯\\_(ツ)_/¯"
        </h1>
      </div>

    </ErrorBoundary>
  }
}
