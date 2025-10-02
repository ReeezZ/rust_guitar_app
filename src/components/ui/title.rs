use leptos::prelude::*;

pub enum HeadingLevel {
  H1,
  H2,
  H3,
  H4,
  H5,
  H6,
}

#[component]
pub fn Title(text: &'static str, level: HeadingLevel) -> impl IntoView {
  match level {
    HeadingLevel::H1 => view! { <h1 class="mb-6 text-3xl font-bold">{text}</h1> }.into_any(),
    HeadingLevel::H2 => view! { <h2 class="mb-5 text-2xl font-semibold">{text}</h2> }.into_any(),
    HeadingLevel::H3 => view! { <h3 class="mb-4 text-xl font-semibold">{text}</h3> }.into_any(),
    HeadingLevel::H4 => view! { <h4 class="mb-3 text-lg font-medium">{text}</h4> }.into_any(),
    HeadingLevel::H5 => view! { <h5 class="mb-2 text-base font-medium">{text}</h5> }.into_any(),
    HeadingLevel::H6 => view! { <h6 class="mb-1 text-sm font-medium">{text}</h6> }.into_any(),
  }
}
