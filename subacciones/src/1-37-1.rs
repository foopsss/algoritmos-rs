use functions::read_input;

fn devolver_cuadrado(num: i64) -> i64 {
    num.pow(2)
}

fn main() {
    let numero = read_input::<i64>("Introduzca un número");

    let cuadrado = devolver_cuadrado(numero);

    println!("");
    println!("El cuadrado de {numero} es {cuadrado}.");
}
