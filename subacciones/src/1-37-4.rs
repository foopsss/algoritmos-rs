use functions::read_input;
use functions::draw_line;

fn es_negativo(mut entrada: i64) -> i64 {
    if entrada > 0 {
        let mut divisor = 1;
        let mut digito;
        let mut suma_digitos = 0;

        let longitud = read_input::<u64>("Especifique la longitud del número");

        for _i in 1..longitud {
            divisor = divisor * 10;
        }

        for _i in 1..=longitud {
            digito = entrada / divisor;
            suma_digitos = suma_digitos + digito;
			entrada = entrada - (digito * divisor);
			divisor = divisor / 10;
        }

        suma_digitos % 7
    } else if entrada == 0 {
        0
    } else {
        -1
    }
}

fn main() {
    let mut eleccion = 'S';

    while eleccion == 'S' {
        let numero = read_input::<i64>("Introduzca un número");
        let clave = es_negativo(numero);

        println!("");
        println!("La clave es {clave}.");

        println!("");
        eleccion = read_input::<char>("¿Desea probar con otro número? [S/N]");

        if eleccion == 'S' {
            draw_line(40);
        }
    }
}
