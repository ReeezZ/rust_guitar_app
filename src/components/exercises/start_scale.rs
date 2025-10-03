use leptos::prelude::*;

use crate::{
  components::{
    exercises::ScaleExerciseForm,
    ui::{button::ButtonColor, Button, ButtonVariant},
  },
  music::{heptatonic_scales::HeptaScaleType, Note, ScaleType},
};

#[component]
pub fn StartScale() -> impl IntoView {
  let show_dialog = RwSignal::new(false);
  let root_note = RwSignal::new(Note::C);
  let scale_type = RwSignal::new(ScaleType::Hepatonic(HeptaScaleType::Minor));
  let min_fret = RwSignal::new(0);
  let max_fret = RwSignal::new(12);

  view! {
    <div class="flex flex-col justify-center">
      <div class="flex justify-center mb-4">
        <Button
          variant=Signal::derive(move || {
            if show_dialog.get() {
              ButtonVariant::Colored(ButtonColor::Purple)
            } else {
              ButtonVariant::Primary
            }
          })
          on_click=move || {
            show_dialog.set(!show_dialog.get());
          }
        >
          "Start Scale Exercise"
        </Button>
      </div>
      <Show when=move || show_dialog.get()>
        <ScaleExerciseForm
          root_note=root_note.read_only()
          on_root_note_change=Callback::new(move |note| root_note.set(note))
          scale_type=scale_type.read_only()
          on_scale_type_change=Callback::new(move |scale| scale_type.set(scale))
          min_fret=min_fret
          max_fret=max_fret
        />
        <a
          class="flex justify-center items-center"
          href=move || {
            format!(
              "/exercise/scale?root_note={}&scale_type={}&min_fret={}&max_fret={}",
              root_note.get(),
              scale_type.get(),
              min_fret.get(),
              max_fret.get(),
            )
          }
        >

          <Button variant=ButtonVariant::Primary>"Start"</Button>
        </a>
      </Show>
    </div>
  }
}
