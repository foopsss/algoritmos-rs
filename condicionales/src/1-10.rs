use functions::read_input;

fn main() {
    let a = read_input::<i16>("Ingrese un valor para A");
    let b = read_input::<i16>("Ingrese un valor para B");

    println!();
    if a % b == 0 {
        println!("B es divisor de A.");
    } else if b % a == 0 {
        println!("A es divisor de B.");
    } else {
        println!("¡No es posible dividir entre estos números!");
    }
}
