use std::rc::Rc;

use crate::models::fretboard_model::FretboardModel;
use crate::music::notes::Note;
use leptos::either::{Either, EitherOf3};
use leptos::prelude::*;

// #[component]
// pub fn FretNoteButton(#[prop()] _note: Note) -> impl IntoView {}

// We can change the return type of the Function to a struct that contains more info like is_blue_note, is_root_note, etc.

#[component]
pub fn Fretboard(
  #[prop(default = 6)] num_strings: u8,
  #[prop(default = 15)] num_frets: u8,
) -> impl IntoView {
  let model = use_context::<FretboardModel>().expect("FretboardModel not in context");

  // Render code similar to what you have, but simpler props
  view! {
    <div class="relative py-16 px-14 bg-primary-shades trans">
      // Your fretboard HTML
      <div class="relative flex-col trapezoid grow bg-[#917140] bg-fretboard">
        {(0..num_strings)
          .map(|string_no| {
            let string_note = match string_no {
              0 => Note::E,
              1 => Note::A,
              2 => Note::D,
              3 => Note::G,
              4 => Note::H,
              5 => Note::E,
              _ => Note::E,
            };

            view! {
              <FretboardString
                string_no=string_no
                num_frets=num_frets
                string_note=string_note
                model=model.clone()
              />
            }
          })
          .collect_view()} <FretboardDetails num_frets=num_frets />
      </div>
    </div>
  }
}

#[component]
fn FretboardString(
  #[prop()] string_no: u8,
  #[prop()] num_frets: u8,
  #[prop()] string_note: Note,
  model: FretboardModel,
) -> impl IntoView {
  let string_strength = 2.0 + 0.5 * string_no as f64;
  let open_note = string_note;
  let open_note_display = model.evaluate_note(open_note);

  view! {
    <div class="flex relative justify-start items-center w-full tilt">
      <div class="relative z-30 justify-center items-center w-8 h-6 border-r-8 border-transparent">
        {move || {
          let display = open_note_display.get();
          if display.visible {
            Either::Left(
              view! {
                <span class=move || {
                  let mut classes = "absolute w-12 font-bold text-center text-white transition-transform cursor-pointer hover:scale-110 drop-shadow-[0_2px_2px_rgba(0,0,0,1)] active:scale-[98%]"
                    .to_string();
                  if display.is_root {
                    classes.push_str(" bg-red-500 rounded-full");
                  }
                  classes
                }>{display.display_text}</span>
              },
            )
          } else {
            Either::Right(view! { <span></span> })
          }
        }}
      </div>

      // Rest of the string rendering
      <div class="flex relative grow">
        // String line
        <div
          class="absolute right-0 -left-60 top-1/2 z-20 -translate-y-1/2 drop-shadow-[0_2px_2px_rgba(0,0,0,0.6)] bg-[repeating-linear-gradient(45deg,_#dddddd,_#555555_2px,_#333333_2px)]"
          style:height=move || format!("{}px", string_strength)
        ></div>

        // Frets
        {(1..=num_frets)
          .map(|fret_no| {
            let fret_note = string_note.add_steps(fret_no as usize);
            let fret_note_display = model.evaluate_note(fret_note);

            view! {
              <div class="flex relative justify-center items-center w-full h-12 text-center bg-transparent grow fretbar-container">
                {move || {
                  let display = fret_note_display.get();
                  if display.visible {
                    Either::Left(
                      view! {
                        <span class=move || {
                          let mut classes = "z-20 font-bold text-center text-white transition-transform cursor-pointer hover:scale-110 drop-shadow-[0_2px_2px_rgba(0,0,0,1)] active:scale-[98%]"
                            .to_string();
                          if display.is_root {
                            classes.push_str(" bg-red-500 rounded-full px-1");
                          }
                          classes
                        }>{display.display_text}</span>
                      },
                    )
                  } else {
                    Either::Right(view! { <span></span> })
                  }
                }}
              </div>
            }
          })
          .collect_view()}
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
        .collect_view()}
    </div>
  }
}
