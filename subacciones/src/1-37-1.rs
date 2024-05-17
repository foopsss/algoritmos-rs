use functions::read_input;

fn devolver_cuadrado(num: i16) -> i16 {
    num.pow(2)
}

fn main() {
    let numero = read_input::<i16>("Introduzca un número");
    let cuadrado = devolver_cuadrado(numero);

    println!();
    println!("El cuadrado de {numero} es {cuadrado}.");
}
