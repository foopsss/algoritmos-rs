use functions::read_input;
use functions::draw_line;

fn main() {
    const METRO_A_PIE: f64 = 3.28;

    for i in 1..=50 {
        println!("Número de edificio: {i}");
        println!("");

        let pisos = read_input::<f64>("Introduzca el número de pisos del edificio");
        let alt_prom = read_input::<f64>("Introduzca la altura promedio de cada piso (en metros)");

        let alt_edificio = (pisos * alt_prom) * METRO_A_PIE;

        println!("");
        println!("La altura del edificio es de {alt_edificio} pies.");

        if i < 50 {
           draw_line(60);
        }
    }
}
