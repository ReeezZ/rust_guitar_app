use crate::fretboard_view_helper::calculate_fret_positions;
use leptos::prelude::*;

/// Static SVG fretboard for visual layout only.
/// No interactivity, no state, just a clean guitar-like grid.
#[component]
pub fn SvgFretboard() -> impl IntoView {
  let svg_width: f64 = 800.0;
  let svg_height: f64 = 300.0;
  let num_strings = 6;
  let num_frets = 17;
  let string_spacing = svg_height / (num_strings as f64 + 1.0);
  let fret_positions = calculate_fret_positions(svg_width, num_frets);

  view! {
      <svg
          width=svg_width
          height=svg_height
          viewBox=format!("0 0 {} {}", svg_width, svg_height)
          class="fretboard-svg"
          style="background: linear-gradient(90deg, #deb887 0%, #f5deb3 100%); border-radius: 8px; box-shadow: 0 2px 8px #0002;"
      >
          // Saddle (nut)
          <rect
              x="0"
              y="0"
              width="8"
              height=svg_height
              fill="#eee"
              stroke="#bbb"
              stroke-width="2"
              rx="2"
          />

          // Frets
          { (1..=num_frets).map(|fret_no| {
              let x_pos = fret_positions[fret_no as usize];
              view! {
                  <line
                      x1=x_pos
                      y1="0"
                      x2=x_pos
                      y2=svg_height
                      stroke="#b0b0b0"
                      stroke-width="3"
                      filter="url(#metallic)"
                  />
              }
          }).collect_view() }

          // Strings
          { (0..num_strings).map(|i| {
              let y_pos = (i as f64 + 1.0) * string_spacing;
              let string_thickness = 1.0 + (i as f64 * 0.3);
              view! {
                  <line
                      x1="0"
                      y1=y_pos
                      x2=svg_width
                      y2=y_pos
                      stroke="#888"
                      stroke-width=string_thickness
                  />
              }
          }).collect_view() }

          // Fret marker dots (dynamic, with double dots at 12 and 24)
          { (1..=num_frets).filter(|&fret| [3,5,7,9,12,15,17,19,21,24].contains(&fret)).map(|fret| {
              let x = (fret_positions[(fret-1) as usize] + fret_positions[fret as usize]) / 2.0;
              let y = svg_height / 2.0;
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
          }).collect_view() }

          // Optional: SVG filter for metallic look (not all browsers support)
          <defs>
              <filter id="metallic">
                  <feSpecularLighting result="specOut" specularExponent="20" lighting-color="#fff">
                      <fePointLight x="-5000" y="-10000" z="20000"/>
                  </feSpecularLighting>
                  <feComposite in="SourceGraphic" in2="specOut" operator="arithmetic" k1="0" k2="1" k3="1" k4="0"/>
              </filter>
          </defs>
      </svg>
  }
}
