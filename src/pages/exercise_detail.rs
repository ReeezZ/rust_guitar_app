use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::models::exercise::Exercise;

#[component]
pub fn ExerciseDetailPage() -> impl IntoView {
    let params = use_params_map();
    let exercise_id = move || {
        params.get().get("id")
            .map(|s| s.clone())
            .unwrap_or_else(|| String::new())
    };
    
    // For now, we'll create a mock exercise since we don't have load_exercise_by_id yet
    // TODO: Implement proper exercise loading from storage
    let exercise = Memo::new(move |_| {
        let id = exercise_id();
        if id.is_empty() {
            return None;
        }
        
        // Mock exercise for now - in real implementation we'd load from storage
        Some(Exercise::new(
            "C Major Scale Practice".to_string(),
            crate::models::exercise::ExerciseType::Scale {
                root_note: crate::music::notes::Note::C,
                scale_type: crate::music::scales::ScaleType::Hepatonic(
                    crate::music::heptatonic_scales::HeptaScaleType::Major
                ),
                fret_range: (0, 12),
            }
        ))
    });

    view! {
        <div class="max-w-4xl mx-auto p-6">
            <div class="mb-6">
                <a href="/exercises" class="text-blue-500 hover:text-blue-700 text-sm">
                    "← Back to Exercises"
                </a>
            </div>

            {move || match exercise.get() {
                Some(ex) => view! {
                    <div class="bg-white border border-gray-200 rounded-lg p-6 shadow-sm">
                        <div class="mb-6">
                            <h1 class="text-3xl font-bold text-gray-800 mb-2">{ex.name.clone()}</h1>
                            <div class="flex items-center space-x-4 text-sm text-gray-600">
                                <span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-full">
                                    {ex.exercise_type.type_name()}
                                </span>
                                <span>"ID: " {ex.id.clone()}</span>
                            </div>
                        </div>

                        <div class="space-y-6">
                            // Exercise Details Section
                            <div>
                                <h2 class="text-xl font-semibold text-gray-800 mb-3">"Exercise Details"</h2>
                                <div class="bg-gray-50 p-4 rounded-lg">
                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                        <div>
                                            <label class="block text-sm font-medium text-gray-700">"Type"</label>
                                            <p class="text-gray-900">{ex.exercise_type.type_name()}</p>
                                        </div>
                                        
                                        {move || match &ex.exercise_type {
                                            crate::models::exercise::ExerciseType::Scale { root_note, scale_type, fret_range } |
                                            crate::models::exercise::ExerciseType::Triad { root_note, scale_type, fret_range } => {
                                                view! {
                                                    <>
                                                        <div>
                                                            <label class="block text-sm font-medium text-gray-700">"Key"</label>
                                                            <p class="text-gray-900">{format!("{} {}", root_note, scale_type.to_string())}</p>
                                                        </div>
                                                        <div>
                                                            <label class="block text-sm font-medium text-gray-700">"Fret Range"</label>
                                                            <p class="text-gray-900">{format!("Frets {}-{}", fret_range.0, fret_range.1)}</p>
                                                        </div>
                                                    </>
                                                }.into_any()
                                            },
                                            _ => view! {
                                                <div class="md:col-span-2">
                                                    <label class="block text-sm font-medium text-gray-700">"Notes"</label>
                                                    <p class="text-gray-600 italic">"No specific key or fret range for this exercise type"</p>
                                                </div>
                                            }.into_any()
                                        }}
                                        
                                        {ex.description.as_ref().map(|desc| view! {
                                            <div class="md:col-span-2">
                                                <label class="block text-sm font-medium text-gray-700">"Description"</label>
                                                <p class="text-gray-900">{desc.clone()}</p>
                                            </div>
                                        })}
                                    </div>
                                </div>
                            </div>

                            // Practice Section (placeholder for future timer/metronome)
                            <div>
                                <h2 class="text-xl font-semibold text-gray-800 mb-3">"Practice Session"</h2>
                                <div class="bg-gray-50 p-6 rounded-lg text-center">
                                    <p class="text-gray-600 mb-4">"Practice timer and metronome will be added here"</p>
                                    <div class="space-y-2">
                                        <button 
                                            class="bg-green-500 hover:bg-green-700 text-white font-bold py-3 px-6 rounded-lg disabled:bg-gray-400"
                                            disabled=true
                                        >
                                            "Start Practice Session (Coming Soon)"
                                        </button>
                                        <p class="text-sm text-gray-500">"Timer, metronome, and session logging will be implemented next"</p>
                                    </div>
                                </div>
                            </div>

                            // Actions Section
                            <div class="flex justify-between items-center pt-4 border-t border-gray-200">
                                <div>
                                    <button class="text-blue-500 hover:text-blue-700 font-medium text-sm">
                                        "Edit Exercise (Coming Soon)"
                                    </button>
                                </div>
                                <div>
                                    <a href="/exercises" class="bg-gray-500 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded">
                                        "Back to List"
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                }.into_any(),
                None => view! {
                    <div class="text-center py-12">
                        <h1 class="text-2xl font-bold text-gray-800 mb-4">"Exercise Not Found"</h1>
                        <p class="text-gray-600 mb-6">"The requested exercise could not be found."</p>
                        <a href="/exercises" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                            "Back to Exercises"
                        </a>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
