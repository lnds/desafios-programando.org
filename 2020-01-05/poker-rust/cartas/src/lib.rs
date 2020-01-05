#[macro_use]
extern crate itertools;
use colored::*;
use enum_iterator::IntoEnumIterator;
use itertools::Itertools;
use rand;
use rand::seq::SliceRandom;
use std::convert::From;
use std::fmt;

#[derive(Clone, Debug, IntoEnumIterator, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Pinta {
    Picas,
    Corazones,
    Diamantes,
    Treboles,
}

impl fmt::Display for Pinta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let icon = match self {
            &Pinta::Picas => "♠",
            &Pinta::Corazones => "♥",
            &Pinta::Diamantes => "♦",
            &Pinta::Treboles => "♣",
        };
        write!(f, "{:<1}", icon)
    }
}

impl Pinta {
    fn valor(&self) -> char {
        match &self {
            &Pinta::Picas => 'P',
            &Pinta::Corazones => 'C',
            &Pinta::Diamantes => 'D',
            &Pinta::Treboles => 'T',
        }
    }
}

impl From<&str> for Pinta {
    fn from(valor: &str) -> Self {
        match valor {
            "C" => Pinta::Corazones,
            "D" => Pinta::Diamantes,
            "T" => Pinta::Treboles,
            "P" => Pinta::Picas,
            _ => panic!("no puede convertir orden desde {}", valor),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Color {
    Rojo,
    Negro,
}

#[derive(Clone, Debug, IntoEnumIterator, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Orden {
    As,
    Dos,
    Tres,
    Cuatro,
    Cinco,
    Seis,
    Siete,
    Ocho,
    Nueve,
    Diez,
    Jack,
    Queen,
    King,
}

impl Orden {
    fn valor(&self) -> &str {
        match self {
            Orden::As => "A",
            Orden::King => "K",
            Orden::Queen => "Q",
            Orden::Jack => "J",
            Orden::Diez => "10",
            Orden::Nueve => "9",
            Orden::Ocho => "8",
            Orden::Siete => "7",
            Orden::Seis => "6",
            Orden::Cinco => "5",
            Orden::Cuatro => "4",
            Orden::Tres => "3",
            Orden::Dos => "2",
        }
    }
}

impl From<&str> for Orden {
    fn from(valor: &str) -> Self {
        match valor {
            "A" => Orden::As,
            "K" => Orden::King,
            "Q" => Orden::Queen,
            "J" => Orden::Jack,
            "10" => Orden::Diez,
            "9" => Orden::Nueve,
            "8" => Orden::Ocho,
            "7" => Orden::Siete,
            "6" => Orden::Seis,
            "5" => Orden::Cinco,
            "4" => Orden::Cuatro,
            "3" => Orden::Tres,
            "2" => Orden::Dos,
            _ => panic!("no puede convertir orden desde {}", valor),
        }
    }
}

impl fmt::Display for Orden {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match *self {
            Orden::As => "A",
            Orden::Dos => "2",
            Orden::Tres => "3",
            Orden::Cuatro => "4",
            Orden::Cinco => "5",
            Orden::Seis => "6",
            Orden::Siete => "7",
            Orden::Ocho => "8",
            Orden::Nueve => "9",
            Orden::Diez => "10",
            Orden::Jack => "J",
            Orden::Queen => "Q",
            Orden::King => "K",
        };
        write!(f, "{}", display)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct Naipe {
    pub orden: Orden,
    pub pinta: Pinta,
    pub color: Color,
}

impl Naipe {
    pub fn new(orden: Orden, pinta: Pinta) -> Naipe {
        let color = match pinta {
            Pinta::Diamantes | Pinta::Corazones => Color::Rojo,
            _ => Color::Negro,
        };
        Naipe {
            orden,
            pinta,
            color,
        }
    }

    pub fn valor(&self) -> String {
        format!("{}{}", self.orden.valor(), self.pinta.valor())
    }
}

impl From<&str> for Naipe {
    fn from(carta: &str) -> Self {
        let valor: &str = &carta[0..carta.len() - 1];
        let pinta: &str = &carta[carta.len() - 1..];
        Naipe::new(Orden::from(valor), Pinta::from(pinta))
    }
}

impl fmt::Display for Naipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let naipe = format!("{}{:>2.2}", self.pinta, self.orden);
        let naipe_decorado = if self.color == Color::Rojo {
            naipe.red()
        } else {
            naipe.black()
        };
        write!(f, "{}", naipe_decorado.bold().on_white())
    }
}

pub struct Baraja(Vec<Naipe>);

impl Baraja {
    pub fn new() -> Self {
        let mut naipes = iproduct!(Pinta::into_enum_iter(), Orden::into_enum_iter())
            .map(|(pinta, orden)| Naipe::new(orden.clone(), pinta.clone()))
            .into_iter()
            .collect::<Vec<Naipe>>();
        naipes.reverse();
        Baraja(naipes)
    }

    pub fn tomar(&mut self) -> Option<Naipe> {
        self.0.pop()
    }

    pub fn barajar(&mut self) {
        let mut rng = rand::thread_rng();
        self.0.shuffle(&mut rng);
    }

    pub fn repartir(&mut self, n: usize) -> Mano {
        Mano::new((0..n).flat_map(|_| self.tomar()).collect())
    }

    pub fn contar(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for Baraja {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for carta in self.0.iter() {
            let _ = writeln!(f, "{}", carta);
        }
        write!(f, "")
    }
}

#[derive(Debug, PartialEq)]
pub struct Mano(Vec<Naipe>);

impl Mano {
    fn new(naipes: Vec<Naipe>) -> Self {
        Mano(naipes.iter().sorted().cloned().collect())
    }

    pub fn valor(&self) -> String {
        self.0
            .iter()
            .map(|n| n.valor())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn contar(&self) -> usize {
        self.0.len()
    }

    pub fn cambiar(&mut self, pos: usize, naipe: Naipe) {
        self.0[pos] = naipe;
    }
}

impl fmt::Display for Mano {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, carta) in self.0.iter().enumerate() {
            let _ = write!(f, "{}:{} ", i + 1, carta);
        }
        write!(f, " ")
    }
}

impl From<&str> for Mano {
    fn from(valor: &str) -> Self {
        let naipes: Vec<Naipe> = valor.split_whitespace().map(|s| Naipe::from(s)).collect();
        Mano::new(naipes)
    }
}

pub fn mano_ganadora<'a>(mano1: &'a Mano, mano2: &'a Mano) -> Option<&'a Mano> {
    let m1 = &mano1.valor();
    let m2 = &mano2.valor();
    let resultado = poker::manos_ganadoras(&[m1, m2])?;
    if resultado.len() == 2 {
        return None;
    }
    if resultado[0] == mano1.valor() {
        return Some(&mano1);
    }
    return Some(&mano2);
}
