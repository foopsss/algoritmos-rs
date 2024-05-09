use std::cmp::Ordering;
use functions::read_input;

fn main() {
    let anio_act = read_input::<i16>("Ingrese el año actual");
    let anio_usuario = read_input::<i16>("Ingrese un año a elección");

    println!();
    match anio_usuario.cmp(&anio_act) {
        Ordering::Less => println!("PASADO."),
        Ordering::Equal => println!("ACTUAL."),
        Ordering::Greater => println!("FUTURO."),
    }
}
