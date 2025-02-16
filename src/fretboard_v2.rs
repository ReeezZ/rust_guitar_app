use leptos::prelude::*;

#[component]
pub fn FretboardV2(
    #[prop(default = 6)] num_strings: u8,
    #[prop(default = 15)] num_frets: u8,
) -> impl IntoView {
    view! {
        <div class="fretboard">
            {(0..num_strings)
                .map(|_| {
                    view! {
                        <div class="note">
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
