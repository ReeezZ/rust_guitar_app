use crate::components::exercises::SongExerciseForm;
use crate::components::ui::{Button, ButtonVariant};
use crate::models::repository::{get_songs_repository, SongsRepository};
use leptos::prelude::*;

#[component]
pub fn SongsList() -> impl IntoView {
  let (songs, set_songs) = signal({
    let repo = get_songs_repository();
    repo.find_all().unwrap_or_default()
  });
  let (show_form, set_show_form) = signal(false);
  let (show_delete_confirmation, set_show_delete_confirmation) = signal(false);
  let (pending_delete_exercise, set_pending_delete_exercise) = signal(None::<(String, String)>);

  // Show delete confirmation dialog
  let show_delete_dialog = move |exercise_id: String, exercise_name: String| {
    set_pending_delete_exercise.set(Some((exercise_id, exercise_name)));
    set_show_delete_confirmation.set(true);
  };

  // Confirm deletion
  let confirm_delete = move || {
    if let Some((exercise_id, _)) = pending_delete_exercise.get() {
      let repo = get_songs_repository();
      let _ = repo.delete(&exercise_id); // Ignore errors for now
      set_songs.update(|exercises| exercises.retain(|e| e.id != exercise_id));
      set_show_delete_confirmation.set(false);
      set_pending_delete_exercise.set(None);
    }
  };

  // Cancel deletion
  let cancel_delete = move || {
    set_show_delete_confirmation.set(false);
    set_pending_delete_exercise.set(None);
  };

  // Handle exercise save from form
  let handle_exercise_save = Callback::new(move |_| {
    // TODO:
    // implement saving
    // set_songs.update(|exercises| {
    // Remove existing exercise if updating, then add the new one
    // songs.retain(|e| e.id != exercise.id);
    // songs.push(exercise);
    // });
    set_show_form.set(false);
  });

  // Handle form cancel
  let handle_form_cancel = Callback::new(move |_: ()| {
    set_show_form.set(false);
  });

  view! {
    <div class="p-6 mx-auto max-w-4xl">
      <div class="flex justify-between items-center mb-6">
        <h1 class="text-3xl font-bold">"My songs"</h1>
        <Button variant=ButtonVariant::Primary on_click=move || set_show_form.set(!show_form.get())>
          {move || if show_form.get() { "Cancel" } else { "Add Exercise" }}
        </Button>
      </div>

      // Add Exercise Form
      {move || {
        if show_form.get() {
          view! {
            <div class="mb-6">
              <SongExerciseForm on_save=handle_exercise_save on_cancel=handle_form_cancel />
            </div>
          }
            .into_any()
        } else {
          view! { <div></div> }.into_any()
        }
      }}

      <div class="space-y-4">
        <For each=move || songs.get() key=|song| song.id.clone() let:song>
          <div class="p-4 bg-white rounded-lg border border-gray-200 shadow-sm">
            <div class="flex justify-between items-center">
              <div>
                <h3 class="text-lg font-semibold">{song.name.clone()}</h3>
                {song
                  .description
                  .as_ref()
                  .map(|desc| view! { <p class="mt-1 text-sm text-gray-500">{desc.clone()}</p> })}
              </div>
              <div class="flex items-center space-x-2">
                <a
                  href=format!("/songs/{id}", id = song.id)
                  class="flex justify-center items-center py-1.5 px-3 text-sm font-medium text-white bg-blue-500 rounded hover:bg-blue-700"
                >
                  "View"
                </a>
                <Button
                  variant=ButtonVariant::Danger
                  on_click={
                    let exercise_id = song.id.clone();
                    let exercise_name = song.name.clone();
                    move || show_delete_dialog(exercise_id.clone(), exercise_name.clone())
                  }
                >
                  "Delete"
                </Button>
              </div>
            </div>
          </div>
        </For>

        {move || {
          songs
            .get()
            .is_empty()
            .then(|| {
              view! {
                <div class="py-8 text-center">
                  <p>"No songs yet. Create your first exercise to get started!"</p>
                </div>
              }
            })
        }}
      </div>

      // Delete confirmation dialog
      {move || {
        if show_delete_confirmation.get() {
          if let Some((_, exercise_name)) = pending_delete_exercise.get() {
            view! {
              <div class="flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-50">
                <div class="p-6 mx-4 max-w-md bg-white rounded-lg">
                  <h3 class="mb-3 text-lg font-semibold text-gray-800">Delete Exercise</h3>
                  <p class="mb-4">
                    "Are you sure you want to delete '"
                    <span class="font-semibold">{exercise_name}</span> "'?"
                  </p>
                  <p class="mb-6 text-sm">"This action cannot be undone."</p>
                  <div class="flex justify-end space-x-3">
                    <Button variant=ButtonVariant::Secondary on_click=move || cancel_delete()>
                      "Cancel"
                    </Button>
                    <Button variant=ButtonVariant::Danger on_click=move || confirm_delete()>
                      Delete
                    </Button>
                  </div>
                </div>
              </div>
            }
              .into_any()
          } else {
            view! { <div></div> }.into_any()
          }
        } else {
          view! { <div></div> }.into_any()
        }
      }}
    </div>
  }
}
