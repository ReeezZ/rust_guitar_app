use leptos::either::EitherOf3;
use leptos::prelude::*;

#[component]
pub fn Fretboard(
  #[prop(default = 6)] num_strings: u8,
  #[prop(default = 15)] num_frets: u8,
) -> impl IntoView {
  view! {
    <div class="flex-col w-full bg-amber-500">
      // Strings and frets
      {(0..num_strings)
        .map(|string_no| {
          view! {
            <div class="flex relative items-center w-full">
              // Nut (Thick first fret)
              <div class="flex justify-center items-center w-20 h-12 font-bold text-center text-white bg-gray-800 border-r-8 border-slate-900">
                {string_no}- 0
              </div>

              // Fretboard Section (Holds both string + frets)
              <div class="flex relative grow">
                // String
                <div class="absolute right-0 left-0 top-1/2 h-1 -translate-y-1/2 bg-slate-300"></div>

                // Frets
                {(1..=num_frets)
                  .map(|fret_no| {
                    view! {
                      <div class="flex z-10 justify-center items-center w-20 h-12 text-center bg-transparent border-r-4 border-slate-700">
                        {string_no}- {fret_no}
                      </div>
                    }
                  })
                  .collect::<Vec<_>>()}
              </div>
            </div>
          }
        })
        .collect::<Vec<_>>()} // Fret markers row (positioned below the frets)
      <div class="flex justify-start w-full">
        // Empty space for the nut
        <div class="w-20 h-6"></div>
        {(1..=num_frets)
          .map(|fret_no| {
            let has_marker = [3, 5, 7, 9, 15, 17, 19, 21].contains(&(fret_no % 12));
            let is_double = fret_no % 12 == 0;

            view! {
              <div class="flex relative justify-center items-center w-20 h-6">
                {move || {
                  if is_double {
                    EitherOf3::A(
                      view! {
                        <>
                          <div class="absolute left-1/4 w-2 h-2 bg-black rounded-full"></div>
                          <div class="absolute right-1/4 w-2 h-2 bg-black rounded-full"></div>
                        </>
                      },
                    )
                  } else if has_marker {
                    EitherOf3::B(view! { <div class="w-2 h-2 bg-black rounded-full"></div> })
                  } else {
                    EitherOf3::C(view! { <></> })
                  }
                }}
              </div>
            }
          })
          .collect::<Vec<_>>()}
      </div>
    </div>
  }
}
