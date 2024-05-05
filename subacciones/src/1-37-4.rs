use functions::read_input;
use functions::draw_line;

fn es_negativo(mut entrada: i64, long_entrada: u32) -> i8 {
    if entrada > 0 {
        let mut suma_digitos = 0;
        let mut divisor;
        let mut digito;

        for i in 1..=long_entrada {
            divisor = i64::pow(10, long_entrada - i);
            digito = entrada / divisor;
            suma_digitos += digito;
            entrada -= digito * divisor;
        }

        (suma_digitos % 7).try_into().expect("¡Error al convertir!")
    } else if entrada == 0 {
        0
    } else {
        -1
    }
}

fn main() {
    let mut eleccion = read_input::<char>("¿Desea comprobar si un número es negativo? [S/N]");
    println!();

    while eleccion == 'S' {
        let numero = read_input::<i64>("Introduzca un número");
        let longitud = read_input::<u32>("Introduzca la longitud del número");

        let clave = es_negativo(numero, longitud);

        println!();
        if clave >= 0 {
            println!("El número es positivo.");
            println!("Clave: {clave}");
        } else {
            println!("El número es negativo.")
        }

        println!();
        eleccion = read_input::<char>("¿Desea probar con otro número? [S/N]");

        if eleccion == 'S' {
            draw_line(40);
        }
    }
}
