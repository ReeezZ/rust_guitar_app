use super::practice_session::PracticeSession;
use crate::components::ui::{Button, ButtonVariant};
use crate::models::exercise::{Exercise, Song};
use crate::models::repository::{get_songs_repository, SongsRepository};
use leptos::prelude::*;

#[component]
pub fn SongDetail(#[prop(into)] song_id: Signal<String>) -> impl IntoView {
  let (song, set_song) = signal(None::<Song>);

  Effect::new(move |_| {
    if !song_id.get().is_empty() {
      let repo = get_songs_repository();
      if let Ok(song) = repo.find_by_id(&song_id.get()) {
        set_song.set(Some(song));
      }
    }
  });

  view! {
    {move || match song.get() {
      Some(song) => {
        view! {
          <SongDetailChecked
            song
            on_song_change=Callback::new(move |updated_song: Song| {
              set_song.set(Some(updated_song));
            })
          />
        }
          .into_any()
      }
      None => {
        view! {
          <div class="py-16 text-center">
            <h2 class="mb-4 text-2xl font-bold text-gray-600">song Not Found</h2>
            <p class="mb-8 text-gray-500">The song you are looking for does not exist.</p>
            <a href="/songs" class="py-2 px-6 text-white bg-blue-500 rounded hover:bg-blue-600">
              "Back to songs"
            </a>
          </div>
        }
          .into_any()
      }
    }}
  }
}

#[component]
fn SongDetailChecked(
  #[prop(into)] song: Signal<Song>,
  #[prop(into)] on_song_change: Callback<Song>,
) -> impl IntoView {
  // Inline description editing state
  let (is_editing_description, set_is_editing_description) = signal(false);
  let (description_edit_value, set_description_edit_value) = signal(String::new());

  // Handle inline description editing
  let start_description_edit = move |current_description: String| {
    set_description_edit_value.set(current_description);
    set_is_editing_description.set(true);
  };

  let save_description_edit = move || {
    let new_description = description_edit_value.get().trim().to_string();
    let mut song = song.get();
    song.description = if new_description.is_empty() {
      None
    } else {
      Some(new_description)
    };

    // Update in storage
    let repo = get_songs_repository();
    if let Ok(()) = repo.update(&song) {
      set_is_editing_description.set(false);
      on_song_change.run(song);
    }
  };

  let cancel_description_edit = move || {
    set_is_editing_description.set(false);
  };

  let handle_title_change = Callback::new(move |new_title: String| {
    let mut ex = song.get_untracked();
    ex.name = new_title;
    let repo = get_songs_repository();
    if let Ok(()) = repo.update(&ex) {
      on_song_change.run(ex);
    }
  });

  view! {
    <div class="container py-8 px-4 mx-auto">
      {move || {
        view! {
          <div>
            <div class="mb-8">
              <div class="flex justify-between items-start mb-4">
                // Title section with inline editing
                <div class="flex-1 mr-4">
                  <Title song=song on_title_change=handle_title_change />
                </div>
              </div>

              // Description section with inline editing
              <div class="mb-4">
                {move || {
                  if is_editing_description.get() {
                    // Edit mode - show textarea and buttons
                    view! {
                      <div class="space-y-2">
                        <textarea
                          class="py-2 px-3 w-full rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                          prop:value=move || description_edit_value.get()
                          on:input=move |e| {
                            set_description_edit_value.set(event_target_value(&e))
                          }
                          placeholder="Enter song description (optional)"
                          rows="3"
                        />
                        <div class="flex justify-end space-x-2">
                          <Button
                            variant=ButtonVariant::Secondary
                            on_click=move || cancel_description_edit()
                          >
                            "Cancel"
                          </Button>
                          <Button
                            variant=ButtonVariant::Primary
                            on_click=move || save_description_edit()
                          >
                            "Save"
                          </Button>
                        </div>
                      </div>
                    }
                      .into_any()
                  } else {
                    match song.get().description.as_ref() {
                      Some(desc) => {
                        let desc_for_edit = desc.clone();
                        // Display mode - show description or placeholder with edit button
                        view! {
                          <div
                            class="relative p-3 rounded-lg border-2 border-transparent transition-all duration-200 cursor-pointer hover:bg-gray-50 hover:border-gray-300 group"
                            on:click=move |_| start_description_edit(desc_for_edit.clone())
                            title="Click to edit description"
                          >
                            <p class="pr-8 text-gray-600">{desc.clone()}</p>
                            <div class="absolute top-2 right-2 text-gray-400 transition-colors duration-200 group-hover:text-gray-600">
                              <svg
                                class="w-4 h-4"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                              >
                                <path
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                  stroke-width="2"
                                  d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
                                />
                              </svg>
                            </div>
                          </div>
                        }
                          .into_any()
                      }
                      None => {
                        view! {
                          <div
                            class="p-3 rounded-lg border-2 border-gray-300 border-dashed transition-all duration-200 cursor-pointer hover:bg-gray-50 hover:border-gray-400"
                            on:click=move |_| start_description_edit(String::new())
                            title="Click to add description"
                          >
                            <p class="italic text-gray-400 transition-colors duration-200 hover:text-gray-600">
                              "+ Add description"
                            </p>
                          </div>
                        }
                          .into_any()
                      }
                    }
                  }
                }}
              </div>

            // Colored configuration buttons with dropdowns - REMOVED
            // Now handled by the PracticeSession component
            </div>

            // Practice Session Section
            <div class="p-6 bg-gray-50 rounded-lg">
              <PracticeSession
                target_time=std::time::Duration::from_secs(15 * 60)
                exercise=Signal::derive(move || Exercise::Song(song.get()))
                // TODO: yeah we should split this....
                on_exercise_update=Callback::new(move |updated_exercise: Exercise| {
                  match updated_exercise {
                    Exercise::Song(updated_song) => {
                      let repo = get_songs_repository();
                      if let Err(e) = repo.update(&updated_song) {
                        leptos::logging::error!("Failed to update song: {:?}", e);
                      } else {
                        on_song_change.run(updated_song);
                      }
                    }
                    _ => leptos::logging::warn!("Received unexpected exercise type in SongDetail"),
                  }
                })
              />
            </div>
          </div>
        }
      }}
    </div>
  }
}

#[component]
fn Title(
  #[prop(into)] song: Signal<Song>,
  #[prop(into)] on_title_change: Callback<String>,
) -> impl IntoView {
  // Inline title editing state
  let (is_editing_title, set_is_editing_title) = signal(false);
  let (title_edit_value, set_title_edit_value) = signal(String::new());

  // Generate title from song type
  // Handle inline title editing
  let start_title_edit = move |current_title: String| {
    set_title_edit_value.set(current_title);
    set_is_editing_title.set(true);
  };

  let save_title_edit = move || {
    let new_title = title_edit_value.get().trim().to_string();
    if !new_title.is_empty() {
      on_title_change.run(new_title.clone());
    }
  };

  let cancel_title_edit = move || {
    set_is_editing_title.set(false);
  };

  view! {
    {move || {
      if is_editing_title.get() {
        view! {
          <div class="space-y-2">
            <input
              type="text"
              class="w-full text-3xl font-bold bg-transparent border-b-2 border-blue-500 focus:outline-none"
              prop:value=title_edit_value
              on:input=move |e| { set_title_edit_value.set(event_target_value(&e)) }
              placeholder="Enter exercise title"
            />
            <div class="flex justify-between items-center">
              <div class="flex space-x-2">
                <Button variant=ButtonVariant::Secondary on_click=move || cancel_title_edit()>
                  "Cancel"
                </Button>
                <Button variant=ButtonVariant::Primary on_click=move || save_title_edit()>
                  "Save"
                </Button>
              </div>
            </div>
          </div>
        }
          .into_any()
      } else {
        view! {
          <h1
            class="relative text-3xl font-bold transition-colors duration-200 cursor-pointer hover:text-blue-600 group"
            on:click=move |_| start_title_edit(song.get_untracked().name)
            title="Click to edit title"
          >
            {move || song.get().name}
            <svg
              class="inline-block ml-2 w-5 h-5 text-gray-400 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"
              />
            </svg>
          </h1>
        }
          .into_any()
      }
    }}
  }
}
