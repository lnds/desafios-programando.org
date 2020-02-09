use std::num::Wrapping;

fn main() {
    if let Some((num, c, n)) = (100..=1000)
        .flat_map(|i| encontrar_capicua(Wrapping(i as u64), 0))
        .max_by(|(_, _, a), (_, _, b)| a.cmp(b))
    {
        println!("num = {}, su capicua es {}, iteraciones: {}", num, c, n);
    }
}

const LIMIT: u64 = 1_000_000;

fn encontrar_capicua(num: Wrapping<u64>, n: u64) -> Option<(Wrapping<u64>, Wrapping<u64>, u64)> {
    let rev = reverse(num);
    let sum = num + rev;
    if n > LIMIT {
        None
    } else if sum == reverse(sum) {
        Some((num, sum, n + 1))
    } else {
        encontrar_capicua(sum, n + 1).map(|(_, sum, n)| (num, sum, n + 1))
    }
}

fn reverse(num: Wrapping<u64>) -> Wrapping<u64> {
    fn calc_capicua(num: Wrapping<u64>, rev: Wrapping<u64>) -> Wrapping<u64> {
        let zero = Wrapping(0u64);
        if num == zero {
            rev
        } else {
            let ten = Wrapping(10u64);
            calc_capicua(num / ten, ten * rev + num % ten)
        }
    }
    calc_capicua(num, Wrapping(0u64))
}
