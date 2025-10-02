use crate::{
  components::{
    exercises::PositionPresetButtons,
    fretboard::base::MAX_FRETS,
    ui::{
      title::{HeadingLevel, Title},
      Button, ButtonVariant,
    },
  },
  models::exercise::{Exercise, ScaleExercise},
};
use crate::{music::notes::NoteExt, music::Note, music::ScaleType};
use leptos::prelude::*;

#[component]
pub fn ConfigurationHeader(
  exercise: Signal<Exercise>,
  on_exercise_update: Callback<Exercise>,
) -> impl IntoView {
  // Modal states for exercise configuration
  let show_root_note_modal = RwSignal::new(false);
  let show_scale_type_modal = RwSignal::new(false);
  let show_fret_range_modal = RwSignal::new(false);

  view! {
    <div class="p-3 mb-6 bg-gray-50 rounded-lg dark:bg-gray-950">
      <div class="flex flex-wrap gap-4 items-center text-sm">

        {match exercise.get() {
          Exercise::Scale(scale) => {
            let ScaleExercise { root_note, scale_type, fret_range } = scale;
            view! {
              <>
                <RootNoteSelection
                  root_note
                  exercise
                  show_fret_range_modal
                  show_root_note_modal
                  show_scale_type_modal
                  on_exercise_update
                />
                <ScaleSelection
                  scale_type
                  show_fret_range_modal
                  show_root_note_modal
                  show_scale_type_modal
                />
                <FretRangeSelection
                  exercise
                  show_fret_range_modal
                  show_root_note_modal
                  show_scale_type_modal
                  on_exercise_update
                  active_min_fret=fret_range.0
                  active_max_fret=fret_range.1
                />
              </>
            }
              .into_any()
          }
          _ => ().into_any(),
        }} <div class="flex gap-2 items-center">
          <span class="font-medium">"Details:"</span>
          <span class="text-xs">{exercise.get().to_string()}</span>
        </div>

      </div>
    </div>
  }
}

#[component]
fn RootNoteSelection(
  exercise: Signal<Exercise>,
  show_root_note_modal: RwSignal<bool>,
  show_scale_type_modal: RwSignal<bool>,
  show_fret_range_modal: RwSignal<bool>,
  on_exercise_update: Callback<Exercise>,
  root_note: Note,
) -> impl IntoView {
  // Temporary selection state for root note modal
  let temp_selected_note = RwSignal::new(None::<Note>);

  let on_confirm_note_change = move || {
    if let Some(selected_note) = temp_selected_note.get() {
      temp_selected_note.set(Some(selected_note));
      show_root_note_modal.set(false);
      let mut exercise = exercise.get().clone();
      exercise.set_root_note(selected_note);
      on_exercise_update.run(exercise);
    }
  };

  view! {
    <div class="flex relative gap-2 items-center">
      <span class="font-medium">"Root:"</span>
      <Button
        variant=ButtonVariant::Primary
        on_click=move || {
          show_scale_type_modal.set(false);
          show_fret_range_modal.set(false);
          temp_selected_note.set(None);
          show_root_note_modal.set(!show_root_note_modal.get());
        }
        title="Click to change root note".to_string()
      >
        {root_note.to_string()}
      </Button>

      // Root note dropdown
      <Show when=move || show_root_note_modal.get()>
        <div class="absolute left-1/2 top-full z-10 mt-1 w-32 bg-white rounded-lg border border-gray-300 shadow-lg transform -translate-x-1/2 dark:bg-black dark:border-gray-700">
          <Title level=HeadingLevel::H6 text="Select Root Note" />
          <div class="flex flex-col justify-center items-center p-2 m-2">
            {move || {
              Note::all_notes()
                .iter()
                .map(move |&note| {
                  let is_root_note = note == root_note;
                  let is_current_root = move || {
                    note == temp_selected_note.get().unwrap_or(root_note)
                  };
                  view! {
                    <Button
                      variant=Signal::derive(move || {
                        if is_root_note {
                          ButtonVariant::Primary
                        } else if is_current_root() {
                          ButtonVariant::Special
                        } else {
                          ButtonVariant::Secondary
                        }
                      })
                      on_click=move || {
                        temp_selected_note.set(Some(note));
                      }
                    >
                      {note.to_short_string()}
                    </Button>
                  }
                })
                .collect::<Vec<_>>()
            }}
          </div>

          // Action buttons
          <div class="flex flex-col justify-end mt-2">
            <button
              class="px-1 my-1 text-sm text-gray-800 bg-red-100 rounded transition-colors hover:bg-red-300"
              on:click=move |_| {
                temp_selected_note.set(None);
                show_root_note_modal.set(false);
              }
            >
              "Cancel"
            </button>
            <button
              class="px-1 my-1 text-sm text-white bg-blue-500 rounded transition-colors hover:bg-blue-700 disabled:bg-gray-400"
              disabled=move || temp_selected_note.get().is_none()
              on:click=move |_| { on_confirm_note_change() }
            >
              "OK"
            </button>
          </div>
        </div>
      </Show>
    </div>
  }
}

#[component]
fn ScaleSelection(
  show_root_note_modal: RwSignal<bool>,
  show_fret_range_modal: RwSignal<bool>,
  show_scale_type_modal: RwSignal<bool>,
  scale_type: ScaleType,
) -> impl IntoView {
  let temporary_selected_scale = RwSignal::new(scale_type);

  view! {
    <div class="flex relative items-center">
      <span class="font-medium">"Scale:"</span>
      <Button
        variant=ButtonVariant::Special
        on_click=move || {
          show_root_note_modal.set(false);
          show_fret_range_modal.set(false);
          show_scale_type_modal.set(!show_scale_type_modal.get());
        }
        title="Click to change scale type".to_string()
      >
        {scale_type.to_string()}
      </Button>

      // Scale type dropdown
      <Show when=move || show_scale_type_modal.get()>
        <div class="absolute left-0 top-full z-10 p-4 mt-1 bg-white rounded-lg border border-gray-300 shadow-lg dark:bg-black min-w-[200px]">
          <Title level=HeadingLevel::H4 text="Select Scale Type" />
          <div class="flex flex-row justify-center items-center mb-4">
            <For
              each=move || ScaleType::all_scale_types()
              key=|scale_type| scale_type.to_string()
              let(button_scale_type)
            >
              <Button
                variant=Signal::derive(move || {
                  if button_scale_type == scale_type {
                    ButtonVariant::Primary
                  } else if button_scale_type == temporary_selected_scale.get() {
                    ButtonVariant::Special
                  } else {
                    ButtonVariant::Secondary
                  }
                })
                on_click=move || {
                  temporary_selected_scale.set(button_scale_type);
                }
              >
                {button_scale_type.to_string()}
              </Button>

            </For>
          </div>
          <div class="flex gap-2 justify-center items-center">
            <Button
              disabled=Signal::derive(move || temporary_selected_scale.get() == scale_type)
              variant=ButtonVariant::Primary
              on_click=move || {
                temporary_selected_scale.set(scale_type);
                show_scale_type_modal.set(false);
              }
            >
              "OK"
            </Button>
            <Button variant=ButtonVariant::Danger on_click=move || show_scale_type_modal.set(false)>
              "Cancel"
            </Button>
          </div>
        </div>
      </Show>
    </div>
  }
}

#[component]
fn FretRangeSelection(
  exercise: Signal<Exercise>,
  show_fret_range_modal: RwSignal<bool>,
  show_root_note_modal: RwSignal<bool>,
  show_scale_type_modal: RwSignal<bool>,
  active_min_fret: u8,
  active_max_fret: u8,
  on_exercise_update: Callback<Exercise>,
) -> impl IntoView {
  view! {
    <div class="flex relative gap-2 items-center">
      <span class="font-medium">"Frets:"</span>
      <Button
        variant=ButtonVariant::Danger
        on_click=move || {
          show_root_note_modal.set(false);
          show_scale_type_modal.set(false);
          show_fret_range_modal.set(!show_fret_range_modal.get());
        }
      >
        {active_min_fret}
        {"-"}
        {active_max_fret}
      </Button>

      // Fret range dropdown
      <Show when=move || {
        show_fret_range_modal.get()
      }>
        {
          let fret_range = RwSignal::new((active_min_fret, active_max_fret));
          let min_fret = move || fret_range.get().0;
          let max_fret = move || fret_range.get().1;
          view! {
            <div class="absolute left-0 top-full z-10 p-2 mt-1 bg-white rounded-lg border border-gray-300 shadow-lg min-w-[200px]">
              <h4 class="text-sm font-semibold">"Set Fret Range"</h4>

              // Fret range
              <div class="flex flex-row gap-2 justify-center items-center">
                <div>
                  <label class="block mx-1 mb-1 text-sm font-medium text-gray-700">Min Fret</label>
                  <input
                    type="number"
                    min="0"
                    max="24"
                    class="rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                    prop:value=move || min_fret().to_string()
                    on:input=move |e| {
                      if let Ok(val) = event_target_value(&e).parse::<u8>() {
                        fret_range.set((val, max_fret()));
                      }
                    }
                  />
                </div>
                <div>
                  <label class="block mx-1 mb-1 text-sm font-medium text-gray-700">Max Fret</label>
                  <input
                    type="number"
                    min="0"
                    max="24"
                    class="rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                    prop:value=move || max_fret().to_string()
                    on:input=move |e| {
                      if let Ok(val) = event_target_value(&e).parse::<u8>() {
                        fret_range.set((min_fret(), val.min(MAX_FRETS as u8)));
                      }
                    }
                  />
                </div>
              </div>

              <div>
                <PositionPresetButtons
                  on_preset_select=move |min, max| {
                    fret_range.set((min, max));
                  }
                  current_range=fret_range
                />
              </div>
              <div class="flex justify-center items-center">
                <button
                  class="py-1 px-3 text-xs font-medium text-white bg-blue-600 rounded hover:bg-blue-700"
                  on:click=move |_| {
                    let mut exercise = exercise.get().clone();
                    exercise.set_fret_range(fret_range.get());
                    on_exercise_update.run(exercise);
                  }
                >
                  "Apply"
                </button>
                <button
                  class="py-1 px-3 text-xs text-gray-600 bg-gray-200 rounded hover:bg-gray-300"
                  on:click=move |_| show_fret_range_modal.set(false)
                >
                  "Close"
                </button>
              </div>
            </div>
          }
        }
      </Show>
    </div>
  }
}
