use std::cmp::Ordering;

use functions::read_input;
use functions::draw_line;

fn es_negativo(mut entrada: i64) -> i8 {
    match entrada.cmp(&0) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => {
            let mut suma_digitos = 0;
            let mut divisor;
            let mut digito;

            let long_entrada = read_input::<u32>("Introduzca la longitud del número");

            for i in 1..=long_entrada {
                divisor = i64::pow(10, long_entrada - i);
                digito = entrada / divisor;
                suma_digitos += digito;
                entrada -= digito * divisor;
            }

            (suma_digitos % 7).try_into().expect("¡Error al convertir!")
        }
    }
}

fn main() {
    let mut eleccion = read_input::<char>("¿Desea comprobar si un número es negativo? [S/N]");
    println!();

    while eleccion == 'S' {
        let numero = read_input::<i64>("Introduzca un número");
        let clave = es_negativo(numero);

        println!();
        match clave.cmp(&0) {
            Ordering::Less => println!("El número es negativo."),
            Ordering::Equal => println!("El número es 0."),
            Ordering::Greater => {
                println!("El número es positivo.");
                println!("Clave: {clave}");
            }
        }

        println!();
        eleccion = read_input::<char>("¿Desea probar con otro número? [S/N]");

        if eleccion == 'S' {
            draw_line(55);
        }
    }
}
