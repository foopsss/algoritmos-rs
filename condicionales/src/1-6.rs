use std::cmp::Ordering;
use functions::read_input;

fn main() {
    let a = read_input::<i16>("Ingrese un valor para el número a");
    let b = read_input::<i16>("Ingrese un valor para el número b");
    let c = read_input::<i16>("Ingrese un valor para el número c");

    let mut mayor = a;
    let mut menor = 0;
    let mut intermedio = 0;

    if b > mayor && c > mayor {
        menor = mayor;

        match b.cmp(&c) {
            Ordering::Less => {
                intermedio = b;
                mayor = c;
            }
            Ordering::Equal => {
                intermedio = b;
                mayor = intermedio;
            }
            Ordering::Greater => {
                intermedio = c;
                mayor = b;
            }
        }
    }

    if b > mayor && c < mayor {
        menor = c;
        intermedio = mayor;
        mayor = b;
    }

    if b < mayor && c > mayor {
        menor = b;
        intermedio = mayor;
        mayor = c;
    }

    if b < mayor && c < mayor {
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

    println!();
    println!("El número más grande es {mayor}.");
    println!("El número más pequeño es {menor}.");
    println!("Entre ellos se encuentra el número {intermedio}.");
}
