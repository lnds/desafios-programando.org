use itertools::Itertools;
use num::{pow, BigInt};
use num_traits::identities::{One, Zero};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for message in args.iter().skip(1) {
        let cipher = crypt(message);
        println!("{}", cipher);
        let decipher = decrypt(cipher);
        println!("{}", decipher);
    }
}

const PRIMES: [u32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

fn crypt(message: &str) -> BigInt {
    message
        .bytes()
        .enumerate()
        .map(|(i, b)| {
            let exp = usize::from(b - b'a' + 1);
            let base = BigInt::from(PRIMES[i]);
            pow(base, exp)
        })
        .product()
}

fn decrypt(cipher: BigInt) -> String {
    let letters: Vec<usize> = factors(&cipher)
        .iter()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, g)| g.count())
        .collect();
    letters
        .iter()
        .map(|&u| char::from(b'a' + u as u8 - 1))
        .collect::<String>()
}

pub fn factors(n: &BigInt) -> Vec<BigInt> {
    if n.is_zero() || n.is_one() {
        vec![]
    } else {
        let f: Vec<BigInt> = num_iter::range(BigInt::from(2u32), n.clone())
            .filter(|i| (n % i).is_zero())
            .take(1)
            .collect();
        if f.is_empty() {
            vec![n.clone()]
        } else {
            [&f[..], &factors(&(n / f[0].clone()))].concat()
        }
    }
}
