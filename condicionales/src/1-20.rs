use functions::read_input;

fn main() {
    let mut numero = read_input::<i16>("Ingrese un número");

    let mut cuadrado;
    let mut cubo;

    println!();
    for i in 1..=10 {
        cuadrado = numero.pow(2);
        cubo = numero.pow(3);

        println!("Número: {numero}");
        println!("Cuadrado: {cuadrado}");
        println!("Cubo: {cubo}");

        if i < 10 {
            println!();
        }

        numero = numero + 1;
    }
}
