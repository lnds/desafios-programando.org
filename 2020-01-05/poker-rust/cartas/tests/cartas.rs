use cartas::*;

#[test]
fn test_reparto() {
    let mut naipes = Baraja::new();
    assert_eq!(naipes.contar(), 52);
    let mano = naipes.repartir(5);
    assert_eq!(mano.contar(), 5);
    assert_eq!(naipes.contar(), 47);
    println!("{:?}", mano);
    assert_eq!(mano.valor(), "AP 2P 3P 4P 5P");
}
