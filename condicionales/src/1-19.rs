use functions::read_input;

fn main() {
    let v = read_input::<i16>("Ingrese un número entero");

    let cuadrado = v.pow(2);
    let cubo = v.pow(3);

    println!("");
    println!("El número es {v}.");
    println!("El cuadrado de dicho número es {cuadrado}.");
    println!("El cubo de dicho número es {cubo}.");
}
