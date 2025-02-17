use leptos::either::EitherOf3;
use leptos::prelude::*;
use lib::types::Note;

#[component]
pub fn FretNoteButton(#[prop()] _note: Note) -> impl IntoView {}

#[component]
pub fn Fretboard(
  #[prop(default = 6)] num_strings: u8,
  #[prop(default = 15)] num_frets: u8,
) -> impl IntoView {
  view! {
    <div class="p-6 mx-16 mt-12 bg-amber-500 bg-opacity-70 rounded">
      <div class="bg-[#be975b] bg-fretboard relative">
        {(0..num_strings)
          .map(|string_no| {
            let string_strength = 2.0 + 0.65 * string_no as f64;
            view! {
              <div class="flex relative justify-start items-center w-full">
                // Nut (Thick first fret)
                <div class="flex justify-center items-center w-16 h-12 font-bold text-center text-white bg-[#494949] border-r-8 border-slate-900">
                  <span>{string_no}- 0</span>
                </div>
                // Fretboard Section (Holds both string + frets)
                <div class="flex relative grow">
                  // String
                  <div
                    class="absolute right-0 left-0 top-1/2 z-20 -translate-y-1/2 bg-[repeating-linear-gradient(45deg,_#dddddd,_#555555_2px,_#333333_2px)]"
                    style:height=move || format!("{}px", string_strength)
                  ></div>

                  // Frets
                  {(1..=num_frets)
                    .map(|fret_no| {
                      view! {
                        <div class="relative flex grow w-full justify-center h-12 items-center text-center bg-transparent border-r-4 [border-image:linear-gradient(0.25turn,#7c7c7c,#cccccc,#7c7c7c)_1_60%]">
                          <span class="z-20">{string_no}- {fret_no}</span>
                        </div>
                      }
                    })
                    .collect::<Vec<_>>()}
                </div>
              </div>
            }
          })
          .collect::<Vec<_>>()}
      // Fret markers row (positioned below the frets)
      <FretboardMarker num_frets=num_frets />
      </div>
    </div>
  }
}

#[component]
pub fn FretboardMarker(#[prop()] num_frets: u8) -> impl IntoView {
  view! {
    <div class="absolute top-[48%] flex justify-start w-full">
      // Empty space for the nut
      <div class="flex w-16 h-4 border-transparent border-r-[8px] "></div>
      {(1..=num_frets)
        .map(|fret_no| {
          let has_marker = [3, 5, 7, 9, 15, 17, 19, 21].contains(&(fret_no % 12));
          let is_double = fret_no % 12 == 0;

          view! {
            <div class="flex relative justify-center flex-grow items-center h-4">
              {move || {
                if is_double {
                  EitherOf3::A(
                    view! {
                      <>
                        <div class="absolute translate-y-12 w-4 h-4 border border-[#a1a09f] bg-[linear-gradient(152deg,_#fff_20%,_#a8a499_100%,_#a8a499)] rounded-full"></div>
                        <div class="absolute -translate-y-12 w-4 h-4 border border-[#a1a09f] bg-[linear-gradient(152deg,_#fff_20%,_#a8a499_100%,_#a8a499)] rounded-full"></div>
                      </>
                    },
                  )
                } else if has_marker {
                  EitherOf3::B(view! { 
                    <div class="absolute w-4 h-4 border border-[#a1a09f] bg-[linear-gradient(152deg,_#fff_20%,_#a8a499_100%,_#a8a499)] rounded-full"></div>
                   })
                } else {
                  EitherOf3::C(view! { <></> })
                }
              }}
            </div>
          }
        })
        .collect::<Vec<_>>()}
    </div>
  }
}
