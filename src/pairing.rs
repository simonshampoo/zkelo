pub struct Pairing {
    elo_a: u16,
    elo_b: u16,
}

impl Pairing {
    pub fn new(elo_a: u16, elo_b: u16) -> Result<Self, &'static str> {
        if (elo_a < 0 || elo_b < 0) {
            return Err("Elo cannot be negative");
        }
        Ok(Pairing { elo_a, elo_b })
    }
}
