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
    <div class="relative py-10 px-20 w-screen bg-cyan-700 rounded">
      <div class="relative flex-col trapezoid grow bg-[#be975b] bg-fretboard">
        {(0..num_strings)
          .map(|string_no| {
            let string_strength = 2.0 + 0.65 * string_no as f64;
            view! {
              <div class="flex relative justify-start items-center w-full string">
                // Nut (Thick first fret)
                // background: linear-gradient(0.25turn, #c9c9c9, #ffffff, #c9c9c9);
                <div class="justify-center items-center w-14 font-bold text-center text-white border-r-8 border-transparent">
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
                        // bg-[linear-gradient(90deg,_#bbbbbb_40%,_#444433_100%,_#48a499)]
                        <div class="flex relative justify-center items-center w-full h-12 text-center bg-transparent grow fretbar-container">
                          <span class="z-20">{string_no}- {fret_no}</span>
                        // <div class="absolute w-1 h-14 left-1 fretbar border-[1px] border-black"></div>
                        </div>
                      }
                    })
                    .collect::<Vec<_>>()}
                </div>
              </div>
            }
          })
          .collect::<Vec<_>>()} // Fret markers row (positioned below the frets)
        <FretboardMarker num_frets=num_frets />
      </div>
      // shadow
      <div class="z-50 w-full h-1 shadow-2xl -translate-y-4 rotate-[0.65deg]"></div>
    </div>
  }
}

#[component]
pub fn FretboardMarker(#[prop()] num_frets: u8) -> impl IntoView {
  view! {
    <div class="flex absolute justify-start w-full top-[48%]">
      // Empty space for the nut
      <div class="flex relative -top-32 w-14 h-80 -z-10 bg-[linear-gradient(90deg,_#000_20%,_#333333_100%,_#a8a499)] border-r-[8px] [border-image:linear-gradient(0.25turn,#aaaaaa,#ffffff,#aaaaaa)_1_100%]"></div>
      {(1..=num_frets)
        .map(|fret_no| {
          let has_marker = [3, 5, 7, 9, 15, 17, 19, 21].contains(&(fret_no % 12));
          let is_double = fret_no % 12 == 0;

          view! {
            <div class="flex relative flex-grow justify-center items-center h-4">
              {move || {
                if is_double {
                  EitherOf3::A(
                    view! {
                      <>
                        <div class="absolute w-4 h-4 rounded-full border translate-y-12 border-[#a1a09f] bg-[linear-gradient(152deg,_#fff_20%,_#a8a499_100%,_#a8a499)]"></div>
                        <div class="absolute w-4 h-4 rounded-full border -translate-y-12 border-[#a1a09f] bg-[linear-gradient(152deg,_#fff_20%,_#a8a499_100%,_#a8a499)]"></div>
                        <div class="absolute left-full w-1 h-80 bg-[linear-gradient(90deg,_#bbbbbb_40%,_#444433_100%,_#48a499)]"></div>
                      </>
                    },
                  )
                } else if has_marker {
                  EitherOf3::B(
                    view! {
                      <div class="absolute w-4 h-4 rounded-full border border-[#a1a09f] bg-[linear-gradient(152deg,_#fff_20%,_#a8a499_100%,_#a8a499)]"></div>
                      <div class="absolute left-full w-1 h-80 bg-[linear-gradient(90deg,_#bbbbbb_40%,_#444433_100%,_#48a499)]"></div>
                    },
                  )
                } else {
                  EitherOf3::C(
                    view! {
                      <>
                        <div class="absolute left-full w-1 h-80 bg-[linear-gradient(90deg,_#bbbbbb_40%,_#444433_100%,_#48a499)]"></div>
                      </>
                    },
                  )
                }
              }}
            </div>
          }
        })
        .collect::<Vec<_>>()}
    </div>
  }
}
