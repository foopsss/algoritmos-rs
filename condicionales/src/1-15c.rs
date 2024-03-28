use functions::read_input;
use functions::draw_line;

fn main() {
    let mut eleccion = 'S';

    while eleccion == 'S' {
        let pisos = read_input::<f64>("Introduzca el número de pisos del edificio");
        let alt_prom = read_input::<f64>("Introduzca la altura promedio de cada piso (en metros)");

        let alt_edificio = (pisos * alt_prom) * 3.28;

        println!("");
        println!("La altura del edificio es de {alt_edificio} pies.");

        println!("");
        eleccion = read_input::<char>("¿Desea realizar otro cálculo? [S/N]");

        if eleccion == 'S' {
            draw_line(60);
        }
    }
}
