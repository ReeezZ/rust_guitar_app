use leptos::either::EitherOf3;
use leptos::prelude::*;

use crate::music::notes::Note;

// #[component]
// pub fn FretNoteButton(#[prop()] _note: Note) -> impl IntoView {}

// We can change the return type of the Function to a struct that contains more info like is_blue_note, is_root_note, etc.
type NoteToStringFn = fn(Note) -> String;
type IsNoteVisible = fn(Note) -> bool;

#[component]
pub fn Fretboard(
  #[prop(default = 6)] num_strings: u8,
  #[prop(default = 15)] num_frets: u8,
  is_note_visible_signal: ReadSignal<IsNoteVisible>,
  note_to_string_signal: ReadSignal<NoteToStringFn>,
) -> impl IntoView {
  // let (note_in_scale, set_note_in_scale) = signal(move |note: Note| -> bool { true });
  // let (note_to_string, set_note_to_string) =
  //   signal(move |note: Note| -> String { note.to_string() });
  let is_note_visible = is_note_visible_signal.get();
  let note_to_string = note_to_string_signal.get();

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
                  filter=is_note_visible
                  note_to_string=note_to_string
                />
              }
            })
            .collect::<Vec<_>>()} // Fret markers row (positioned below the frets)
          <FretboardDetails num_frets=num_frets />
        </div>
      </div>
    </div>
  }
}

fn note_for_fret(
  string_note: Note,
  fret_no: u8,
  filter: IsNoteVisible,
  note_to_string: NoteToStringFn,
) -> Option<String> {
  let note = string_note.add_steps(fret_no as usize);
  if filter(note) {
    Some(note_to_string(note))
  } else {
    None
  }
}

#[component]
fn FretboardString(
  #[prop()] string_no: u8,
  #[prop()] num_frets: u8,
  #[prop()] string_note: Note, // TODO change to Note trait, keep the depency clean
  filter: IsNoteVisible,
  note_to_string: NoteToStringFn,
) -> impl IntoView {
  let string_strength = 2.0 + 0.5 * string_no as f64;
  let note = note_for_fret(string_note, 0, filter, note_to_string);
  view! {
    <div class="flex relative justify-start items-center w-full tilt">
      <div class="relative z-30 justify-center items-center w-8 h-6 border-r-8 border-transparent">
        <span class="absolute w-12 font-bold text-center text-white transition-transform cursor-pointer hover:scale-110 drop-shadow-[0_2px_2px_rgba(0,0,0,1)] active:scale-[98%]">
          {note.unwrap_or("".to_string())}
        </span>
      </div>

      <div class="flex relative grow">
        <div
          class="absolute right-0 -left-60 top-1/2 z-20 -translate-y-1/2 drop-shadow-[0_2px_2px_rgba(0,0,0,0.6)] bg-[repeating-linear-gradient(45deg,_#dddddd,_#555555_2px,_#333333_2px)]"
          style:height=move || format!("{}px", string_strength)
        ></div>

        {(1..=num_frets)
          .map(|fret_no| {
            let note = note_for_fret(string_note, fret_no, filter, note_to_string);
            view! {
              <div class="flex relative justify-center items-center w-full h-12 text-center bg-transparent grow fretbar-container">
                <span class="z-20 font-bold text-center text-white transition-transform cursor-pointer hover:scale-110 drop-shadow-[0_2px_2px_rgba(0,0,0,1)] active:scale-[98%]">
                  {note.unwrap_or("".to_string())}
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
