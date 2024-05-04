use functions::read_input;
use functions::draw_line;

fn devolver_menor(num_a: i16, num_b: i16, num_c: i16) -> i16 {
    if num_a < num_b && num_a < num_c {
        num_a
    } else if num_b < num_a && num_b < num_c {
        num_b
    } else {
        num_c
    }
}

fn main() {
    let mut eleccion = read_input::<char>("¿Desea hallar el menor de tres valores? [S/N]");
    println!("");

    while eleccion == 'S' {
        let a = read_input::<i16>("Ingrese un valor numérico para A");
        let b = read_input::<i16>("Ingrese un valor numérico para B");
        let c = read_input::<i16>("Ingrese un valor numérico para C");

        let menor = devolver_menor(a, b, c);

        println!("");
        println!("De los números introducidos, el menor es {menor}.");

        println!("");
        eleccion = read_input::<char>("¿Desea realizar otra comparación? [S/N]");

        if eleccion == 'S' {
            draw_line(50);
        }
    }
}
