use leptos::{
    prelude::{ClassAttribute, ElementChild, Get, OnAttribute, RwSignal, Set},
    *,
};

// Standard-Stimmung (EADGBE, tiefste bis höchste Saite)
const STANDARD_TUNING: [&str; 6] = ["E", "A", "D", "G", "B", "E"];
// Chromatische Skala
const NOTES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

fn get_note_for_fret(string: usize, fret: usize) -> &'static str {
    let open_note = STANDARD_TUNING[string];
    let start_index = NOTES.iter().position(|&n| n == open_note).unwrap();
    NOTES[(start_index + fret) % 12]
}

#[component]
fn Fretboard() -> impl IntoView {
    let selected_note = RwSignal::new("".to_string());

    view! {
        <div class="fretboard">
            {move || {
                (0..6)
                    .rev()
                    .map(|string| {
                        view! {
                            <div class="string">
                                {move || {
                                    (0..12)
                                        .map(move |fret| {
                                            let note = get_note_for_fret(string, fret);
                                            let onclick = {
                                                let selected_note = selected_note.clone();
                                                move |_| selected_note.set(note.to_string())
                                            };
                                            view! {
                                                <button class="fret" on:click=onclick>
                                                    {note}
                                                </button>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }}
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
            }} <p>"Ausgewählte Note: " {move || selected_note.get()}</p>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h1>"Gitarren Griffbrett"</h1>
        <Fretboard />
    }
}
