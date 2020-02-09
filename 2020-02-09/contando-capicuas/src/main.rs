type Int = u64;

fn main() {
    if let Some((num, c, n)) = (100..=1000)
        .flat_map(|i| encontrar_capicua(i, 1))
        .max_by(|(_, _, a), (_, _, b)| a.cmp(b))
    {
        println!("num = {}, su capicua es {}, iteraciones: {}", num, c, n);
    }
}

const LIMIT: Int = 1_000_000;

fn encontrar_capicua(num: Int, n: Int) -> Option<(Int, Int, Int)> {
    let rev = reverse(num)?;
    let sum = num + rev;
    if n > LIMIT {
        None
    } else if sum == reverse(sum)? {
        Some((num, sum, n))
    } else {
        encontrar_capicua(sum, n + 1).map(|(_, sum, n)| (num, sum, n    ))
    }
}

fn reverse(num: Int) -> Option<Int> {
    fn calc_capicua(num: Int, rev: Int) -> Option<Int> {
        if num == 0 {
            Some(rev)
        } else {
            calc_capicua(num.checked_div(10)?, (rev.checked_mul(10)?).checked_add(num.checked_rem(10)?)?)
        }
    }
    calc_capicua(num, 0)
}
