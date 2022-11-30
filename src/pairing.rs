struct Pairing {
    elo_a: u16,
    elo_b: u16,
}

impl Pairing {
    fn new(elo_a: u16, elo_b: u16) -> Self {
        if (elo_a < 0 || elo_b < 0) {
            panic!("elo cannot be negative");
        }
        Pairing { elo_a, elo_b }
    }
}
