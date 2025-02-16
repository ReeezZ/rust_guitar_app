use leptos::prelude::*;

#[component]
pub fn Fretboard(
    #[prop(default = 6)] num_strings: u8,
    #[prop(default = 15)] num_frets: u8,
) -> impl IntoView {
    view! {
        <div class="flex-col mt-12 w-full h-72 bg-amber-500">
            {(0..num_strings)
                .map(|_| {
                    view! {
                        <div class="flex relative w-full h-2 before:w-full before:h-2 before:bg-gradient-to-r before:from-[#eee] to-[#999] shadow-[76px_3px_10px_#806233]">
                            {(0..num_frets)
                                .map({ |_| view! { <div class=""></div> } })
                                .collect::<Vec<_>>()}
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
