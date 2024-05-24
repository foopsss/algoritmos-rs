use functions::{draw_line, read_input};

fn main() {
    let mut eleccion =
        read_input::<char>("¿Desea calcular la altura de un edificio en pies? [S/N]");
    println!();

    while eleccion == 'S' {
        const METRO_A_PIE: f64 = 3.28;

        let pisos = read_input::<f64>("Introduzca el número de pisos del edificio");
        let alt_prom = read_input::<f64>("Introduzca la altura promedio de cada piso (en metros)");

        let alt_edificio = (pisos * alt_prom) * METRO_A_PIE;

        println!();
        println!("La altura del edificio es de {alt_edificio} pies.");

        println!();
        eleccion = read_input::<char>("¿Desea realizar otro cálculo? [S/N]");

        if eleccion == 'S' {
            draw_line(60);
        }
    }
}
