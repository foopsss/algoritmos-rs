use functions::read_input;

fn main() {
    let anio_act = read_input::<i16>("Ingrese el año actual");
    let anio_usuario = read_input::<i16>("Ingrese un año a elección");

    println!();
    if anio_usuario > anio_act {
        println!("FUTURO.");
    } else if anio_usuario == anio_act {
        println!("ACTUAL.");
    } else {
        println!("PASADO.");
    }
}
