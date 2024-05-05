use functions::read_input;

fn main() {
    const METRO_A_PIE: f64 = 3.28;

    let pisos = read_input::<f64>("Introduzca el número de pisos del edificio");
    let alt_prom = read_input::<f64>("Introduzca la altura promedio de cada piso (en metros)");

    let alt_edificio = (pisos * alt_prom) * METRO_A_PIE;

    println!();
    println!("La altura del edificio es de {alt_edificio} pies.");
}
