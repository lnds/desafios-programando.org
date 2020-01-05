use cartas::*;
use colored::*;
use std::io;
use std::result::Result;

enum Accion {
    Apostar(i32),
    Salir,
}

#[derive(Debug)]
enum Error {
    AccionInvalida,
    Bancarrota,
    MontoInvalido,
    MontoExcedePozo(i32),
}

const INICIAL: i32 = 500; // monto inicial de apuesta

fn salir(mensaje: &str) -> Result<(), Error> {
    println!("{}", mensaje.yellow().bold());
    Ok(())
}

fn main() -> Result<(), Error> {
    let mut pozo = INICIAL;
    println!("\n{}\n", "Bienvenido al juego del Poker".bold());
    loop {
        match ingresar_apuesta(pozo) {
            Ok(accion) => match accion {
                Accion::Salir => return salir("gracias por jugar"),
                Accion::Apostar(apuesta) => pozo += jugar(apuesta),
            },
            Err(error) => match error {
                Error::AccionInvalida => println!("no pudo interpretar tu apuesta"),
                Error::Bancarrota => salir("Lo siento, pero estás en bancarrota")?,
                Error::MontoInvalido => println!("no puedo procesar tu acción"),
                Error::MontoExcedePozo(apuesta) => println!(
                    "Tu apuesta (${}) no debe exceder tu pozo de dinero (${})",
                    apuesta, pozo
                ),
            },
        }
    }
}

fn jugar(apuesta: i32) -> i32 {
    println!("tu apuesta es: {}\n", apuesta);
    let mut naipes = Baraja::new();
    naipes.barajar();

    let mut mano_jugador = naipes.repartir(5);
    let mano_croupier = naipes.repartir(5);
    mostrar_mano("tu mano:", &mano_jugador);

    cambiar_cartas(&mut mano_jugador, &mut naipes);

    mostrar_mano("tu mano:", &mano_jugador);
    mostrar_mano("mi mano:", &mano_croupier);
    let ganancia = determinar_ganador(&mano_jugador, &mano_croupier, apuesta);
    if ganancia > 0 {
        println!("ganaste $ {}", ganancia);
    } else {
        println!("perdiste $ {}", ganancia.abs());
    }
    ganancia
}

fn cambiar_cartas(mano: &mut Mano, naipes: &mut Baraja) {
    for _ in 0..5 {
        let carta = pedir_numero(
            r"Ingresa la carta que quieres cambiar (0 para terminar):",
            0,
            5,
        );
        if carta == 0 {
            return;
        }
        if let Some(naipe) = naipes.tomar() {
            mano.cambiar((carta - 1) as usize, naipe);
        } else {
            println!("No hay cartas disponibles");
            return;
        }
    }
}

fn ingresar_apuesta(pozo: i32) -> Result<Accion, Error> {
    if pozo <= 0 {
        println!("Ya no tienes dinero");
        return Err(Error::Bancarrota);
    }
    println!("Tu pozo es: {}", pozo);
    println!(
        "{}:",
        "Ingresa el monto de tu apuesta (o escribe salir)"
            .yellow()
            .bold()
    );
    let mut accion = String::new();
    match io::stdin().read_line(&mut accion) {
        Ok(_) => match accion.to_lowercase().trim() {
            "salir" => Ok(Accion::Salir),
            val if val.chars().all(|c| c.is_digit(10)) => {
                let monto = val.parse::<i32>().or(Err(Error::MontoInvalido))?;
                if monto <= pozo {
                    Ok(Accion::Apostar(monto))
                } else {
                    Err(Error::MontoExcedePozo(monto))
                }
            }
            _ => Err(Error::AccionInvalida),
        },
        _ => Err(Error::AccionInvalida),
    }
}

fn determinar_ganador(mano_jugador: &Mano, mano_croupier: &Mano, monto_apuesta: i32) -> i32 {
    match mano_ganadora(&mano_jugador, &mano_croupier) {
        Some(mano) => {
            if *mano == *mano_jugador {
                println!("{}", "Tú ganas".yellow().bold());
                monto_apuesta
            } else {
                println!("{}", "Perdiste".red().bold());
                -monto_apuesta
            }
        }
        None => {
            println!("No hay ganador");
            0
        }
    }
}

fn pedir_numero(mensaje: &str, min: u8, max: u8) -> u8 {
    println!("{}", mensaje.bold().yellow());
    let mut num = String::new();
    loop {
        match io::stdin().read_line(&mut num) {
            Ok(_) => {
                let n = num.trim().parse::<u8>().unwrap_or(0);
                if n >= min && n <= max {
                    return n;
                }
                println!("debe ingresar un valor entre {} y {}", min, max);
            }
            _ => println!("debe ingresar un valor entre {} y {}", min, max),
        }
    }
}

fn mostrar_mano(mensaje: &str, mano: &Mano) {
    println!("{} {}\n", mensaje.blue().bold(), mano);
}
