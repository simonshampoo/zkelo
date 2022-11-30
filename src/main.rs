use elo::Pairing;
use secrets::SecretVec;
use std::env;
/*

I assume that players within a range of +-100 of each other is fair rating.

Maybe we could introduce custom ranges to indicate fair pairings but I think 100 is sufficient.
*/
fn main() {
    // bro wtf is this shit? it works but i dont like it.
    let elo_a = SecretVec::from(unsafe { env::args().nth(1).unwrap().as_bytes_mut() });

    let elo_b = SecretVec::from(unsafe { env::args().nth(2).unwrap().as_bytes_mut() });

    let p = Pairing { elo_a, elo_b };
    println!("{:?}, {:?}", &p.elo_a, &p.elo_b);
}
