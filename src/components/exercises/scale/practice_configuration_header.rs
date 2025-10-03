use crate::{
  components::{
    exercises::scale::PositionPresetButtons,
    ui::{
      button::ButtonColor,
      title::{HeadingLevel, Title},
      Button, ButtonVariant, FretRangeSelector,
    },
  },
  models::exercise::ScaleExercise,
};
use crate::{music::notes::NoteExt, music::Note, music::ScaleType};
use leptos::prelude::*;

#[component]
pub fn ConfigurationHeader(
  exercise: Signal<ScaleExercise>,
  on_exercise_update: Callback<ScaleExercise>,
  #[prop(into)] can_edit: Signal<bool>,
) -> impl IntoView {
  // Modal states for exercise configuration
  let show_root_note_modal = RwSignal::new(false);
  let show_scale_type_modal = RwSignal::new(false);
  let show_fret_range_modal = RwSignal::new(false);

  let root_note = Signal::derive(move || exercise.get().root_note);
  let scale_type = Signal::derive(move || exercise.get().scale_type);
  let fret_range = Signal::derive(move || exercise.get().fret_range);

  view! {
    <div class="p-3 mb-6 bg-gray-50 rounded-lg dark:bg-gray-950">
      <div class="flex flex-wrap gap-4 items-center text-sm">

        <>
          <RootNoteSelection
            root_note=root_note.get()
            exercise
            show_fret_range_modal
            show_root_note_modal
            show_scale_type_modal
            on_exercise_update
            can_edit
          />
          <ScaleSelection
            scale_type=scale_type.get()
            show_fret_range_modal
            show_root_note_modal
            show_scale_type_modal
            can_edit
          />
          <FretRangeSelection
            exercise
            show_fret_range_modal
            show_root_note_modal
            show_scale_type_modal
            on_exercise_update
            active_min_fret=fret_range.get().0
            active_max_fret=fret_range.get().1
            can_edit
          />
        </>
        <div class="flex gap-2 items-center">
          <span class="font-medium">"Details:"</span>
          <span class="text-xs">{exercise.get().to_string()}</span>
        </div>

      </div>
    </div>
  }
}

#[component]
fn RootNoteSelection(
  exercise: Signal<ScaleExercise>,
  show_root_note_modal: RwSignal<bool>,
  show_scale_type_modal: RwSignal<bool>,
  show_fret_range_modal: RwSignal<bool>,
  on_exercise_update: Callback<ScaleExercise>,
  root_note: Note,
  #[prop(into)] can_edit: Signal<bool>,
) -> impl IntoView {
  // Temporary selection state for root note modal
  let temp_selected_note = RwSignal::new(None::<Note>);

  let on_confirm_note_change = move || {
    if let Some(selected_note) = temp_selected_note.get() {
      temp_selected_note.set(Some(selected_note));
      show_root_note_modal.set(false);
      let mut exercise = exercise.get();
      exercise.root_note = selected_note;
      on_exercise_update.run(exercise);
    }
  };

  view! {
    <div class="flex relative gap-2 items-center">
      <span class="font-medium">"Root:"</span>
      <Button
        variant=ButtonVariant::Primary
        disabled=Signal::derive(move || !can_edit.get())
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
                          ButtonVariant::Colored(ButtonColor::Purple)
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
            <Button
              on_click=move || {
                temp_selected_note.set(None);
                show_root_note_modal.set(false);
              }
              variant=ButtonVariant::Secondary
            >
              "Cancel"
            </Button>

            <Button
              on_click=move || { on_confirm_note_change() }
              variant=ButtonVariant::Primary
              disabled=Signal::derive(move || temp_selected_note.get().is_none() || !can_edit.get())
            >
              "OK"
            </Button>

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
  // TODO: should probably be a signal
  scale_type: ScaleType,
  #[prop(into)] can_edit: Signal<bool>,
) -> impl IntoView {
  let temporary_selected_scale = RwSignal::new(scale_type);

  view! {
    <div class="flex relative items-center">
      <span class="font-medium">"Scale:"</span>
      <Button
        variant=ButtonVariant::Colored(ButtonColor::Purple)
        disabled=Signal::derive(move || !can_edit.get())
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
                    ButtonVariant::Colored(ButtonColor::Purple)
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
              disabled=Signal::derive(move || {
                temporary_selected_scale.get() == scale_type || !can_edit.get()
              })
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
  exercise: Signal<ScaleExercise>,
  show_fret_range_modal: RwSignal<bool>,
  show_root_note_modal: RwSignal<bool>,
  show_scale_type_modal: RwSignal<bool>,
  active_min_fret: u8,
  active_max_fret: u8,
  on_exercise_update: Callback<ScaleExercise>,
  #[prop(into)] can_edit: Signal<bool>,
) -> impl IntoView {
  view! {
    <div class="flex relative gap-2 items-center">
      <span class="font-medium">"Frets:"</span>
      <Button
        disabled=Signal::derive(move || !can_edit.get())
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
          let min_fret = RwSignal::new(active_min_fret);
          let max_fret = RwSignal::new(active_max_fret);
          view! {
            <div class="absolute left-0 top-full z-10 p-2 mt-1 bg-white rounded-lg border border-gray-500 shadow-lg dark:bg-black min-w-[200px]">
              <h4 class="text-sm font-semibold">"Set Fret Range"</h4>
              <FretRangeSelector
                start_fret=min_fret
                end_fret=max_fret
                label="Set fret range"
              ></FretRangeSelector>
              // Fret range

              <div>
                <PositionPresetButtons
                  on_preset_select=move |min, max| {
                    min_fret.set(min);
                    max_fret.set(max);
                  }
                  current_range=Signal::derive(move || (min_fret.get(), max_fret.get()))
                />
              </div>
              <div class="flex justify-center items-center">
                <Button
                  disabled=Signal::derive(move || !can_edit.get())
                  variant=ButtonVariant::Primary
                  on_click=move || {
                    let mut exercise = exercise.get();
                    exercise.fret_range = (min_fret.get(), max_fret.get());
                    on_exercise_update.run(exercise);
                  }
                >
                  "Apply"
                </Button>

                <Button
                  on_click=move || show_fret_range_modal.set(false)
                  variant=ButtonVariant::Secondary
                >
                  "Close"
                </Button>
              </div>
            </div>
          }
        }
      </Show>
    </div>
  }
}
