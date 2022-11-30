use secrets::Secret;
/*

I assume that players within a range of +-100 of each other is fair rating.

Maybe we could introduce custom ranges to indicate fair pairings but I think 100 is sufficient.
*/
fn main() {
    Secret::<[u8; 16]>::random(|s| {
        println!("{:?}", s);
    })
}
