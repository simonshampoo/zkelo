use elo::Pairing;
use secrets::SecretVec;
use std::env;
/*

I assume that players within a range of +-100 of each other is fair rating.

Maybe we could introduce custom ranges to indicate fair pairings but I think 100 is sufficient.
*/


/*


We have these two ELO ratings, A, and B where they are represented by SecretVec<u8>, 
the ratings as bytes. 

What we can do is take the two values from the commandline, hide them from the program

Then we can do a rangeproof on |A-B| <= 100 via bulletproofs to prove that we have a fair paring
without the program knowing exactly what the values are 

It is kinda bad that the user needs to know both of the ELO's since they pass it in via cmdline args
but it can, and probably will be, changed in the future. 
*/
fn main() {
    // bro wtf is this shit? it works but i dont like it.
    let elo_a = SecretVec::from(unsafe { env::args().nth(1).unwrap().as_bytes_mut() });

    let elo_b = SecretVec::from(unsafe { env::args().nth(2).unwrap().as_bytes_mut() });

    let p = Pairing { elo_a, elo_b };
    println!("{:?}, {:?}", &p.elo_a, &p.elo_b);
}
