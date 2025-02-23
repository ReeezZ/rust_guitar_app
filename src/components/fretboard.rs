use leptos::either::EitherOf3;
use leptos::prelude::*;

// #[component]
// pub fn FretNoteButton(#[prop()] _note: Note) -> impl IntoView {}

#[component]
pub fn Fretboard(
  #[prop(default = 6)] num_strings: u8,
  #[prop(default = 15)] num_frets: u8,
) -> impl IntoView {
  view! {
    <div class="relative py-10 px-14 bg-cyan-700">
      <div class="flex justify-center items-center trapezoid-shadow">
        // fretboard end
        <div class="absolute -right-5 w-10 h-[288px] trapezoid-end bg-[#be975b] bg-fretboard">
          <div class="absolute right-4 z-10 w-1 h-[288px] bg-[linear-gradient(90deg,_#bbbbbb_40%,_#444433_100%,_#48a499)]"></div>
        </div>
        <div class="relative flex-col trapezoid grow bg-[#be975b] bg-fretboard">
          {(0..num_strings)
            .map(|string_no| {
              view! { <FretboardString string_no=string_no num_frets=num_frets /> }
            })
            .collect::<Vec<_>>()} // Fret markers row (positioned below the frets)
          <FretboardDetails num_frets=num_frets />
        </div>
      </div>
    </div>
  }
}

#[component]
fn FretboardString(string_no: u8, num_frets: u8) -> impl IntoView {
  let string_strength = 2.0 + 0.5 * string_no as f64;
  view! {
    <div class="flex relative justify-start items-center w-full tilt">
      // Nut (Thick first fret)
      <div class="relative z-30 justify-center items-center w-8 h-6 border-r-8 border-transparent">
        <span class="absolute w-12 font-bold text-center text-white transition-transform cursor-pointer hover:scale-110 drop-shadow-[0_2px_2px_rgba(0,0,0,1)] active:scale-[98%]">
          {string_no}-0
        </span>
      </div>
      // Fretboard Section (Holds both string + frets)
      <div class="flex relative grow">
        // String
        <div
          class="absolute right-0 -left-60 top-1/2 z-20 -translate-y-1/2 drop-shadow-[0_2px_2px_rgba(0,0,0,0.6)] bg-[repeating-linear-gradient(45deg,_#dddddd,_#555555_2px,_#333333_2px)]"
          style:height=move || format!("{}px", string_strength)
        ></div>

        // Frets
        {(1..=num_frets)
          .map(|fret_no| {
            view! {
              <div class="flex relative justify-center items-center w-full h-12 text-center bg-transparent grow fretbar-container">
                <span class="z-20 font-bold text-center text-white transition-transform cursor-pointer hover:scale-110 drop-shadow-[0_2px_2px_rgba(0,0,0,1)] active:scale-[98%]">
                  {string_no}- {fret_no}
                </span>
              </div>
            }
          })
          .collect::<Vec<_>>()}
      </div>
    </div>
  }
}

#[component]
pub fn FretboardDetails(#[prop()] num_frets: u8) -> impl IntoView {
  view! {
    <div class="flex absolute justify-start w-full top-[48%] -z-10">
      // First fret/String guide details
      <div class="flex relative -top-32 w-8 h-80 -z-10 bg-[linear-gradient(90deg,_#000_20%,_#333333_100%,_#a8a499)] border-r-[8px] [border-image:linear-gradient(0.25turn,#aaaaaa,#ffffff,#aaaaaa)_1_100%]"></div>
      {(1..=num_frets)
        .map(|fret_no| {
          let has_marker = [3, 5, 7, 9, 15, 17, 19, 21].contains(&(fret_no % 12));
          let is_double = fret_no % 12 == 0;

          view! {
            <div class="flex relative justify-center items-center h-4 grow">
              {move || {
                if is_double {
                  EitherOf3::A(
                    view! {
                      <>
                        // is_double
                        <div class="absolute w-4 h-4 rounded-full border translate-y-12 border-[#a1a09f] bg-[linear-gradient(152deg,_#fff_20%,_#a8a499_100%,_#a8a499)]"></div>
                        <div class="absolute w-4 h-4 rounded-full border -translate-y-12 border-[#a1a09f] bg-[linear-gradient(152deg,_#fff_20%,_#a8a499_100%,_#a8a499)]"></div>
                        <div class="absolute left-full w-1 h-80 -z-10 bg-[linear-gradient(90deg,_#bbbbbb_40%,_#444433_100%,_#48a499)]"></div>
                      </>
                    },
                  )
                } else if has_marker {
                  EitherOf3::B(
                    view! {
                      <div class="absolute w-4 h-4 rounded-full border border-[#a1a09f] bg-[linear-gradient(152deg,_#fff_20%,_#a8a499_100%,_#a8a499)]"></div>
                      <div class="absolute left-full w-1 h-80 -z-10 bg-[linear-gradient(90deg,_#bbbbbb_40%,_#444433_100%,_#48a499)]"></div>
                    },
                  )
                } else {
                  EitherOf3::C(
                    view! {
                      <>
                        <div class="absolute left-full w-1 h-80 -z-10 bg-[linear-gradient(90deg,_#bbbbbb_40%,_#444433_100%,_#48a499)]"></div>
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
