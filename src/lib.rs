use std::env;

pub struct Pairing {
    elo_a: u16,
    elo_b: u16,
}

impl Pairing {
    pub fn new(elo_a: u16, elo_b: u16) -> Result<Self, &'static str> {
        Ok(Pairing { elo_a, elo_b })
    }

    pub fn is_valid_pairing() -> bool {
        todo!();
    }
}

pub fn calculate_expected_score(ra: u16, rb: u16) -> f32 {
    let x = if ra - rb > 0 { ra - rb } else { rb - ra };
    1.0 / (1.0 + f32::powf(10.0, ((x) / 400).into())) * 100.0
}

pub fn read_args() {}
