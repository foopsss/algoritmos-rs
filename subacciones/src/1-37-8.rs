use std::mem::swap;

use functions::read_input;

fn intercambiar(mut a: i16, mut b: i16) -> (i16, i16) {
    swap(&mut a, &mut b);

    (a, b)
}

fn main() {
    let num_a = read_input::<i16>("Introduzca un valor para A");
    let num_b = read_input::<i16>("Introduzca un valor para B");

    println!();
    println!("Primer valor de A: {num_a}");
    println!("Primer valor de B: {num_b}");

    // "intercambiar" devuelve una tupla, así que
    // le asigno individualmente a cada variable
    // los valores retornados por la función.
    let (num_a, num_b) = intercambiar(num_a, num_b);

    println!();
    println!("Nuevo valor de A: {num_a}");
    println!("Nuevo valor de B: {num_b}");
}
