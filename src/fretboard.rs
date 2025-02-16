use leptos::prelude::*;

#[component]
pub fn Fretboard(
    #[prop(default = 6)] num_strings: u8,
    #[prop(default = 15)] num_frets: u8,
) -> impl IntoView {
    view! {
        <div class="fretboard bg-100">
            {(0..num_strings)
                .map(|_| {
                    view! {
                        <div class="py-2 px-3 m-1 text-white bg-blue-700 rounded border-l-2 border-b-4 border-blue-800 shadow-lg">
                            {(0..num_frets)
                                .map({ |_| view! { <div class="note-string">LUL</div> } })
                                .collect::<Vec<_>>()}
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
