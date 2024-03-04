use functions::read_integer;

fn main() {
    let a = read_integer("Ingrese un valor para A");
    let b = read_integer("Ingrese un valor para B");

    println!("");
    if a % b == 0 {
        println!("B es divisor de A.");
    } else if b % a == 0 {
        println!("A es divisor de B.");
    }
}
