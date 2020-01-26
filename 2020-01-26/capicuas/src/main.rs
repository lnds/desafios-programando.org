fn main() {
    (12..99)
        .flat_map(|i| encontrar_capicua(i))
        .for_each(|(n, c)| {
            println!("capicua de {} es {}", n, c);
        });
}

fn encontrar_capicua(num: u64) -> Option<(u64, u64)> {
    let rev = reverse(num);
    let sum = num + rev;
    if sum == reverse(sum) {
        Some((num, sum))
    } else {
        encontrar_capicua(sum).map(|(_, sum)| (num, sum))
    }
}

fn reverse(num: u64) -> u64 {
    fn calc_capicua(num: u64, rev: u64) -> u64 {
        match num {
            0 => rev,
            _ => calc_capicua(num / 10, 10 * rev + num % 10),
        }
    }
    calc_capicua(num, 0)
}
