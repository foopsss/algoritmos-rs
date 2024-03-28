use functions::read_input;

fn main() {
    let pisos = read_input::<f64>("Introduzca el número de pisos del edificio");
    let alt_prom = read_input::<f64>("Introduzca la altura promedio de cada piso (en metros)");

    let alt_edificio = (pisos * alt_prom) * 3.28;

    println!("");
    println!("La altura del edificio es de {alt_edificio} pies.");
}
