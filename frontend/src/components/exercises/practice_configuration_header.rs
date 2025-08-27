use crate::models::Exercise;
use leptos::prelude::*;
use shared::{music::notes::NoteExt, ExerciseType, Note, ScaleType};

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
    <div class="p-3 mb-6 bg-gray-50 rounded-lg">
      <div class="flex flex-wrap gap-4 items-center text-sm">
        <div class="flex gap-2 items-center">
          <span class="font-medium text-gray-700">"Type:"</span>
          <span class="py-1 px-2 text-xs font-medium text-blue-800 bg-blue-100 rounded">
            {exercise.get().exercise_type.type_name()}
          </span>
        </div>

        {match exercise.get().exercise_type {
          ExerciseType::Scale { root_note, scale_type, .. }
          | ExerciseType::Triad { root_note, scale_type, .. } => {
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
                  on_exercise_update
                />
              </>
            }
              .into_any()
          }
          _ => {

            view! { <div></div> }
              .into_any()
          }
        }}

        {exercise
          .get()
          .exercise_type
          .get_fret_range()
          .map(|(min, max)| {
            view! {
              <div class="flex relative gap-2 items-center">
                <span class="font-medium text-gray-700">"Frets:"</span>
                <button
                  class="py-1 px-2 text-xs font-medium text-orange-800 bg-orange-100 rounded transition-colors cursor-pointer hover:bg-orange-200"
                  on:click=move |_| {
                    show_root_note_modal.set(false);
                    show_scale_type_modal.set(false);
                    show_fret_range_modal.set(!show_fret_range_modal.get());
                  }
                  title="Click to change fret range"
                >
                  {format!("{min}-{max}")}
                </button>

                // Fret range dropdown
                <Show when=move || show_fret_range_modal.get()>
                  <div class="absolute left-0 top-full z-10 p-4 mt-1 bg-white rounded-lg border border-gray-300 shadow-lg min-w-[200px]">
                    <h4 class="mb-2 text-sm font-semibold">"Set Fret Range"</h4>
                    <p class="mb-3 text-xs text-gray-600">
                      "Fret range selection - functionality coming soon"
                    </p>
                    <button
                      class="py-1 px-3 text-xs text-gray-600 bg-gray-200 rounded hover:bg-gray-300"
                      on:click=move |_| show_fret_range_modal.set(false)
                    >
                      "Close"
                    </button>
                  </div>
                </Show>
              </div>
            }
          })}

        <div class="flex gap-2 items-center">
          <span class="font-medium text-gray-700">"Details:"</span>
          <span class="text-xs text-gray-600">{exercise.get().exercise_type.to_string()}</span>
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
      exercise.exercise_type.set_root_note(selected_note);
      on_exercise_update.run(exercise);
    }
  };

  view! {
    <div class="flex relative gap-2 items-center">
      <span class="font-medium text-gray-700">"Root:"</span>
      <button
        class="py-1 px-2 text-xs font-medium text-indigo-800 bg-indigo-100 rounded transition-colors cursor-pointer hover:bg-indigo-200"
        on:click=move |_| {
          show_scale_type_modal.set(false);
          show_fret_range_modal.set(false);
          temp_selected_note.set(None);
          show_root_note_modal.set(!show_root_note_modal.get());
        }
        title="Click to change root note"
      >
        {root_note.to_string()}
      </button>

      // Root note dropdown
      <Show when=move || show_root_note_modal.get()>
        <div class="absolute left-1/2 top-full z-10 mt-1 w-32 bg-white rounded-lg border border-gray-300 shadow-lg transform -translate-x-1/2">
          <h4 class="mb-1 text-xs font-semibold text-center">"Root Note"</h4>
          <div class="flex flex-col">
            {move || {
              Note::all_notes()
                .iter()
                .map(|&note| {
                  let note_str = note.to_short_string();
                  let is_root_note = note == root_note;
                  let is_current_root = note == temp_selected_note.get().unwrap_or(root_note);

                  view! {
                    <button
                      class=if is_current_root {
                        "my-1 text-xs font-bold rounded border-2 border-indigo-600 bg-indigo-600 text-white transition-colors"
                      } else if is_root_note {
                        "my-1 text-xs font-bold rounded border-2 border-green-600 bg-green-600 text-white transition-colors"
                      } else {
                        "my-1 text-xs font-medium rounded border border-gray-300 bg-gray-100 text-gray-700 hover:bg-gray-200 transition-colors"
                      }
                      on:click=move |_| {
                        temp_selected_note.set(Some(note));
                      }
                    >
                      {note_str}
                    </button>
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
              class="px-1 my-1 text-sm text-white bg-blue-600 rounded transition-colors hover:bg-blue-700 disabled:bg-gray-400"
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
  on_exercise_update: Callback<Exercise>,
  scale_type: ScaleType,
) -> impl IntoView {
  view! {
    <div class="flex relative gap-2 items-center">
      <span class="font-medium text-gray-700">"Scale:"</span>
      <button
        class="py-1 px-2 text-xs font-medium text-purple-800 bg-purple-100 rounded transition-colors cursor-pointer hover:bg-purple-200"
        on:click=move |_| {
          show_root_note_modal.set(false);
          show_fret_range_modal.set(false);
          show_scale_type_modal.set(!show_scale_type_modal.get());
        }
        title="Click to change scale type"
      >
        {scale_type.to_string()}
      </button>

      // Scale type dropdown
      <Show when=move || show_scale_type_modal.get()>
        <div class="absolute left-0 top-full z-10 p-4 mt-1 bg-white rounded-lg border border-gray-300 shadow-lg min-w-[200px]">
          <h4 class="mb-2 text-sm font-semibold">"Select Scale Type"</h4>
          <p class="mb-3 text-xs text-gray-600">
            "Scale type selection - functionality coming soon"
          </p>
          <button
            class="py-1 px-3 text-xs text-gray-600 bg-gray-200 rounded hover:bg-gray-300"
            on:click=move |_| show_scale_type_modal.set(false)
          >
            "Close"
          </button>
        </div>
      </Show>
    </div>
  }
}
