use leptos::prelude::*;

#[component]
pub fn Fretboard(
  #[prop(default = 6)] num_strings: u8,
  #[prop(default = 15)] num_frets: u8,
) -> impl IntoView {
  view! {
    <div class="p-4 my-12 mx-auto min-w-fit max-w-fit min-h-fit bg-amber-500">
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
                // String (spans full width after nut)
                <div class="absolute right-0 left-0 top-1/2 h-1 -translate-y-1/2 bg-slate-300"></div>

                // Frets (sit on top of string)
                {(1..=num_frets)
                  .map(|fret_no| {
                    view! {
                      <div class="flex justify-center items-center w-20 h-12 z-30 text-center bg-transparent border-r-4 border-slate-700">
                        {string_no}- {fret_no}
                      </div>
                    }
                  })
                  .collect::<Vec<_>>()}
              </div>
            </div>
          }
        })
        .collect::<Vec<_>>()}
    </div>
  }
}
