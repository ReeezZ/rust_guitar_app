use leptos::prelude::*;

#[component]
pub fn Fretboard(
  #[prop(default = 6)] num_strings: u8,
  #[prop(default = 15)] num_frets: u8,
) -> impl IntoView {
  view! {
    <div class="flex-col p-4 my-12 w-full h-96 bg-amber-500">
      {(0..num_strings)
        .map(|string_no| {
          view! {
            <div class="flex relative items-center">
              // Nut (Fret 0) - thicker left border
              <div class="flex justify-center content-center items-center w-20 h-12 font-bold text-center text-white bg-gray-800 border-l-8 border-slate-900">
                {string_no}- 0
              </div>

              // String - Now starts *after* the nut
              <div
                class="absolute left-20 top-1/2 h-1 bg-slate-300"
                style="width: calc(100% - 20px);"
              ></div>

              // Frets
              {(1..=num_frets)
                .map(|fret_no| {
                  view! {
                    <div class="flex relative justify-center content-center items-center w-20 h-12 text-center border-r-4 border-slate-700">
                      {string_no}- {fret_no}
                    </div>
                  }
                })
                .collect::<Vec<_>>()}
            </div>
          }
        })
        .collect::<Vec<_>>()}
    </div>
  }
}
