use functions::read_double;

fn main() {
    let corriente: f64 = read_double("Introduzca el valor de la corriente eléctrica del conductor");
    let conductividad: f64 = read_double("Introduzca el valor de la conductividad eléctrica del conductor");

    let sección: f64 = (corriente / conductividad) + (corriente / conductividad) * 0.25;

    println!("");
    println!("La sección del conductor es: {sección}");
}
