use leptos::prelude::*;

#[component]
pub fn Fretboard(
    #[prop(default = 6)] num_strings: u8,
    #[prop(default = 15)] num_frets: u8,
) -> impl IntoView {
    view! {
        <div class="flex-col my-12 w-full h-96 bg-amber-500">
            {(0..num_strings)
                .map(|string_no| {
                    view! {
                        <div class="flex relative content-center after:absolute after:left-0 after:top-1/2 after:h-1 after:bg-slate-300 after:w-full after:block">
                            {(0..num_frets)
                                .map({
                                    |fret_no| {
                                        view! {
                                            <div class="content-center w-20 h-12 text-center border-r-4 border-slate-700">
                                                {string_no}- {fret_no}
                                            </div>
                                        }
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
