use functions::read_input;

fn main() {
    let corriente = read_input::<f64>("Introduzca el valor de la corriente eléctrica del conductor");
    let conductividad = read_input::<f64>("Introduzca el valor de la conductividad eléctrica del conductor");

    let seccion = (corriente / conductividad) + (corriente / conductividad) * 0.25;

    println!("");
    println!("La sección del conductor es: {seccion}");
}
