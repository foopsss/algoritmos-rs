use functions::read_integer;

fn main() {
    let anio_act = read_integer("Ingrese el año actual");
    let anio_usuario = read_integer("Ingrese un año a elección");

    println!("");
    if anio_usuario > anio_act {
        println!("FUTURO.");
    } else if anio_usuario == anio_act {
        println!("ACTUAL.");
    } else {
        println!("PASADO.");
    }
}
