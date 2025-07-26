/// Examples showing how to use the new FretboardConfig system
/// 
/// This demonstrates the magic number elimination in SvgFretboard component.
/// All hardcoded constants have been moved to a configurable struct with
/// sensible defaults and comprehensive documentation.

use crate::components::svg_fretboard::{FretboardConfig, SvgFretboard};
use leptos::prelude::*;

#[component]
pub fn FretboardConfigExamples() -> impl IntoView {
    // Individual signals for each example
    let (start_1, end_1) = (RwSignal::new(3), RwSignal::new(7));
    let (start_2, end_2) = (RwSignal::new(3), RwSignal::new(7));
    let (start_3, end_3) = (RwSignal::new(3), RwSignal::new(7));
    let (start_4, end_4) = (RwSignal::new(3), RwSignal::new(7));
    let (start_5, end_5) = (RwSignal::new(3), RwSignal::new(7));
    let (start_6, end_6) = (RwSignal::new(3), RwSignal::new(7));

    view! {
        <div class="space-y-8">
            <h2 class="text-2xl font-bold">"Fretboard Configuration Examples"</h2>
            
            // Example 1: Default configuration (6-string guitar)
            <div class="border p-4 rounded">
                <h3 class="text-lg font-semibold mb-4">"Default Configuration (6-string guitar)"</h3>
                
                // Controls for Example 1
                <div class="flex gap-4 mb-4 p-2 bg-gray-50 rounded">
                    <div>
                        <label class="block text-sm">"Start: " {move || start_1.get()}</label>
                        <input 
                            type="range" 
                            min="0" 
                            max="20"
                            class="w-24"
                            prop:value=move || start_1.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val < end_1.get() {
                                        start_1.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                    <div>
                        <label class="block text-sm">"End: " {move || end_1.get()}</label>
                        <input 
                            type="range" 
                            min="1" 
                            max="24"
                            class="w-24"
                            prop:value=move || end_1.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val > start_1.get() {
                                        end_1.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                </div>
                
                <SvgFretboard 
                    start_fret=start_1.read_only().into() 
                    end_fret=end_1.read_only().into()
                />
            </div>

            // Example 2: Bass guitar configuration (4 strings, wider aspect ratio)
            <div class="border p-4 rounded">
                <h3 class="text-lg font-semibold mb-4">"Bass Guitar (4 strings, wider)"</h3>
                
                // Controls for Example 2
                <div class="flex gap-4 mb-4 p-2 bg-gray-50 rounded">
                    <div>
                        <label class="block text-sm">"Start: " {move || start_2.get()}</label>
                        <input 
                            type="range" 
                            min="0" 
                            max="20"
                            class="w-24"
                            prop:value=move || start_2.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val < end_2.get() {
                                        start_2.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                    <div>
                        <label class="block text-sm">"End: " {move || end_2.get()}</label>
                        <input 
                            type="range" 
                            min="1" 
                            max="24"
                            class="w-24"
                            prop:value=move || end_2.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val > start_2.get() {
                                        end_2.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                </div>
                
                <SvgFretboard 
                    start_fret=start_2.read_only().into() 
                    end_fret=end_2.read_only().into()
                    config=FretboardConfig {
                        num_strings: 4,
                        svg_aspect_ratio: 4.0, // Wider aspect ratio for bass
                        max_frets: 20, // Typically fewer frets than guitar
                        ..Default::default()
                    }
                />
            </div>

            // Example 3: 7-string extended range guitar
            <div class="border p-4 rounded">
                <h3 class="text-lg font-semibold mb-4">"7-String Extended Range Guitar"</h3>
                
                // Controls for Example 3
                <div class="flex gap-4 mb-4 p-2 bg-gray-50 rounded">
                    <div>
                        <label class="block text-sm">"Start: " {move || start_3.get()}</label>
                        <input 
                            type="range" 
                            min="0" 
                            max="20"
                            class="w-24"
                            prop:value=move || start_3.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val < end_3.get() {
                                        start_3.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                    <div>
                        <label class="block text-sm">"End: " {move || end_3.get()}</label>
                        <input 
                            type="range" 
                            min="1" 
                            max="24"
                            class="w-24"
                            prop:value=move || end_3.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val > start_3.get() {
                                        end_3.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                </div>
                
                <SvgFretboard 
                    start_fret=start_3.read_only().into() 
                    end_fret=end_3.read_only().into()
                    config=FretboardConfig {
                        num_strings: 7,
                        max_frets: 24, // Extended range often has more frets
                        extra_frets: 2, // Show more context frets
                        ..Default::default()
                    }
                />
            </div>

            // Example 4: Compact view (smaller, tighter margins)
            <div class="border p-4 rounded">
                <h3 class="text-lg font-semibold mb-4">"Compact View"</h3>
                
                // Controls for Example 4
                <div class="flex gap-4 mb-4 p-2 bg-gray-50 rounded">
                    <div>
                        <label class="block text-sm">"Start: " {move || start_4.get()}</label>
                        <input 
                            type="range" 
                            min="0" 
                            max="20"
                            class="w-24"
                            prop:value=move || start_4.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val < end_4.get() {
                                        start_4.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                    <div>
                        <label class="block text-sm">"End: " {move || end_4.get()}</label>
                        <input 
                            type="range" 
                            min="1" 
                            max="24"
                            class="w-24"
                            prop:value=move || end_4.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val > start_4.get() {
                                        end_4.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                </div>
                
                <SvgFretboard 
                    start_fret=start_4.read_only().into() 
                    end_fret=end_4.read_only().into()
                    config=FretboardConfig {
                        svg_width_ratio: 0.6, // Smaller width
                        svg_aspect_ratio: 2.5, // Taller/more compact
                        fret_margin_percentage: 0.02, // Tighter margins
                        nut_width: 10.0, // Thinner nut
                        ..Default::default()
                    }
                />
            </div>

            // Example 5: Custom marker positions (pedagogical - only show octaves)
            <div class="border p-4 rounded">
                <h3 class="text-lg font-semibold mb-4">"Educational - Octave Markers Only"</h3>
                
                // Controls for Example 5
                <div class="flex gap-4 mb-4 p-2 bg-gray-50 rounded">
                    <div>
                        <label class="block text-sm">"Start: " {move || start_5.get()}</label>
                        <input 
                            type="range" 
                            min="0" 
                            max="20"
                            class="w-24"
                            prop:value=move || start_5.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val < end_5.get() {
                                        start_5.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                    <div>
                        <label class="block text-sm">"End: " {move || end_5.get()}</label>
                        <input 
                            type="range" 
                            min="1" 
                            max="24"
                            class="w-24"
                            prop:value=move || end_5.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val > start_5.get() {
                                        end_5.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                </div>
                
                <SvgFretboard 
                    start_fret=start_5.read_only().into() 
                    end_fret=end_5.read_only().into()
                    config=FretboardConfig {
                        marker_positions: vec![12, 24], // Only octave markers
                        ..Default::default()
                    }
                />
            </div>

            // Example 6: Ultra-wide view for detailed analysis
            <div class="border p-4 rounded">
                <h3 class="text-lg font-semibold mb-4">"Ultra-wide Analysis View"</h3>
                
                // Controls for Example 6
                <div class="flex gap-4 mb-4 p-2 bg-gray-50 rounded">
                    <div>
                        <label class="block text-sm">"Start: " {move || start_6.get()}</label>
                        <input 
                            type="range" 
                            min="0" 
                            max="20"
                            class="w-24"
                            prop:value=move || start_6.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val < end_6.get() {
                                        start_6.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                    <div>
                        <label class="block text-sm">"End: " {move || end_6.get()}</label>
                        <input 
                            type="range" 
                            min="1" 
                            max="24"
                            class="w-24"
                            prop:value=move || end_6.get()
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<usize>() {
                                    if val > start_6.get() {
                                        end_6.set(val);
                                    }
                                }
                            }
                        />
                    </div>
                </div>
                
                <SvgFretboard 
                    start_fret=start_6.read_only().into() 
                    end_fret=end_6.read_only().into()
                    config=FretboardConfig {
                        svg_width_ratio: 0.95, // Use almost full width
                        svg_aspect_ratio: 5.0, // Very wide
                        extra_frets: 3, // Show lots of context
                        fret_margin_percentage: 0.08, // More margin for labels
                        ..Default::default()
                    }
                />
            </div>

            // Quick comparison section
            <div class="border p-4 rounded bg-blue-50">
                <h3 class="text-lg font-semibold mb-2">"💡 Configuration Tips"</h3>
                <div class="text-sm text-gray-700">
                    <div style="margin-bottom: 0.5rem;">
                        "• " <strong>"num_strings:"</strong> " 4 (bass), 6 (standard), 7+ (extended range)"
                    </div>
                    <div style="margin-bottom: 0.5rem;">
                        "• " <strong>"svg_aspect_ratio:"</strong> " Higher = wider/more horizontal"
                    </div>
                    <div style="margin-bottom: 0.5rem;">
                        "• " <strong>"svg_width_ratio:"</strong> " 0.0-1.0, controls overall size"
                    </div>
                    <div style="margin-bottom: 0.5rem;">
                        "• " <strong>"extra_frets:"</strong> " Shows context beyond active range"
                    </div>
                    <div style="margin-bottom: 0.5rem;">
                        "• " <strong>"marker_positions:"</strong> " Customize fret dots for different teaching styles"
                    </div>
                </div>
            </div>
        </div>
    }
}

fn event_target_value(ev: &leptos::ev::Event) -> String {
    use leptos::wasm_bindgen::JsCast;
    let target = ev.target().unwrap();
    let input: web_sys::HtmlInputElement = target.dyn_into().unwrap();
    input.value()
}
