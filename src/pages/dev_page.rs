use std::{fmt, str::FromStr};

use leptos::prelude::*;
use leptos_router::hooks::query_signal;

use crate::{
  components::exercises::scale::practice_configuration_header::RootNoteSelection,
  music::{heptatonic_scales::HeptaScaleType, Note, ScaleType},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
  A,
  B,
  C,
}

impl fmt::Display for State {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      State::A => "A",
      State::B => "B",
      State::C => "C",
    };
    write!(f, "{}", s)
  }
}

impl FromStr for State {
  type Err = ();

  fn from_str(input: &str) -> Result<State, Self::Err> {
    match input {
      "A" => Ok(State::A),
      "B" => Ok(State::B),
      "C" => Ok(State::C),
      _ => Err(()),
    }
  }
}

#[component]
pub fn DevPage() -> impl IntoView {
  let (state, set_state) = query_signal::<State>("state");
  let (note, set_note) = query_signal::<Note>("note");
  let (scale_type, set_scale_type) = query_signal::<ScaleType>("scale_type");

  let state = Signal::derive(move || state.get().unwrap_or(State::A));
  let on_state_changed = Callback::new(move |new_state: State| {
    set_state.set(Some(new_state));
  });

  let note = Signal::derive(move || note.get().unwrap_or(Note::C));
  let on_note_changed = Callback::new(move |new_note: Note| {
    set_note.set(Some(new_note));
  });

  let scale_type = Signal::derive(move || {
    scale_type
      .get()
      .unwrap_or(ScaleType::Hepatonic(HeptaScaleType::Major))
  });
  let on_scale_type_changed = Callback::new(move |new_scale_type: ScaleType| {
    set_scale_type.set(Some(new_scale_type));
  });

  view! {
    <div>
      <ChildComponent state on_state_changed />
      <NoteComponent note on_note_changed />
      <ScaleTypeComponent scale_type on_scale_type_changed />
      <RootNoteSelection
        show_root_note_modal=RwSignal::new(true)
        show_scale_type_modal=RwSignal::new(false)
        show_fret_range_modal=RwSignal::new(false)
        can_edit=Signal::derive(move || true)
        root_note=note
        on_root_note_changed=on_note_changed
      />
    </div>
  }
}

#[component]
pub fn ScaleTypeComponent(
  #[prop(into)] scale_type: Signal<ScaleType>,
  on_scale_type_changed: Callback<ScaleType>,
) -> impl IntoView {
  view! {
    <div>
      <p>"Scale Type is: " {move || scale_type.get().to_string()}</p>
      <button on:click=move |_| {
        on_scale_type_changed.run(ScaleType::Hepatonic(HeptaScaleType::Major))
      }>"Set Scale Type to Major"</button>
      <button on:click=move |_| {
        on_scale_type_changed.run(ScaleType::Hepatonic(HeptaScaleType::Minor))
      }>"Set Scale Type to Minor"</button>

    </div>
  }
}

#[component]
pub fn NoteComponent(note: Signal<Note>, on_note_changed: Callback<Note>) -> impl IntoView {
  view! {
    <div>
      <p>"Note is: " {move || note.get().to_string()}</p>
      <div class="space-x-2">
        {Note::all_notes()
          .iter()
          .map(|&n| {
            view! { <button on:click=move |_| on_note_changed.run(n)>{n.to_string()}</button> }
          })
          .collect::<Vec<_>>()}
      </div>
    </div>
  }
}

#[component]
fn ChildComponent(state: Signal<State>, on_state_changed: Callback<State>) -> impl IntoView {
  view! {
    <div>
      <p>
        "State is: "
        {move || match state.get() {
          State::A => "A",
          State::B => "B",
          State::C => "C",
        }}
      </p>
      <div class="space-x-2">
        <button on:click=move |_| on_state_changed.run(State::A)>"Set State to A"</button>
        <button on:click=move |_| on_state_changed.run(State::B)>"Set State to B"</button>
        <button on:click=move |_| on_state_changed.run(State::C)>"Set State to C"</button>
      </div>
    </div>
  }
}
