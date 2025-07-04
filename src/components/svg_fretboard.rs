use crate::fretboard_view_helper::calculate_fret_positions;
use leptos::prelude::*;
use leptos_use::{use_window_size, UseWindowSizeReturn};

/// A responsive SVG fretboard that adjusts to the window size.
#[component]
pub fn SvgFretboard(num_frets: Signal<usize>, fretboard_fill_ratio: Signal<f64>) -> impl IntoView {
    // Constants for SVG dimensions and scaling
    const SVG_WIDTH_RATIO: f64 = 0.9; // 90% of window width
    const SVG_ASPECT_RATIO: f64 = 3.0; // width / height
    const FRET_MARGIN_PERCENTAGE: f64 = 0.05; // 5% of svg_height
    const NUT_WIDTH: f64 = 14.0;

    // Get reactive window dimensions.
    let UseWindowSizeReturn { width, height: _ } = use_window_size();

    // Calculate SVG dimensions based on window size, maintaining an aspect ratio.
    let svg_width = Memo::new(move |_| width.get() * SVG_WIDTH_RATIO);
    let svg_height = Memo::new(move |_| svg_width.get() / SVG_ASPECT_RATIO);

    let num_strings = 6;
    let fret_margin = Memo::new(move |_| svg_height.get() * FRET_MARGIN_PERCENTAGE);

    view! {
        <svg
            width=move || svg_width.get()
            height=move || svg_height.get()
            viewBox=move || format!("0 0 {} {}", svg_width.get(), svg_height.get())
            class="fretboard-svg"
            style="background: linear-gradient(90deg, #deb887 0%, #f5deb3 100%); border-radius: 8px; box-shadow: 0 2px 8px #0002; border: 1px solid #c00;"
        >
            {move || {
                let current_svg_width = svg_width.get();
                let current_svg_height = svg_height.get();
                let current_fret_margin = fret_margin.get();
                let string_spacing = current_svg_height / (num_strings as f64 + 1.0);

                let num_frets_val = num_frets.get();

                let playable_width = (current_svg_width - NUT_WIDTH) * fretboard_fill_ratio.get();
                let fret_positions = calculate_fret_positions(playable_width, num_frets_val as u8);

                // Saddle (nut) - visually distinct and as tall as fret area
                let nut = view! {
                    <rect
                        x="0"
                        y=current_fret_margin
                        width=NUT_WIDTH
                        height=current_svg_height - 2.0 * current_fret_margin
                        fill="#f8f8f8"
                        stroke="#222"
                        stroke-width="5"
                        rx="3"
                    />
                };

                // Frets
                let frets = (1..=num_frets_val).map(|fret_no| {
                    let x_pos_relative_to_nut = fret_positions[fret_no as usize];
                    let x_pos = x_pos_relative_to_nut + NUT_WIDTH;
                    view! {
                        <line
                            x1=x_pos
                            y1=current_fret_margin
                            x2=x_pos
                            y2=current_svg_height - current_fret_margin
                            stroke="#444"
                            stroke-width="5"
                        />
                    }
                }).collect_view();

                // Strings
                let strings = (0..num_strings).map(|i| {
                    let y_pos = (i as f64 + 1.0) * string_spacing;
                    let string_thickness = 1.0 + (i as f64);
                    view! {
                        <line
                            x1="0"
                            y1=y_pos
                            x2=current_svg_width
                            y2=y_pos
                            stroke="#888"
                            stroke-width=string_thickness
                        />
                    }
                }).collect_view();

                // Fret marker dots
                let markers = (1..=num_frets_val).filter(|&fret| [3,5,7,9,12,15,17,19,21,24].contains(&fret)).map(|fret| {
                    let x_prev_fret_relative = fret_positions[(fret-1) as usize];
                    let x_curr_fret_relative = fret_positions[fret as usize];
                    let x_relative = (x_prev_fret_relative + x_curr_fret_relative) / 2.0;
                    let x = x_relative + NUT_WIDTH; // Offset by NUT_WIDTH
                    let y = current_svg_height / 2.0;
                    let r = if fret == 12 || fret == 24 { 8.0 } else { 6.0 };
                    let y_offset = 28.0;

                    if fret == 12 || fret == 24 {
                        view! {
                            <g>
                                <circle cx=x cy=y-y_offset r=r fill="#444" opacity="0.25" />
                                <circle cx=x cy=y+y_offset r=r fill="#444" opacity="0.25" />
                            </g>
                        }
                    } else {
                        view! {
                            <g>
                                <circle cx=x cy=y r=r fill="#444" opacity="0.25" />
                                <circle cx=x cy=y+y_offset r=r fill="#444" opacity="0" />
                            </g>
                        }
                    }
                }).collect_view();

                // Simple visual cue for guitar body
                let body = {
                    let last_fret_x = fret_positions[num_frets_val as usize] + NUT_WIDTH;
                    let body_x = last_fret_x;
                    let body_width = current_svg_width - body_x;
                    view! {
                        <rect
                            x=body_x
                            y=current_fret_margin
                            width=body_width
                            height=current_svg_height - 2.0 * current_fret_margin
                            fill="#a67c52"
                            opacity="0.25"
                            rx="18"
                        />
                    }
                };

                view! {
                    {nut}
                    {frets}
                    {strings}
                    {markers}
                    {body}
                }
            }}
        </svg>
    }
}
