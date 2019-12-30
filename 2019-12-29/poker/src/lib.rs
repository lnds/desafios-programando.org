use itertools::Itertools;

const PINTAS: [char; 4] = ['P', 'C', 'T', 'D'];

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct Carta(u8, char);

impl Carta {
    pub fn new(carta: &str) -> Option<Carta> {
        let valor = match &carta[0..carta.len() - 1] {
            "A" => 14,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            s => s.parse().ok().filter(|&x| x >= 2 && x <= 10)?,
        };
        let pinta = carta.chars().last().filter(|c| PINTAS.contains(c))?;
        Some(Carta(valor, pinta))
    }
}

#[test]
fn cartas() {
    assert!(Carta::new("11C").is_none());
    assert!(Carta::new("AP").is_some());
    let carta = Carta::new("KT");
    assert_eq!(carta.unwrap().0, 13);

    let cartas: Vec<Carta> = "AT 2D 10T 3T KC"
        .split_whitespace()
        .flat_map(|s| Carta::new(s))
        .collect();
    assert_eq!(cartas.len(), 5);

    let cartas: Vec<Carta> = "10D 9Z 10T QT JD"
        .split_ascii_whitespace()
        .flat_map(|s| Carta::new(s))
        .collect();
    assert_eq!(cartas.len(), 4);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Mano {
    Invalida,
    CartaAlta(u8, u8, u8, u8, u8),
    Par(u8, u8, u8, u8),
    DoblePar(u8, u8, u8),
    Trio(u8, u8, u8),
    Escala(u8),
    Color(u8),
    FullHouse(u8, u8),
    Poker(u8, u8),
    EscalaDeColor(u8),
}

impl Mano {
    fn new(cartas: &[Carta]) -> Self {
        if cartas.len() != 5 {
            return Mano::Invalida;
        }

        let grupos = cartas
            .iter()
            .sorted_by_key(|c| c.0)
            .group_by(|&c| c.0)
            .into_iter()
            .map(|(_, g)| g.map(|c| c.0).collect())
            .sorted_by_key(|v: &Vec<u8>| v.len())
            .collect::<Vec<_>>();
        let clasi = grupos.iter().map(|v| v.len()).collect::<Vec<usize>>();
        match &clasi[..] {
            [_, 4] => Mano::Poker(grupos[1][0], grupos[0][0]),
            [2, 3] => Mano::FullHouse(grupos[1][0], grupos[0][0]),
            [_, _, 3] => Mano::Trio(grupos[2][0], grupos[1][0], grupos[0][0]),
            [_, 2, 2] => Mano::DoblePar(grupos[2][0], grupos[1][0], grupos[0][0]),
            [_, _, _, 2] => Mano::Par(grupos[3][0], grupos[0][0], grupos[0][0], grupos[0][0]),
            _ => {
                // no encontramos grupos, así que vemos si tenemos escalas o color
                let cartas: Vec<Carta> = cartas
                    .iter()
                    .sorted_by(|&x, &y| y.cmp(&x))
                    .cloned()
                    .collect();
                let escala = cartas
                    .windows(2)
                    .all(|w| w[0].0 == w[1].0 + 1 || w[0].0 == 14 && w[1].0 == 5);
                let color = cartas.iter().all(|c| c.1 == cartas[0].1);

                if !escala && !color {
                    Mano::CartaAlta(
                        cartas[0].0,
                        cartas[1].0,
                        cartas[2].0,
                        cartas[3].0,
                        cartas[4].0,
                    )
                } else {
                    // fix situation when we have an ace in cards
                    let max: u8 = cartas
                        .iter()
                        .map(|c| if c.0 == 14 { 1 } else { c.0 })
                        .max()
                        .unwrap();

                    if escala && color {
                        Mano::EscalaDeColor(max)
                    } else if escala {
                        Mano::Escala(max)
                    } else {
                        Mano::Color(max)
                    }
                }
            }
        }
    }
}

pub fn manos_ganadoras<'a>(manos: &[&'a str]) -> Option<Vec<&'a str>> {
    let manos = manos
        .iter()
        .map(|mano| {
            let mano_poker = Mano::new(
                mano.split_whitespace()
                    .flat_map(|carta| Carta::new(carta))
                    .collect::<Vec<Carta>>().as_slice()
            );

            (mano_poker, *mano)
        })
        .sorted_by(|x, y| y.cmp(&x))
        .collect::<Vec<(Mano, &'a str)>>();

    if manos.iter().any(|(mano, _)| *mano == Mano::Invalida) {
        None
    } else {
        let winner = &manos[0].0;
        Some(
            manos
                .iter()
                .take_while(|mano| mano.0 == *winner) // empates
                .map(|m| m.1)
                .collect()
        )
    }
}
