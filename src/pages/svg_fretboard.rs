use crate::components::svg_fretboard::SvgFretboard;
use leptos::{ev, prelude::*};

/// Extracts the value from an input event.
/// See: https://leptos.dev/docs/reference/events/
fn event_target_value(ev: &ev::Event) -> String {
    use leptos::wasm_bindgen::JsCast;
    let target = ev.target().unwrap();
    let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
    input.value()
}

/// Page for the SVG fretboard with a runtime-adjustable fret count slider.
/// See: https://leptos.dev/docs/reference/signals/
#[component]
pub fn SvgFretboardPage() -> impl IntoView {
    let num_frets = RwSignal::new(17_usize);
    let fretboard_fill_ratio = RwSignal::new(1.0_f64);

    view! {
        <div style="margin-bottom: 1em;">
            <label for="fret-slider">
                "Number of frets: " {move || num_frets.get()}
            </label>
            <input
                id="fret-slider"
                type="range"
                min="5"
                max="22"
                prop:value=move || num_frets.get()
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    if let Ok(val) = val.parse::<usize>() {
                        num_frets.set(val);
                    }
                }
            />
        </div>
        <div style="margin-bottom: 1em;">
            <label for="fill-ratio-slider">
                "Fretboard Fill Ratio: " {move || format!("{:.2}", fretboard_fill_ratio.get())}
            </label>
            <input
                id="fill-ratio-slider"
                type="range"
                min="0.5"
                max="1.4"
                step="0.01"
                prop:value=move || fretboard_fill_ratio.get()
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    if let Ok(val) = val.parse::<f64>() {
                        fretboard_fill_ratio.set(val);
                    }
                }
            />
        </div>
        <SvgFretboard num_frets=num_frets.read_only().into() fretboard_fill_ratio=fretboard_fill_ratio.read_only().into() />
    }
}
