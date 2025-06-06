use leptos::prelude::*;

#[component]
pub fn SvgFretboard() -> impl IntoView {
  view! {
    <svg
      // Beispielbreite
      width="800"
      // Beispielhöhe
      height="300"
      // Wichtig für Skalierbarkeit
      viewBox="0 0 800 300"
      // Für CSS-Styling
      class="fretboard-svg"
    >
      {move || {
        let num_strings = 6;
        let fretboard_height = 300.0;
        let string_spacing = fretboard_height / (num_strings as f64 + 1.0);
        (0..num_strings)
          .map(|i| {
            let y_pos = (i as f64 + 1.0) * string_spacing;
            // Oder aus deiner FretboardModel
            // Oder aus deiner FretboardModel
            // Höhe deines viewBox
            // Gleichmäßige Verteilung

            // Y-Position jeder Saite
            view! {
              <line
                // Start bei X=0 (Sattel)
                x1="0"
                // Y-Position der Saite
                y1=y_pos
                // Ende bei X=800 (volle Breite)
                x2="800"
                y2=y_pos
                stroke="silver"
                // Spätere Saiten dicker machen
                stroke-width="1"
              />
            }
          })
          .collect_view()
      }}
    // Höhe deines viewBox
    // Gleichmäßige Verteilung

    // Y-Position jeder Saite
    // Start bei X=0 (Sattel)
    // Y-Position der Saite
    // Ende bei X=800 (volle Breite)
    // Spätere Saiten dicker machen
    // Höhe deines viewBox
    // Gleichmäßige Verteilung

    // Y-Position jeder Saite
    // Start bei X=0 (Sattel)
    // Y-Position der Saite
    // Ende bei X=800 (volle Breite)
    // Spätere Saiten dicker machen
    </svg>
  }
}
