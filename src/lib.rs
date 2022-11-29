pub fn calculate_expected_score(ra: u16, rb: u16) -> f32 {
    let x = if ra - rb > 0 { ra - rb } else { rb - ra };
    1.0 / (1.0 + f32::powf(10.0, ((x) / 400).into())) * 100.0
}
