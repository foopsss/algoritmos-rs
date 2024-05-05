use functions::read_input;

fn main() {
    let x = read_input::<i16>("Introduzca un valor para X");
    let y = read_input::<i16>("Introduzca un valor para Y");
    let z = read_input::<i16>("Introduzca un valor para Z");

    let med_geo = (x * y * z) / 3;

    println!();
    println!("Valor de X: {x}");
    println!("Valor de Y: {y}");
    println!("Valor de Z: {z}");
    println!("Valor de la media geométrica: {med_geo}");
}
