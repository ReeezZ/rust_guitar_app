use leptos::prelude::*;

use crate::components::ui::{button::ButtonColor, Button, ButtonVariant};

#[component]
pub fn PositionPresetButtons(
  #[prop(into)] on_preset_select: Callback<(u8, u8)>,
  #[prop(into)] current_range: Signal<(u8, u8)>,
) -> impl IntoView {
  view! {
    <div class="p-1 mt-2 text-center bg-gray-500 rounded-md border border-gray-400">
      <label class="justify-center text-sm font-medium text-center">"Position Presets"</label>
      <div class="flex flex-shrink gap-2 justify-center items-center">
        <PositionPresetButton
          label="R".to_string()
          range=(0, 4)
          on_preset_select=on_preset_select
          current_range
        />
        <PositionPresetButton
          label="1".to_string()
          range=(2, 6)
          on_preset_select=on_preset_select
          current_range
        />

        <PositionPresetButton
          label="2".to_string()
          range=(4, 8)
          on_preset_select=on_preset_select
          current_range
        />
        <PositionPresetButton
          label="3".to_string()
          range=(6, 10)
          on_preset_select=on_preset_select
          current_range
        />
        <PositionPresetButton
          label="4".to_string()
          range=(8, 12)
          on_preset_select=on_preset_select
          current_range
        />
      </div>
    </div>
  }
}

#[component]
fn PositionPresetButton(
  #[prop(into)] label: String,
  #[prop(into)] range: (u8, u8),
  #[prop(into)] on_preset_select: Callback<(u8, u8)>,
  #[prop(into)] current_range: Signal<(u8, u8)>,
) -> impl IntoView {
  view! {
    <Button
      variant=Signal::derive(move || {
        if range == current_range.get() {
          ButtonVariant::Colored(ButtonColor::Purple)
        } else {
          ButtonVariant::Secondary
        }
      })
      on_click=Callback::new(move |_| on_preset_select.run(range))
    >
      {label.clone()}
    </Button>
  }
}
