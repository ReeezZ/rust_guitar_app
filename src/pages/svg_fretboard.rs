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
    let fret_stretch_factor = RwSignal::new(1.5_f64);

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
            <label for="stretch-slider">
                "Fret Stretch: " {move || format!("{:.2}", fret_stretch_factor.get())}
            </label>
            <input
                id="stretch-slider"
                type="range"
                min="1.0"
                max="2.0"
                step="0.01"
                prop:value=move || fret_stretch_factor.get()
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    if let Ok(val) = val.parse::<f64>() {
                        fret_stretch_factor.set(val);
                    }
                }
            />
        </div>
        <SvgFretboard num_frets=num_frets.read_only().into() fret_stretch_factor=fret_stretch_factor.read_only().into() />
    }
}
