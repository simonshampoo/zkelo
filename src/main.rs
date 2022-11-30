use secrets::Secret;
use std::env;
/*

I assume that players within a range of +-100 of each other is fair rating.

Maybe we could introduce custom ranges to indicate fair pairings but I think 100 is sufficient.
*/
fn main() {
    let elo_a = Secret::<[u16; 16]>::new(|mut elo_a| {
        elo_a = std::env::args().nth(1).expect("no elo given");
    });
    let elo_b = Secret::<[u8; 16]>::new(|mut elo_a| {
        elo_a = std::env::args().nth(1).expect("no elo given");
    });
}
