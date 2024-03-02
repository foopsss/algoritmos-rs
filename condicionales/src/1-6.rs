use functions::read_integer;

fn main() {
    let a = read_integer("Ingrese un valor para el número a");
    let b = read_integer("Ingrese un valor para el número b");
    let c = read_integer("Ingrese un valor para el número c");

    let mut mayor = a;
    let mut menor = 0;
    let mut intermedio = 0;

    if b > mayor && c > mayor {
        menor = mayor;

        if b > c {
            intermedio = c;
            mayor = b;
        } else if b == c {
            intermedio = b;
            mayor = intermedio;
        } else if b < c {
            intermedio = b;
            mayor = c;
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
        if b > c {
            menor = c;
            intermedio = b;
        } else if b == c {
            intermedio = b;
            menor = intermedio;
        } else if b < c {
            menor = b;
            intermedio = c;
        }
    }

    println!("");
    println!("El número más grande es {mayor}.");
    println!("El número más pequeño es {menor}.");
    println!("Entre ellos se encuentra el número {intermedio}.");
}
