use num::{pow, BigInt};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        let message = &args[i];
        let cipher = crypt(message);
        println!("{}", cipher);
    }
    //println!("{}", decrypt(cipher));
}

fn crypt(message: &str) -> BigInt {
    let primes: [u32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    message
        .bytes()
        .enumerate()
        .map(|(i, b)| {
            let exp = usize::from(b - b'a' + 1);
            let base = BigInt::from(primes[i]);
            pow(base, exp)
        })
        .product()
}

fn decrypt(cipher: BigInt) -> String {
    String::new()
}
