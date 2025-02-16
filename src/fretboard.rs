use leptos::prelude::*;

#[component]
pub fn Fretboard(
    #[prop(default = 6)] num_strings: u8,
    #[prop(default = 15)] num_frets: u8,
) -> impl IntoView {
    view! {
        <div class="flex-col my-12 w-full h-96 bg-amber-500 p-4">
            {(0..num_strings)
                .map(|string_no| {
                    view! {
                        <div class="flex relative items-center">
                            // Nut (Fret 0) - thicker border
                            <div class="content-center w-20 h-12 text-center border-l-8 border-slate-900 bg-gray-800 text-white font-bold flex items-center justify-center">
                                {string_no}- 0
                            </div>
                            
                            // String - positioned behind the frets
                            <div class="absolute left-0 top-1/2 h-1 w-full bg-slate-300"></div>

                            // Frets
                            {(1..=num_frets)
                                .map(|fret_no| {
                                    view! {
                                        <div class="relative content-center w-20 h-12 text-center border-r-4 border-slate-700 flex items-center justify-center">
                                            {string_no}- {fret_no}
                                        </div>
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