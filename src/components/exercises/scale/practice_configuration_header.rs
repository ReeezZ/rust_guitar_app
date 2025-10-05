use crate::components::{
  exercises::scale::PositionPresetButtons,
  ui::{
    button::ButtonColor,
    title::{HeadingLevel, Title},
    Button, ButtonVariant, FretRangeSelector,
  },
};
use crate::{music::notes::NoteExt, music::Note, music::ScaleType};
use leptos::prelude::*;

#[component]
pub fn ConfigurationHeader(
  #[prop(into)] root_note: Signal<Note>,
  #[prop(into)] scale_type: Signal<ScaleType>,
  #[prop(into)] min_fret: Signal<u8>,
  #[prop(into)] max_fret: Signal<u8>,
  on_root_note_changed: Callback<Note>,
  on_scale_type_changed: Callback<ScaleType>,
  on_min_fret_changed: Callback<u8>,
  on_max_fret_changed: Callback<u8>,
  #[prop(into)] can_edit: Signal<bool>,
) -> impl IntoView {
  // Modal states for exercise configuration
  let show_root_note_modal = RwSignal::new(false);
  let show_scale_type_modal = RwSignal::new(false);
  let show_fret_range_modal = RwSignal::new(false);

  view! {
    <div class="p-3 mb-6 bg-gray-50 rounded-lg dark:bg-gray-950">
      <div class="flex flex-wrap gap-4 items-center text-sm">

        <>
          <RootNoteSelection
            root_note
            on_root_note_changed
            show_fret_range_modal
            show_root_note_modal
            show_scale_type_modal
            can_edit
          />
          <ScaleSelection
            scale_type
            show_fret_range_modal
            show_root_note_modal
            show_scale_type_modal
            on_scale_type_changed
            can_edit
          />
          <FretRangeSelection
            show_fret_range_modal
            show_root_note_modal
            show_scale_type_modal
            min_fret
            on_min_fret_changed
            max_fret
            on_max_fret_changed
            can_edit
          />
        </>

      </div>
    </div>
  }
}

#[component]
pub fn RootNoteSelection(
  show_root_note_modal: RwSignal<bool>,
  show_scale_type_modal: RwSignal<bool>,
  show_fret_range_modal: RwSignal<bool>,
  on_root_note_changed: Callback<Note>,
  #[prop(into)] root_note: Signal<Note>,
  #[prop(into)] can_edit: Signal<bool>,
) -> impl IntoView {
  // Temporary selection state for root note modal
  let temp_selected_note = RwSignal::new(None::<Note>);

  let on_confirm_note_change = move || {
    if let Some(selected_note) = temp_selected_note.get() {
      show_root_note_modal.set(false);
      on_root_note_changed.run(selected_note);
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
        {move || root_note.get().to_string()}
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
                  let is_root_note = note == root_note.get();
                  let is_current_root = move || {
                    note == temp_selected_note.get().unwrap_or(root_note.get())
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
              on_click=on_confirm_note_change
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
  scale_type: Signal<ScaleType>,
  on_scale_type_changed: Callback<ScaleType>,
  #[prop(into)] can_edit: Signal<bool>,
) -> impl IntoView {
  let temporary_selected_scale = RwSignal::new(scale_type.get());

  let on_confirm = move || {
    show_scale_type_modal.set(false);
    on_scale_type_changed.run(temporary_selected_scale.get());
  };

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
        {move || scale_type.get().to_string()}
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
                  if button_scale_type == scale_type.get() {
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
                temporary_selected_scale.get() == scale_type.get() || !can_edit.get()
              })
              variant=ButtonVariant::Primary
              on_click=on_confirm
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
  show_fret_range_modal: RwSignal<bool>,
  show_root_note_modal: RwSignal<bool>,
  show_scale_type_modal: RwSignal<bool>,
  #[prop(into)] min_fret: Signal<u8>,
  #[prop(into)] on_min_fret_changed: Callback<u8>,
  #[prop(into)] max_fret: Signal<u8>,
  #[prop(into)] on_max_fret_changed: Callback<u8>,
  #[prop(into)] can_edit: Signal<bool>,
) -> impl IntoView {
  let selected_min_fret = RwSignal::new(min_fret.get());
  let selected_max_fret = RwSignal::new(max_fret.get());

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
        {move || min_fret.get()}
        {"-"}
        {move || max_fret.get()}
      </Button>

      <Show when=move || show_fret_range_modal.get()>
        <div class="absolute left-0 top-full z-10 p-2 mt-1 bg-white rounded-lg border border-gray-500 shadow-lg dark:bg-black min-w-[200px]">
          <h4 class="text-sm font-semibold">"Set Fret Range"</h4>
          <FretRangeSelector
            start_fret=selected_min_fret
            on_start_fret_changed=Callback::new(move |new_start| {
              selected_min_fret.set(new_start)
            })
            end_fret=selected_max_fret
            on_end_fret_changed=Callback::new(move |new_end| { selected_max_fret.set(new_end) })
            label="Set fret range"
          ></FretRangeSelector>

          <div>
            <PositionPresetButtons
              on_preset_select=move |min, max| {
                selected_min_fret.set(min);
                selected_max_fret.set(max);
              }
              current_range=Signal::derive(move || (
                selected_min_fret.get(),
                selected_max_fret.get(),
              ))
            />
          </div>
          <div class="flex justify-center items-center">
            <Button
              on_click=move || {
                on_min_fret_changed.run(selected_min_fret.get());
                on_max_fret_changed.run(selected_max_fret.get());
                show_fret_range_modal.set(false);
              }
              variant=ButtonVariant::Primary
              disabled=Signal::derive(move || {
                (selected_min_fret.get() == min_fret.get()
                  && selected_max_fret.get() == max_fret.get()) || !can_edit.get()
              })
            >
              "OK"
            </Button>

            <Button
              on_click=move || show_fret_range_modal.set(false)
              variant=ButtonVariant::Secondary
            >
              "Close"
            </Button>
          </div>
        </div>
      </Show>
    </div>
  }
}
