#[macro_use]
extern crate itertools;
use sha2::{Digest, Sha512};

fn main() {
    let target = Sha512::new().chain(b"help").result();
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    let col = iproduct!(alpha.chars(), alpha.chars(), alpha.chars(), alpha.chars())
        .map(|(a, b, c, d)| format!("{}{}{}{}", a, b, c, d))
        .filter(|candidate| Sha512::new().chain(&candidate).result() == target)
        .collect::<Vec<String>>();
    println!("{:?}", col);
}
