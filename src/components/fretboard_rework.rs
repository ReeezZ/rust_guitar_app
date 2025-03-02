use leptos::either::EitherOf3;
use leptos::logging::log;
use leptos::prelude::*;

use crate::music::notes::Note;

use super::fretboard_model::{FretCoord, FretState, FretboardModel};

#[component]
pub fn FretboardRework(#[prop()] fretboard: RwSignal<FretboardModel>) -> impl IntoView {
  view! {
    <div class="relative py-16 px-14 bg-primary-shades trans">
      <div class="flex justify-center items-center trapezoid-shadow">
        // fretboard end
        <div class="absolute -right-5 w-10 h-[288px] trapezoid-end bg-[#917140] bg-fretboard">
          <div class="absolute right-4 z-10 w-1 h-[288px] bg-[linear-gradient(90deg,_#bbbbbb_40%,_#444433_100%,_#48a499)]"></div>
        </div>
        <div class="relative flex-col trapezoid grow bg-[#917140] bg-fretboard">
          {move || {
            let num_strings = fretboard.with(|fb| fb.get_num_strings());
            (0..num_strings)
              .rev()
              .map(|string_no| {
                let string_note = fretboard.with(|fb| fb.get_tuning()[string_no as usize]);
                let fretboard_for_string = fretboard;

                // clone the fretboard for each string

                view! {
                  <FretboardString
                    string_no=string_no
                    num_frets=fretboard_for_string.with(|fb| fb.get_num_frets())
                    string_note=string_note
                    fret_state_signals=fretboard_for_string
                      .with(|fb| fb.get_frets_of_string(string_no).clone())
                  />
                }
              })
              .collect_view()
          }}
          {move || view! { <FretboardDetails num_frets=fretboard.with(|fb| fb.get_num_frets()) /> }}
        </div>
      </div>
    </div>
  }
}

#[component]
fn FretboardString(
  #[prop()] string_no: u8,
  #[prop()] num_frets: u8,
  #[prop()] string_note: Note,
  #[prop()] fret_state_signals: Vec<RwSignal<FretState>>,
) -> impl IntoView {
  let string_strength = 2.0 + 0.5 * string_no as f64;

  view! {
    <div class="flex relative justify-start items-center w-full tilt">
      <div class="relative z-30 justify-center items-center w-8 h-6 border-r-8 border-transparent">
        <FretboardNote
          note=string_note
          coord=FretCoord {
            string_idx: string_no,
            fret_idx: 0,
          }
          fret_state_signal=fret_state_signals[0]
        />
      </div>

      <div class="flex relative grow">
        <div
          class="absolute right-0 -left-60 top-1/2 z-20 -translate-y-1/2 drop-shadow-[0_2px_2px_rgba(0,0,0,0.6)] bg-[repeating-linear-gradient(45deg,_#dddddd,_#555555_2px,_#333333_2px)]"
          style:height=move || format!("{}px", string_strength)
        ></div>

        {move || {
          (1..=num_frets)
            .map(|fret_no| {
              view! {
                <div class="flex relative justify-center items-center w-full h-12 text-center bg-transparent grow fretbar-container">
                  <FretboardNote
                    note=string_note.add_steps(fret_no as usize)
                    coord=FretCoord {
                      string_idx: string_no,
                      fret_idx: fret_no,
                    }
                    fret_state_signal=fret_state_signals[fret_no as usize]
                  />
                </div>
              }
            })
            .collect_view()
        }}
      </div>
    </div>
  }
}

#[component]
fn FretboardNote(
  #[prop()] note: Note,
  #[prop()] coord: FretCoord,
  #[prop()] fret_state_signal: RwSignal<FretState>,
) -> impl IntoView {
  // Toggle function to demonstrate interaction
  let toggle = move |_| {
    // Debug output
    log!(
      "Toggled fret at string {} fret {}",
      coord.string_idx,
      coord.fret_idx
    );

    match fret_state_signal.get() {
      FretState::Hidden => fret_state_signal.set(FretState::Normal),
      FretState::Normal => fret_state_signal.set(FretState::Root),
      FretState::Root => fret_state_signal.set(FretState::Hidden),
    };
  };

  view! {
    <div on:click=toggle>
      {move || {
        match fret_state_signal.get() {
          FretState::Root => {
            EitherOf3::A(
              view! {
                <span class="relative z-20 font-bold text-center text-white transition-transform cursor-pointer hover:scale-110 drop-shadow-[0_2px_2px_rgba(0,0,0,1)] active:scale-[98%]">
                  <span class="absolute inset-0 z-10 w-full h-full bg-red-500 rounded-full opacity-50"></span>
                  <span class="relative z-20">{note.to_string()}</span>
                </span>
              },
            )
          }
          FretState::Normal => {
            EitherOf3::B(
              view! {
                <span class="relative z-20 font-bold text-center text-white transition-transform cursor-pointer hover:scale-110 drop-shadow-[0_2px_2px_rgba(0,0,0,1)] active:scale-[98%]">
                  <span class="absolute inset-0 z-10 w-full h-full rounded-full opacity-20 bg-slate-400"></span>
                  <span class="relative z-20">{note.to_string()}</span>
                </span>
              },
            )
          }
          _ => EitherOf3::C(view! { <span></span> }),
        }
      }}
    </div>
  }
}

#[component]
pub fn FretboardDetails(#[prop()] num_frets: u8) -> impl IntoView {
  view! {
    <div class="flex absolute justify-start w-full top-[48%] -z-10">
      // First fret/String guide details
      <div class="flex relative -top-32 w-8 h-80 -z-10 bg-[linear-gradient(90deg,_#000_20%,_#333333_100%,_#a8a499)] border-r-[8px] [border-image:linear-gradient(0.25turn,#aaaaaa,#ffffff,#aaaaaa)_1_100%]"></div>
      {move || {
        (1..=num_frets)
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
          .collect_view()
      }}
    </div>
  }
}
