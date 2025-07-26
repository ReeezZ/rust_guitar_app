/// Important: The `scale_length` is the length of the vibrating string (e.g., 648mm for a Stratocaster).
/// This length must be converted into the coordinates of your `viewBox`.
/// For example, if your `viewBox` is 800 units wide and you want to display 24 frets,
/// then the `scale_length` must be scaled accordingly so that it fits the width of your SVG.
pub fn calculate_fret_positions(scale_length: f64, num_frets: u8) -> Vec<f64> {
  let mut positions = Vec::with_capacity(num_frets as usize + 1); // +1 for saddle
  let twelfth_root_of_2 = 2.0_f64.powf(1.0 / 12.0);

  // Position of saddle (fret 0) is 0
  positions.push(0.0);

  for n in 1..=num_frets {
    let position = scale_length * (1.0 - (1.0 / twelfth_root_of_2).powi(n as i32));
    positions.push(position);
  }
  positions
}
