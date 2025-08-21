use std::collections::HashMap;

use leptos::prelude::*;

use crate::fretboard::{
  components::{
    base::{FretClickEvent, FretState, FretStateColor, Fretboard},
    with_notes::{FretClickEventWithNote, FretboardWithNotes},
  },
  model::FretCoord,
};

fn get_fret_positions() -> HashMap<FretCoord, Signal<FretState>> {
  // Build a sample set of fret states to visualize different cases
  let mut fret_positions: HashMap<FretCoord, Signal<FretState>> = HashMap::new();

  // Normal notes across several strings/frets
  for (s, f) in [(0, 5), (1, 3), (2, 7), (3, 2)] {
    let sig = RwSignal::new(FretState::Normal(
      FretStateColor::Green,
      format!("{}-{}", f, s),
    ));
    fret_positions.insert(
      FretCoord {
        string_idx: s,
        fret_idx: f,
      },
      sig.into(),
    );
  }

  // Colored examples
  fret_positions.insert(
    FretCoord {
      string_idx: 4,
      fret_idx: 8,
    },
    RwSignal::new(FretState::Normal(FretStateColor::Blue, "foo".into())).into(),
  );
  fret_positions.insert(
    FretCoord {
      string_idx: 5,
      fret_idx: 0,
    },
    RwSignal::new(FretState::Normal(FretStateColor::Red, "foo".into())).into(),
  );
  fret_positions.insert(
    FretCoord {
      string_idx: 5,
      fret_idx: 4,
    },
    RwSignal::new(FretState::Normal(
      FretStateColor::Red,
      "loooooooong text".into(),
    ))
    .into(),
  );

  // A hidden example (should not render) - included to ensure Hidden is ignored
  fret_positions.insert(
    FretCoord {
      string_idx: 2,
      fret_idx: 9,
    },
    RwSignal::new(FretState::Hidden).into(),
  );

  fret_positions
}

/// Page for the SVG fretboard with a runtime-adjustable fret count slider.
/// See: https://leptos.dev/docs/reference/signals/
#[component]
pub fn FretboardDevPage() -> impl IntoView {
  let frets = RwSignal::new(get_fret_positions());

  let label = RwSignal::new(String::from("Label"));
  let color = RwSignal::new(FretStateColor::Green);
  let hidden = RwSignal::new(false);

  let update_fret = Callback::new(move |coord: FretCoord| {
    frets.update(|map| {
      let state = if hidden.get() {
        FretState::Hidden
      } else {
        FretState::Normal(color.get(), label.get())
      };
      map.insert(coord, RwSignal::new(state).into());
    });
  });

  let handle_note_clicked = Callback::new(move |coord: FretClickEventWithNote| {
    leptos::logging::log!("{:?} {:?}", coord.note, coord.coord);
    update_fret.run(coord.coord);
  });

  let handle_fret_clicked = Callback::new(move |coord: FretClickEvent| {
    leptos::logging::log!("{:?}", coord);
    update_fret.run(coord.coord);
  });

  view! {
    <h1 class="mb-2 text-xl font-bold">"Fretboard Dev: FretboardWithNotes"</h1>
    <p class="mb-4 text-sm text-gray-600">
      Test page showing a variety of FretState values (Normal, Colored, Hidden).
    </p>
    <FretsEditor frets label color hidden />
    <div>
      <FretboardWithNotes
        fret_states=frets
        start_fret=0
        end_fret=12
        on_note_clicked=handle_note_clicked
      />

    </div>

    <div>
      <h1 class="mb-2 text-xl font-bold">"Fretboard (base)"</h1>
      <Fretboard
        fret_states=frets.into()
        start_fret=0.into()
        end_fret=12.into()
        on_fret_clicked=handle_fret_clicked
      />
    </div>

    <div>
      <h1 class="mb-2 text-xl font-bold">
        "Fretboard (base) with no callback to check Clickable areas are not rendered"
      </h1>
      <Fretboard fret_states=frets.into() start_fret=0.into() end_fret=12.into() />
    </div>
  }
}

#[component]
fn FretsEditor(
  frets: RwSignal<HashMap<FretCoord, Signal<FretState>>>,
  #[prop(into)] label: RwSignal<String>,
  #[prop(into)] color: RwSignal<FretStateColor>,
  #[prop(into)] hidden: RwSignal<bool>,
) -> impl IntoView {
  let reset_sample = move |_| frets.set(get_fret_positions());

  view! {
    <div class="p-4 mt-4 space-y-3 bg-gray-50 rounded border">
      <h2 class="font-semibold">"Edit Fret State"</h2>
      <div class="flex flex-wrap gap-4 items-end">
        <label class="flex flex-col text-sm">
          <span>"Label"</span>
          <input
            r#type="text"
            class="p-1 rounded border"
            prop:value=move || label.get()
            on:input=move |ev| label.set(event_target_value(&ev))
          />
        </label>
        <label class="flex flex-col text-sm">
          <span>"Color"</span>
          <select
            class="p-1 rounded border"
            on:change=move |ev| {
              match event_target_value(&ev).as_str() {
                "Red" => color.set(FretStateColor::Red),
                "Blue" => color.set(FretStateColor::Blue),
                _ => color.set(FretStateColor::Green),
              }
            }
          >
            <option value="Green" selected=move || color.get() == FretStateColor::Green>
              "Green"
            </option>
            <option value="Red" selected=move || color.get() == FretStateColor::Red>
              "Red"
            </option>
            <option value="Blue" selected=move || color.get() == FretStateColor::Blue>
              "Blue"
            </option>
          </select>
        </label>
        <label class="flex gap-2 items-center mt-4 text-sm">
          <input
            r#type="checkbox"
            prop:checked=move || hidden.get()
            on:change=move |ev| {
              use leptos::wasm_bindgen::JsCast;
              if let Some(t) = ev.target() {
                if let Ok(input) = t.dyn_into::<web_sys::HtmlInputElement>() {
                  hidden.set(input.checked());
                }
              }
            }
          />
          <span>"Hidden"</span>
        </label>
        <button class="py-1 px-3 text-gray-800 bg-gray-300 rounded" on:click=reset_sample>
          "Reset Sample"
        </button>
      </div>
    </div>
  }
}

// Local helper to extract value from input/select elements
fn event_target_value(ev: &leptos::ev::Event) -> String {
  use leptos::wasm_bindgen::JsCast;
  if let Some(target) = ev.target() {
    if let Ok(input) = target.clone().dyn_into::<web_sys::HtmlInputElement>() {
      return input.value();
    }
    if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
      return select.value();
    }
  }
  String::new()
}
