use functions::read_input;
use std::cmp::Ordering;

fn main() {
    let a = read_input::<i16>("Ingrese un valor para el número a");
    let b = read_input::<i16>("Ingrese un valor para el número b");
    let c = read_input::<i16>("Ingrese un valor para el número c");

    let mut mayor = 0;
    let mut menor = 0;
    let mut intermedio = 0;

    if a > b && a > c {
        mayor = a;

        match b.cmp(&c) {
            Ordering::Less => {
                menor = b;
                intermedio = c;
            }
            Ordering::Equal => {
                intermedio = b;
                menor = intermedio;
            }
            Ordering::Greater => {
                menor = c;
                intermedio = b;
            }
        }
    }

    if b > a && b > c {
        mayor = b;

        match a.cmp(&c) {
            Ordering::Less => {
                menor = a;
                intermedio = c;
            }
            Ordering::Equal => {
                intermedio = a;
                menor = intermedio;
            }
            Ordering::Greater => {
                menor = c;
                intermedio = a;
            }
        }
    }

    if c > a && c > b {
        mayor = c;

        match a.cmp(&b) {
            Ordering::Less => {
                menor = a;
                intermedio = b;
            }
            Ordering::Equal => {
                intermedio = a;
                menor = intermedio;
            }
            Ordering::Greater => {
                menor = b;
                intermedio = a;
            }
        }
    }

    println!();
    if a == b && b == c {
        println!("¡Los tres números coinciden!");
    } else {
        println!("El número más grande es {mayor}.");
        println!("El número más pequeño es {menor}.");

        if intermedio != menor {
            println!("Entre ellos se encuentra el número {intermedio}.");
        }
    }
}
