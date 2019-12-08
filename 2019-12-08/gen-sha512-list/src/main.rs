use sha2::{Digest, Sha512};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    for arg in args {
        let target = Sha512::new().chain(arg).result();
        println!("{:x}", target);
    }
}

