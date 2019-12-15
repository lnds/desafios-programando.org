use sha2::{Digest, Sha512};
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    // t0 nos sirve para medir tiempo
    let t0 = std::time::Instant::now();

    // toma de los argumentos el nombre del archivo de hashes
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // crea un hashset con los hahes que lee del archivo
    let mut hashes = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();

    // loop de búsqueda
    for i in 0..10000 {
        let candidate = format!("{:04.4}", i);
        let hash = format!("{:x}", Sha512::new().chain(&candidate).result());
        if hashes.contains(&hash) {
            println!("{}", candidate);
            hashes.remove(&hash);
        }
        if hashes.is_empty() { break }
    }

    // muestra el tiempo de ejecución
    let dur = t0.elapsed();
    let secs = dur.as_secs();
    let msec = dur.subsec_millis();
    println!("tiempo ocupado: {} segundos {} milisegundos", secs, msec);
}
