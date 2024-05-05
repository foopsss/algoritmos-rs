use functions::read_input;

fn main() {
    let mut numero = read_input::<i16>("Ingrese un número");
    let incremento = read_input::<i16>("Ingrese la cantidad de veces a incrementarlo");

    let mut cuadrado;
    let mut cubo;
    let mut suma_cuadrados = 0;

    println!();
    for _i in 1..=incremento {
        cuadrado = numero.pow(2);
        cubo = numero.pow(3);

        println!("Número: {numero}");
        println!("Cuadrado: {cuadrado}");
        println!("Cubo: {cubo}");
        println!();

        suma_cuadrados = suma_cuadrados + cuadrado;
        numero = numero + 1;
    }

    println!("El resultado de la suma de cuadrados es {suma_cuadrados}.");
}
